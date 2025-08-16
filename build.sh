#!/bin/bash
set -e  # Exit on error

echo "Installing PostgreSQL client..."
apt-get update && apt-get install -y postgresql-client

echo "Waiting for PostgreSQL to be ready..."
until PGPASSWORD="$DB_PASSWORD" psql -h "$DB_HOST" -U "$DB_USER" -d "$DB_NAME" -c '\q'; do
  echo "PostgreSQL is unavailable - sleeping"
  sleep 1
done

echo "PostgreSQL is up - executing migrations"

# Initialize extensions first
echo "Initializing database extensions..."
PGPASSWORD="$DB_PASSWORD" psql -h "$DB_HOST" -U "$DB_USER" -d "$DB_NAME" -f migrations/20250404000000_init_extensions.sql

# Run other migrations
for migration in migrations/*.sql; do
    if [[ $migration != *"init_extensions.sql" ]]; then
        echo "Applying migration: $migration"
        PGPASSWORD="$DB_PASSWORD" psql -h "$DB_HOST" -U "$DB_USER" -d "$DB_NAME" -f "$migration"
    fi
done

echo "Building application..."
cargo build --release

echo "Starting application..."
./target/release/owami-network