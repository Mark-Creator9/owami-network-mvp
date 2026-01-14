use crate::audit_log;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use governor::{DefaultKeyedRateLimiter, Quota, RateLimiter};
use lru_time_cache::LruCache;
use std::net::SocketAddr;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing::warn;

// Global rate limiter for IP-based limiting
type IpRateLimiter = Arc<Mutex<LruCache<String, u32>>>;

// Governor-based rate limiter for general requests
type ApiRateLimiter = Arc<DefaultKeyedRateLimiter<String>>;

#[derive(Clone)]
pub struct RateLimitingConfig {
    // IP-based rate limiting: max requests per IP per minute
    pub ip_requests_per_minute: u32,
    // API rate limiting: max requests per endpoint per second
    pub api_requests_per_second: u32,
    // Burst capacity for governor
    pub burst_capacity: u32,
}

impl Default for RateLimitingConfig {
    fn default() -> Self {
        Self {
            ip_requests_per_minute: 100, // 100 requests per minute per IP
            api_requests_per_second: 10, // 10 requests per second per endpoint
            burst_capacity: 5,           // Allow bursts of 5 requests
        }
    }
}

pub struct RateLimiterState {
    pub ip_limiter: IpRateLimiter,
    pub api_limiter: ApiRateLimiter,
    pub config: RateLimitingConfig,
}

impl RateLimiterState {
    pub fn new(config: RateLimitingConfig) -> Self {
        // Initialize IP-based rate limiter with LRU cache that expires after 1 minute
        let ip_limiter = Arc::new(Mutex::new(LruCache::with_expiry_duration(
            Duration::from_secs(60),
        )));

        // Initialize governor-based rate limiter for API endpoints
        let quota = Quota::per_second(NonZeroU32::new(config.api_requests_per_second).unwrap())
            .allow_burst(NonZeroU32::new(config.burst_capacity).unwrap());
        let api_limiter = Arc::new(RateLimiter::keyed(quota));

        Self {
            ip_limiter,
            api_limiter,
            config,
        }
    }
}

/// Middleware for IP-based rate limiting
pub async fn ip_rate_limiter_middleware(
    State(rate_limiter): State<Arc<RateLimiterState>>,
    addr: SocketAddr,
    request: Request,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    let ip = addr.ip().to_string();
    let mut ip_cache = rate_limiter.ip_limiter.lock().await;

    let count = ip_cache.entry(ip.clone()).or_insert(0);
    *count += 1;

    if *count > rate_limiter.config.ip_requests_per_minute {
        // Log the rate limit event
        let _ = audit_log::log_security_event(
            "Rate limit exceeded".to_string(),
            format!(
                "IP {} exceeded rate limit of {} requests per minute",
                ip, rate_limiter.config.ip_requests_per_minute
            ),
            "failure".to_string(),
            None,
        );

        warn!("Rate limit exceeded for IP: {}", ip);
        return Err(axum::http::StatusCode::TOO_MANY_REQUESTS);
    }

    drop(ip_cache);
    Ok(next.run(request).await)
}

/// Middleware for API endpoint rate limiting
pub async fn api_rate_limiter_middleware(
    State(rate_limiter): State<Arc<RateLimiterState>>,
    request: Request,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    let path = request.uri().path().to_string();

    // Check if the request is allowed based on the endpoint
    if rate_limiter.api_limiter.check_key(&path).is_err() {
        // Log the rate limit event
        let _ = audit_log::log_security_event(
            "API rate limit exceeded".to_string(),
            format!(
                "Endpoint {} exceeded rate limit of {} requests per second",
                path, rate_limiter.config.api_requests_per_second
            ),
            "failure".to_string(),
            None,
        );

        warn!("API rate limit exceeded for endpoint: {}", path);
        return Err(axum::http::StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(request).await)
}

/// Combined middleware that applies both IP and API rate limiting
pub async fn rate_limiter_middleware(
    State(rate_limiter): State<Arc<RateLimiterState>>,
    request: Request,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    // Extract client IP from request extensions (set by Axum/hyper)
    let addr = request
        .extensions()
        .get::<SocketAddr>()
        .cloned()
        .unwrap_or_else(|| {
            // Fallback to a default address if not available
            SocketAddr::from(([0, 0, 0, 0], 0))
        });
    let ip = addr.ip().to_string();
    let path = request.uri().path().to_string();

    // First check IP-based rate limiting
    let mut ip_cache = rate_limiter.ip_limiter.lock().await;
    let count = ip_cache.entry(ip.clone()).or_insert(0);
    *count += 1;

    if *count > rate_limiter.config.ip_requests_per_minute {
        // Log the rate limit event
        let _ = audit_log::log_security_event(
            "Rate limit exceeded".to_string(),
            format!(
                "IP {} exceeded rate limit of {} requests per minute",
                ip, rate_limiter.config.ip_requests_per_minute
            ),
            "failure".to_string(),
            None,
        );

        warn!("Rate limit exceeded for IP: {}", ip);
        return Err(axum::http::StatusCode::TOO_MANY_REQUESTS);
    }
    drop(ip_cache);

    // Then check API-based rate limiting
    if rate_limiter.api_limiter.check_key(&path).is_err() {
        // Log the rate limit event
        let _ = audit_log::log_security_event(
            "API rate limit exceeded".to_string(),
            format!(
                "Endpoint {} exceeded rate limit of {} requests per second",
                path, rate_limiter.config.api_requests_per_second
            ),
            "failure".to_string(),
            None,
        );

        warn!("API rate limit exceeded for endpoint: {}", path);
        return Err(axum::http::StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(request).await)
}

/// DDoS protection middleware that checks for suspicious patterns
pub async fn ddos_protection_middleware(
    request: Request,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    // Check for common DDoS patterns
    let user_agent = request
        .headers()
        .get("User-Agent")
        .and_then(|ua| ua.to_str().ok())
        .unwrap_or("unknown");

    // Simple check for suspicious User-Agents (can be expanded)
    if user_agent.contains("bot") || user_agent.contains("crawler") {
        let _ = audit_log::log_security_event(
            "DDoS protection triggered".to_string(),
            format!("Suspicious User-Agent detected: {}", user_agent),
            "warning".to_string(),
            None,
        );

        warn!("Suspicious User-Agent detected: {}", user_agent);
        return Err(axum::http::StatusCode::FORBIDDEN);
    }

    // Check for empty or malformed headers
    if let Some(host) = request.headers().get("Host") {
        if host.is_empty() {
            let _ = audit_log::log_security_event(
                "DDoS protection triggered".to_string(),
                "Empty Host header detected".to_string(),
                "warning".to_string(),
                None,
            );

            warn!("Empty Host header detected");
            return Err(axum::http::StatusCode::BAD_REQUEST);
        }
    }

    Ok(next.run(request).await)
}

/// Health check endpoint that should not be rate limited
pub async fn health_check() -> &'static str {
    "OK"
}
