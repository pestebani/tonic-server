#!/bin/sh
set -e


DB_NAME=${AGENDA_DB:-agenda-tests}

# Create the database if it doesn't already exist
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE DATABASE "$DB_NAME";
EOSQL
