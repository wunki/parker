#!/usr/bin/env bash
set -eo pipefail

# This script is used to check when the database becomes available. It's
# persistent and will never stop trying to connect. So use it with the
# `timeout` command. E.g. to keep checking for 10 seconds:
# 
#   timeout 10 ./scripts/db_available.sh || exit -1
# 
# What's the `||`? Glad you asked! That means if the first command succeeds,
# the second one will _not_ be executed.
# 
# So in this case, it will throw an `exit -1` once the timeout has been
# reached. The `exit` command will let us know about the failure, because
# everything else then 0 is considered a failure!
#
# Now onto the actual script...which is a lot shorter than this comment.

export PGPASSWORD=${POSTGRES_PASSWORD}
until psql -h "localhost" -U "${POSTGRES_USER}" -p "${POSTGRES_PORT}" -d "postgres" -c '\q' 2>/dev/null; do
  >&2 echo "PostgreSQL is not ready yet. Sleeping for 1 second..."
  sleep 1
done
