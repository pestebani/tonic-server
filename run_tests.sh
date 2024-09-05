#!/bin/bash

# Run the tests

docker compose -f docker-compose-tests.yaml up -d

export DATABASE_URL=postgres://postgres:postgres@localhost:5432/agenda-tests

sleep 5s
cargo test -- --test-threads=1


docker compose -f docker-compose-tests.yaml down
