export INFISICAL_DISABLE_UPDATE_CHECK := "true"

run:
	infisical run --env=dev -- cargo run

tunnel:
	#!/usr/bin/env bash
	token=$(cloudflared tunnel token trash-can)
	cloudflared tunnel run --token $token

test:
	#!/usr/bin/env bash
	curl -X GET http://localhost:9999/instagram

lint:
	@cargo clippy -- \
		--allow clippy::needless_return \
		--allow clippy::uninlined_format_args
