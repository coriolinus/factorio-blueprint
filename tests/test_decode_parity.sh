#!/usr/bin/env bash

set -e

cd "$(git rev-parse --show-toplevel)"

if ! [ -x target/release/factorio-blueprint ]; then
    cargo build --release
fi

check () {
    path="$1"
    filename="$(basename "$path")"

    # pipe the output through jq so inconsequential style differences vanish
    python_output="$(src/deflate.py "$path" | jq .)"
    rust_output="$(target/release/factorio-blueprint decode --file "$path" | jq .)"

    if [ "$python_output" != "$rust_output" ]; then
        exit_code=1
        echo >&2 "decode mismatch for $filename"
    fi
}

exit_code=0

for example in tests/examples/*.txt; do
    check "$example"
done

if [ "$exit_code" -eq 0 ]; then
    echo "ok"
fi

exit "$exit_code"
