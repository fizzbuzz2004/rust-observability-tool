#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT_DIR"
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"

PROFILE="${1:-debug}"
if [[ "$#" -gt 1 ]]; then
    echo "Usage: $0 [debug|release]" >&2
    exit 2
fi

case "$PROFILE" in
    debug|release) ;;
    *)
        echo "Usage: $0 [debug|release]" >&2
        exit 2
        ;;
esac

BUILD_ARGS=()
if [[ "$PROFILE" == "release" ]]; then
    BUILD_ARGS+=(--release)
fi

cargo +nightly build \
    -Z build-std=core \
    --target bpfel-unknown-none \
    -p my-observability-tool-ebpf \
    "${BUILD_ARGS[@]}"

EBPF_OBJECT="target/bpfel-unknown-none/$PROFILE/my-observability-tool"
if [[ ! -s "$EBPF_OBJECT" ]]; then
    echo "eBPF build did not produce a non-empty object: $EBPF_OBJECT" >&2
    exit 1
fi

cargo build -p my-observability-tool "${BUILD_ARGS[@]}"
