use std::{net::SocketAddr, sync::Arc};

use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::get,
    Router,
};
use tower::ServiceExt; // for `oneshot`

use owami_network::rate_limiting::{
    api_rate_limiter_middleware, ddos_protection_middleware, health_check, rate_limiter_middleware,
    RateLimiterState, RateLimitingConfig,
};

fn make_state(config: RateLimitingConfig) -> Arc<RateLimiterState> {
    Arc::new(RateLimiterState::new(config))
}

#[tokio::test]
async fn ip_rate_limiter_allows_under_limit_and_blocks_over_limit() {
    // Allow 2 requests per minute per IP
    let config = RateLimitingConfig {
        ip_requests_per_minute: 2,
        api_requests_per_second: 100,
        burst_capacity: 100,
    };
    let state = make_state(config);

    let app = Router::new()
        .route("/any", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            rate_limiter_middleware,
        ));

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();

    // First request should pass
    let req1 = Request::builder()
        .uri("/any")
        .extension(addr)
        .body(Body::empty())
        .unwrap();
    let res1 = app.clone().oneshot(req1).await.unwrap();
    assert_eq!(res1.status(), StatusCode::OK);

    // Second request should pass
    let req2 = Request::builder()
        .uri("/any")
        .extension(addr)
        .body(Body::empty())
        .unwrap();
    let res2 = app.clone().oneshot(req2).await.unwrap();
    assert_eq!(res2.status(), StatusCode::OK);

    // Third request should be blocked (429)
    let req3 = Request::builder()
        .uri("/any")
        .extension(addr)
        .body(Body::empty())
        .unwrap();
    let res3 = app.clone().oneshot(req3).await.unwrap();
    assert_eq!(res3.status(), StatusCode::TOO_MANY_REQUESTS);
}

#[tokio::test]
async fn api_rate_limiter_blocks_second_immediate_call_same_endpoint() {
    // Allow 1 request per second with burst 1
    let config = RateLimitingConfig {
        ip_requests_per_minute: 100,
        api_requests_per_second: 1,
        burst_capacity: 1,
    };
    let state = make_state(config);

    let app = Router::new()
        .route("/endpoint", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            api_rate_limiter_middleware,
        ));

    // First call allowed
    let req1 = Request::builder()
        .uri("/endpoint")
        .body(Body::empty())
        .unwrap();
    let res1 = app.clone().oneshot(req1).await.unwrap();
    assert_eq!(res1.status(), StatusCode::OK);

    // Immediate second call should be blocked for the same key (path)
    let req2 = Request::builder()
        .uri("/endpoint")
        .body(Body::empty())
        .unwrap();
    let res2 = app.clone().oneshot(req2).await.unwrap();
    assert_eq!(res2.status(), StatusCode::TOO_MANY_REQUESTS);
}

#[tokio::test]
async fn api_rate_limiter_allows_different_endpoints_independently() {
    // 1 req/sec with burst 1; different paths should have separate keys
    let config = RateLimitingConfig {
        ip_requests_per_minute: 100,
        api_requests_per_second: 1,
        burst_capacity: 1,
    };
    let state = make_state(config);

    let app = Router::new()
        .route("/a", get(|| async { "A" }))
        .route("/b", get(|| async { "B" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            api_rate_limiter_middleware,
        ));

    let req_a = Request::builder().uri("/a").body(Body::empty()).unwrap();
    let res_a = app.clone().oneshot(req_a).await.unwrap();
    assert_eq!(res_a.status(), StatusCode::OK);

    let req_b = Request::builder().uri("/b").body(Body::empty()).unwrap();
    let res_b = app.clone().oneshot(req_b).await.unwrap();
    assert_eq!(res_b.status(), StatusCode::OK);
}

#[tokio::test]
async fn combined_rate_limiter_prioritizes_ip_limit() {
    // IP limit 1, API generous so block comes from IP check
    let config = RateLimitingConfig {
        ip_requests_per_minute: 1,
        api_requests_per_second: 100,
        burst_capacity: 100,
    };
    let state = make_state(config);

    let app = Router::new()
        .route("/ok", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            rate_limiter_middleware,
        ));

    let addr: SocketAddr = "10.0.0.1:8080".parse().unwrap();

    // First call passes
    let req1 = Request::builder()
        .uri("/ok")
        .extension(addr)
        .body(Body::empty())
        .unwrap();
    let res1 = app.clone().oneshot(req1).await.unwrap();
    assert_eq!(res1.status(), StatusCode::OK);

    // Second call should be blocked due to IP limit
    let req2 = Request::builder()
        .uri("/ok")
        .extension(addr)
        .body(Body::empty())
        .unwrap();
    let res2 = app.clone().oneshot(req2).await.unwrap();
    assert_eq!(res2.status(), StatusCode::TOO_MANY_REQUESTS);
}

#[tokio::test]
async fn ddos_protection_blocks_suspicious_user_agent() {
    let app = Router::new()
        .route("/whatever", get(|| async { "OK" }))
        .layer(axum::middleware::from_fn(ddos_protection_middleware));

    let req = Request::builder()
        .uri("/whatever")
        .header("User-Agent", "evil-bot/1.0")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn ddos_protection_blocks_empty_host_header() {
    let app = Router::new()
        .route("/whatever", get(|| async { "OK" }))
        .layer(axum::middleware::from_fn(ddos_protection_middleware));

    let req = Request::builder()
        .uri("/whatever")
        .header("Host", "")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn health_check_returns_ok() {
    let ok = health_check().await;
    assert_eq!(ok, "OK");
}
