#!/bin/bash
set -e

PACKAGE="$1"
echo "$PACKAGE"

psql -U lemmy -d postgres -c "DROP DATABASE IF EXISTS lemmy;"
psql -U lemmy -d postgres -c "CREATE DATABASE lemmy;"

export LEMMY_DATABASE_URL=postgres://lemmy:12345678@localhost:5432/lemmy
# tests are executed in working directory crates/api (or similar),
# so to load the config we need to traverse to the repo root
export LEMMY_CONFIG_LOCATION=../../config/config.hjson
export RUST_BACKTRACE=1

if [ -n "$PACKAGE" ];
then
  cargo test -p $PACKAGE --all-features --no-fail-fast
else
  cargo test --workspace --no-fail-fast
fi

# Add this to do printlns: -- --nocapture
