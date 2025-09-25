# Owami Network — Investor Brief (Draft)

This brief summarizes the MVP testnet plan, milestones, and investment readiness. Dates and amounts are placeholders pending your confirmation.

## Executive Summary
- Vision: Production-ready MVP blockchain with token, auth, and DB-backed state.
- Ask: $1,000,000 seed to fund 12 months of execution.
- Use of Funds: Engineering, security review, cloud, GTM, ops.
- Key Milestones: Public testnet launch, security review, API docs, multi-node testnet, early pilot integrations.

## Testnet MVP Plan
- Day-1 Scope: Auth, token (mint/transfer/balance), blockchain (mine/info/blocks), rate limiting, audit logs, landing UI.
- Rollout:
  - T0 (Launch): Single-node Render deployment, file+DB logging, health endpoints, Postman collection.
  - T+4w: OpenAPI docs, monitoring/alerts, backup/restore drill, faucet, demo mode.
  - T+8w: Multi-node testnet, metrics (Prometheus), load tests, latency targets.
  - T+12w: Security audit fixes, throughput benchmarks, SDK stubs.
- KPIs: 99.5% uptime, <300ms p50 API latency, 100+ active API users, 10k+ tx on testnet, 2 pilot projects.

## Technical Readiness
- Status: Server runs with production config (port 3002), migrations stable, tests 18/19 (transfer 400 by design when insufficient funds).
- Open Items: Decide transfer demo semantics; finalize rate limit configs; add OpenAPI.
- Render: `render.yaml` present; environment variables and DB connectivity required.

## Security & Compliance
- JWT auth, ED25519 signatures, rate limiting, audit logs.
- Planned: 3rd-party security review (T+8w ~ T+12w), incident response runbook, data retention policy.

## Tokenomics & Governance (Outline)
- Supply and allocations, vesting schedules, on-chain/off-chain governance model.
- Testnet policy: Faucet + rate-limited mint for demos.

## Go-To-Market & Traction
- Early users: target pilots in fintech/supply chain.
- Dev relations: docs portal, Postman, tutorials, community channels.

## 12-Month Plan (Tranche-Based)
- Tranche 1 (Launch → T+8w): Public testnet, OpenAPI, monitoring, faucet.
- Tranche 2 (T+8w → T+20w): Multi-node, metrics, performance benchmarks.
- Tranche 3 (T+20w → T+52w): Security audit remediation, pilot integrations, enterprise features.

## Investor Protections
- Monthly reporting, board observer, information rights, milestone-based unlocks, cash controls.

## Risks & Mitigations
- Technical (DB/consistency), regulatory, market adoption, operational. Mitigate via staged rollout, audits, backups, clear SLAs.

## Data Room Checklist
- Architecture, roadmap, financial model, policies (security, IR), compliance artifacts, test results, monitoring dashboards.

---

This is a draft for discussion. Provide the testnet launch window (4/8/12 weeks) and I’ll replace placeholders with concrete dates and budgets.