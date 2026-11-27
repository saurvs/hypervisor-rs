#!/bin/sh
# Ad-hoc sign a binary with the com.apple.security.hypervisor entitlement and
# run it. Hypervisor.framework refuses to create a VM in unsigned processes.
#
# Usage: scripts/run-signed.sh path/to/binary [args...]
#
# This repository's .cargo/config.toml uses it as the cargo runner on macOS,
# so `cargo run --example ...` and `cargo test` sign automatically.
set -eu

if [ "$#" -lt 1 ]; then
    echo "usage: $0 BINARY [ARGS...]" >&2
    exit 2
fi

entitlements="$(cd "$(dirname "$0")/.." && pwd)/entitlements.plist"
codesign --sign - --entitlements "$entitlements" --force "$1" 2>/dev/null
exec "$@"
