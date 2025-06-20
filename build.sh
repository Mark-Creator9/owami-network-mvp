#!/bin/bash
set -e  # Exit on error

echo "Installing PostgreSQL client..."
apt-get update && apt-get install -y postgresql-client

echo "Waiting for PostgreSQL to be ready..."
until PGPASSWORD=wIVb8cOrk2f8yGLxWBWRvgpBn9xsihhT psql -h dpg-d18a6th5pdvs73ca2gb0-a -U owami_network_db_user -d owami_network_db -c '\q'; do
  echo "PostgreSQL is unavailable - sleeping"
  sleep 1
done

echo "PostgreSQL is up - executing migrations"

# Initialize extensions first
echo "Initializing database extensions..."
PGPASSWORD=wIVb8cOrk2f8yGLxWBWRvgpBn9xsihhT psql -h dpg-d18a6th5pdvs73ca2gb0-a -U owami_network_db_user -d owami_network_db -f migrations/20250404000000_init_extensions.sql

# Run other migrations
for migration in migrations/*.sql; do
    if [[ $migration != *"init_extensions.sql" ]]; then
        echo "Applying migration: $migration"
        PGPASSWORD=wIVb8cOrk2f8yGLxWBWRvgpBn9xsihhT psql -h dpg-d18a6th5pdvs73ca2gb0-a -U owami_network_db_user -d owami_network_db -f "$migration"
    fi
done

echo "Building application with offline mode..."
SQLX_OFFLINE=true cargo build --release

echo "Starting application..."
./target/release/owami-network