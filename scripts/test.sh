#!/bin/sh
set -e

psql -U lemmy -d postgres -c "DROP DATABASE IF EXISTS lemmy;"
psql -U lemmy -d postgres -c "CREATE DATABASE lemmy;"

export LEMMY_DATABASE_URL=postgres://lemmy:12345678@localhost:5432/lemmy
# tests are executed in working directory crates/api (or similar),
# so to load the config we need to traverse to the repo root
export LEMMY_CONFIG_LOCATION=../../config/config.hjson
RUST_BACKTRACE=1 \
  cargo test --workspace --no-fail-fast
