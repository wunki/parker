# Used for debugging, can print any variable in the Makefile.
# Use it with: `make print-VARIABLE`
print-%  : ; @echo $* = $($*)

# Get's everything ready so we can start our development.
.PHONY: migrate
migrate:
	sqlx database create
	sqlx migrate run
