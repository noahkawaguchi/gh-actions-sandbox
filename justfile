set dotenv-load := true

dc-test := "docker compose -p sandbox-test -f docker-compose.test.yml"

# Run tests in Docker
test:
    {{dc-test}} run --rm --build test

# Remove the Compose stack used for testing
test-clean:
    {{dc-test}} down --rmi local
