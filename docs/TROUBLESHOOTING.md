# Troubleshooting

## Common Issues
- **Database connection fails**: Verify `DATABASE_URL` in `.env` is correct and PostgreSQL is running.
- **Migrations fail**: Ensure `sqlx` is installed and DB is reachable; check migration order.
- **JWT errors (401/403)**: Ensure `JWT_SECRET` is set before starting the server.
- **Transfer returns 400**: Sender likely has insufficient balance; mint or adjust demo flow.
- **Port conflicts**: Change `PORT` in `.env` to a free port.

## Logs
- **App logs**: `logs/server.out.log`, `logs/server.err.log`
- **Audit logs**: `logs/audit.log`

## Getting Help
- Run `cargo test` to validate setup.
- Attach logs and environment details when filing issues.