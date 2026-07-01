####################################################################################################
# Config
####################################################################################################

set dotenv-load

pg-tag := '18.4-alpine3.23' # Should match the tag in `ci-cd.yml`

####################################################################################################
# All checks to match CI
####################################################################################################

# Run tests, lints, format checking, and spell checking to match CI (default recipe)
all-checks: test lint fmt-check spell-check

####################################################################################################
# Testing
####################################################################################################

test-db-name := 'sandbox_test_db'
test-db-port := '2345'
test-db-url := 'postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@localhost:' \
               + test-db-port + '/$POSTGRES_DB'

# Run tests using an ephemeral Postgres container (default recipe)
test:
    #!/usr/bin/env bash
    trap 'just test-clean' EXIT

    if docker inspect {{ test-db-name }} >/dev/null 2>&1; then \
        docker rm -f {{ test-db-name }}; sleep 3; fi
    docker run --rm --name {{ test-db-name }} --env-file .env -p {{ test-db-port }}:5432 -d \
        postgres:{{ pg-tag }}
    until docker exec {{ test-db-name }} pg_isready > /dev/null 2>&1; do sleep 1; done
    DATABASE_URL={{test-db-url}} cargo test --workspace --all-targets
    docker stop {{ test-db-name }} > /dev/null 2>&1 &

# Stop the test database container
[private]
test-clean:
    docker stop {{ test-db-name }}

####################################################################################################
# Other checks
####################################################################################################

# Lint with Clippy (denying warnings)
lint *ARGS:
    cargo clippy --workspace --all-targets {{ ARGS }} -- --deny warnings

# Lint for aarch64-unknown-linux-gnu, x86_64-unknown-linux-gnu, and aarch64-apple-darwin
lint-targets: (lint '--target' 'aarch64-unknown-linux-gnu') \
              (lint '--target' 'x86_64-unknown-linux-gnu') \
              (lint '--target' 'aarch64-apple-darwin')

# Check formatting
fmt-check:
    cargo fmt --all --check && echo 'Formatting check passed'

# Check spelling with Codebook
spell-check:
    git ls-files -z | xargs -0 codebook-lsp lint
