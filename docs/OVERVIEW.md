# Owami Network — Project Overview

This document helps external users, testers, and contributors quickly understand the project and find what they need.

## Quick Links
- Getting Started: `docs/QUICKSTART.md`
- API Guide (with examples): `docs/API_GUIDE.md`
- Testnet & Environment Setup: `docs/TESTNET_GUIDE.md`
- Troubleshooting: `docs/TROUBLESHOOTING.md`
- Investor Brief: `docs/investor/INVESTOR_BRIEF.md` (if applicable)

## Repository Map
- `src/` — Rust source code (API, blockchain logic, DB)
- `migrations/` — SQLx migrations
- `config/` — Configuration files (e.g., `production.toml`)
- `landing/` — Minimal web UI and docs
- `scripts/` — Helper scripts for dev and ops
- `postman/` — Postman collection and environment
- `docs/` — Documentation portal (start here)

## Primary Entry Points
- Start server: `./start_server.bat` (Windows) or `cargo run --release`
- Health check: `GET /api/health`
- Landing page: `GET /landing`

## Contributing
- Open a PR with a clear description and linked issue.
- Run tests locally: `cargo test`. For API tests: `./test_api_endpoints.ps1`.
- Keep docs updated when changing public APIs or flows.