#!/usr/bin/env bash
set -eo pipefail

export PGPASSWORD=${POSTGRES_PASSWORD}
until psql -h "localhost" -U "${POSTGRES_USER}" -p "${POSTGRES_PORT}" -d "postgres" -c '\q' 2>/dev/null; do
  >&2 echo "PostgreSQL is not ready yet. Sleeping for 1 second..."
  sleep 1
done
