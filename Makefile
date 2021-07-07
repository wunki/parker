# PostgreSQL configuration defaults, overwritten by environment variables.
POSTGRES_USER ?= postgres
POSTGRES_PASSWORD ?= postgres
POSTGRES_PORT ?= 5432
POSTGRES_DB ?= parker_dev

export DATABASE_URL=postgres://$(POSTGRES_USER):$(POSTGRES_PASSWORD)@localhost:$(POSTGRES_PORT)/$(POSTGRES_DB)

# Used for debugging, can print any variable in the Makefile.
# Use it with: `make print-VARIABLE`
print-%  : ; @echo $* = $($*)

# Get's everything ready so we can start our development.
.PHONY: start-dev
start-dev: postgres postgres_ready
	sqlx database create
	sqlx migrate run

# Stops development. It was a nice day.
.PHONY: stop-dev
stop-dev:
	docker stop 'postgres-$(POSTGRES_DB)'

# Starts the PostgreSQL database as a Docker container.
.PHONY: postgres
postgres:
	docker run --rm --detach \
		-e POSTGRES_PASSWORD='$(POSTGRES_PASSWORD)' \
		-e POSTGRES_USER='$(POSTGRES_USER)' \
		-e POSTGRES_DB='$(POSTGRES_DB)' \
		-p $(POSTGRES_PORT):5432 \
		--name 'postgres-$(POSTGRES_DB)' \
		postgres:13

# Checks if the PostgreSQL database is up and running.
.PHONY: postgres_ready
postgres_ready:
	timeout 10 ./scripts/db_available.sh || exit -1

