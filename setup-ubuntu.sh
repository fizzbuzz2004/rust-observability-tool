#!/usr/bin/env bash
set -euo pipefail

if [[ "${EUID}" -eq 0 ]]; then
    echo "Run this script as your normal Ubuntu user, without sudo." >&2
    exit 1
fi

if [[ ! -r /etc/os-release ]]; then
    echo "Cannot identify this operating system; expected Ubuntu 24.04." >&2
    exit 1
fi

# shellcheck disable=SC1091
source /etc/os-release
if [[ "${ID:-}" != "ubuntu" || "${VERSION_ID:-}" != "24.04" ]]; then
    echo "This setup script supports Ubuntu 24.04; detected ${PRETTY_NAME:-unknown}." >&2
    exit 1
fi

if [[ "$(uname -m)" != "x86_64" ]]; then
    echo "This project targets x86-64 (__x64_sys_execve); detected $(uname -m)." >&2
    exit 1
fi

if ! command -v sudo >/dev/null 2>&1; then
    echo "sudo is required to install Ubuntu packages." >&2
    exit 1
fi

sudo apt-get update
sudo apt-get install -y \
    build-essential \
    ca-certificates \
    curl \
    git \
    pkg-config \
    libelf-dev \
    zlib1g-dev

export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"

if ! command -v rustup >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
        | sh -s -- -y --profile minimal
fi

source "${CARGO_HOME:-$HOME/.cargo}/env"
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"

rustup toolchain install nightly --component rust-src

if ! command -v bpf-linker >/dev/null 2>&1; then
    cargo install bpf-linker --locked
fi

echo
echo "Ubuntu build prerequisites are installed."
echo "Build the project from its repository root with: ./build.sh"
echo "Run it with: sudo ./target/debug/my-observability-tool"
