
export COMPOSE_DOCKER_CLI_BUILD=1
export DOCKER_BUILDKIT=1
EXPORT = export RUSTPATH=$(PWD)

test:
	cargo test -- --test-threads 1 --nocapture
	