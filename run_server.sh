#!/bin/sh
exec cargo run \
    --quiet \
    --release \
    --target-dir=/tmp/dns-server-target \
    --manifest-path $(dirname $0)/Cargo.toml -- "$@"
