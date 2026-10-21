#!/usr/bin/env bash

set -euo pipefail

repository_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repository_root"

cargo run --quiet --no-default-features --features typescript --bin generate-types
exec wasm-pack build --target web --no-default-features --features typescript "$@"
