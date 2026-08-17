export INFISICAL_DISABLE_UPDATE_CHECK := "true"

run:
	infisical run --env=dev -- cargo run

test:
	infisical run --env=dev -- just _test

tunnel:
	#!/usr/bin/env bash
	token=$(cloudflared tunnel token trash-can)
	cloudflared tunnel run --token $token

# ig_user_id=$(curl -s -X GET "https://graph.instagram.com/v26.0/me?access_token=$INSTAGRAM_ACCESS_TOKEN" | jq -r ".id")
# curl -s -X POST "https://graph.instagram.com/v26.0/$ig_user_id/subscribed_apps?subscribed_fields=messages,message_reactions,messaging_handover,messaging_optins,messaging_seen,story_insights&access_token=$INSTAGRAM_ACCESS_TOKEN" | jq
# curl -s -X GET "https://graph.instagram.com/v26.0/$ig_user_id/subscribed_apps?access_token=$INSTAGRAM_ACCESS_TOKEN" | jq
# curl -X POST "https://graph.instagram.com/v26.0/$ig_user_id/messages" -H "Authorization: Bearer $INSTAGRAM_ACCESS_TOKEN" -H "Content-Type: application/json" \
# 		-d '{ "recipient":{ "id":"1432629015593423" }, "message":{ "text":"<TEXT_OR_LINK>" } }'

_test:
	#!/usr/bin/env bash
	ig_user_id=$(curl -s -X GET "https://graph.instagram.com/v26.0/me?access_token=$INSTAGRAM_ACCESS_TOKEN")
	echo $ig_user_id | jq;


lint:
	@cargo clippy -- \
		--allow clippy::needless_return \
		--allow clippy::uninlined_format_args
