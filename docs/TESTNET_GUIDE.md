# Testnet Guide

This guide explains how to connect to the local dev environment and an external testnet (Render) once deployed.

## Local
- Base URL: `http://localhost:3002`
- Steps: follow `docs/QUICKSTART.md`

## Render (Planned)
- Base URL: `https://<your-service>.onrender.com`
- Health: `GET /api/health`
- Landing: `GET /landing`
- Logs: Render dashboard → Logs

## Accounts & Funding
- For demos, pre-fund addresses using the mint endpoint (admin only or demo mode).
- Consider a faucet service for public testnet.

## Rate Limits
- Default rate limiting is enabled; testers should avoid spamming endpoints.

## Reporting Issues
- Open GitHub issues with logs and steps to reproduce.