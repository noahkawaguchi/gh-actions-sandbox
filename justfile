set dotenv-load := true

pg-tag := "18.0-alpine3.22"
test-db-name := "sandbox_test_db"
test-db-port := "2345"
test-db-url := "postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@localhost:" \
               + test-db-port + "/$POSTGRES_DB"

# Run tests using an ephemeral Postgres container
test:
    if docker inspect {{test-db-name}} >/dev/null 2>&1; then \
        docker rm -f {{test-db-name}}; sleep 3; fi
    docker run --rm --name {{test-db-name}} --env-file .env -p {{test-db-port}}:5432 -d \
        postgres:{{pg-tag}}
    DATABASE_URL={{test-db-url}} cargo test --workspace --all-targets
    docker stop {{test-db-name}} > /dev/null 2>&1 &

# Manually stop the test database container
test-clean:
    docker stop {{test-db-name}}
