# API Guide

This guide documents the core REST API endpoints with example requests and expected responses.

Base URL (local): `http://localhost:3002`

## Authentication
- POST `/api/auth/register`
- POST `/api/auth/login`
- GET `/api/auth/profile` (requires `Authorization: Bearer <JWT>`)

### Example: Register
```http
POST /api/auth/register HTTP/1.1
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "Str0ngP@ss!",
  "name": "Tester"
}
```

### Example: Login
```http
POST /api/auth/login HTTP/1.1
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "Str0ngP@ss!"
}
```

Response:
```json
{
  "token": "<JWT>",
  "user": { "id": 1, "email": "user@example.com" }
}
```

## Token
- GET `/api/token/info`
- GET `/api/token/balance/:address`
- POST `/api/token/transfer` (requires JWT)
- POST `/api/token/mint/:address` (requires admin or demo mode)
- POST `/api/token/mint` (JSON payload)
- GET `/api/token/transactions`

### Example: Mint (JSON)
```http
POST /api/token/mint HTTP/1.1
Content-Type: application/json
Authorization: Bearer <JWT>

{
  "address": "test_address_1",
  "amount": 1000
}
```

### Example: Transfer
```http
POST /api/token/transfer HTTP/1.1
Content-Type: application/json
Authorization: Bearer <JWT>

{
  "from": "test_address_1",
  "to": "test_address_2",
  "amount": 100
}
```

## Blockchain
- GET `/api/blockchain/info`
- GET `/api/blockchain/blocks`
- POST `/api/blockchain/mine`

## DApp
- GET `/api/dapps`
- POST `/api/dapps`
- GET `/api/dapps/:id`

## Health & Landing
- GET `/api/health`
- GET `/landing`

## Notes
- Some endpoints require JWT. Use the login token in the `Authorization` header.
- Transfer may return 400 if sender balance is insufficient.