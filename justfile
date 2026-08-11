export INFISICAL_DISABLE_UPDATE_CHECK := "true"

run:
	infisical run --env=dev -- cargo run

lint:
	@cargo clippy -- \
		--allow clippy::needless_return \
		--allow clippy::uninlined_format_args
