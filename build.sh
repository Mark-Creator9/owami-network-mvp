#!/bin/bash
set -e  # Exit on error

echo "Installing PostgreSQL client..."
apt-get update && apt-get install -y postgresql-client

echo "Running database migrations..."
# Initialize extensions first
PGPASSWORD=$POSTGRES_PASSWORD psql -h $POSTGRES_HOST -U $POSTGRES_USER -d $POSTGRES_DATABASE -p $POSTGRES_PORT -f migrations/20250404000000_init_extensions.sql

# Run other migrations
for migration in migrations/*.sql; do
    if [[ $migration != *"init_extensions.sql" ]]; then
        echo "Applying migration: $migration"
        PGPASSWORD=$POSTGRES_PASSWORD psql -h $POSTGRES_HOST -U $POSTGRES_USER -d $POSTGRES_DATABASE -p $POSTGRES_PORT -f "$migration"
    fi
done

echo "Building application..."
cargo build --release

echo "Starting application..."
./target/release/owami-network