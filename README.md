# Rust Observability Tool

A small Linux process-execution monitor built with Rust, Aya, and eBPF. The
eBPF kprobe attaches to `__x64_sys_execve` and sends the process ID and
command name to the userspace application through a perf-event array.

## Workspace

- `my-observability-tool-ebpf/` — eBPF kprobe and `EVENTS` map.
- `my-observability-tool/` — Aya userspace loader and perf-event reader.
- `my-observability-tool-common/` — event structure shared by both crates.
- `build.sh` — builds the eBPF object first, then the userspace application.

## Requirements

- Linux with eBPF support and permission to load and attach BPF programs.
- Rust and Cargo, plus the nightly toolchain with `rust-src`.
- `bpf-linker` available from Cargo's bin directory.

Install the Rust components and linker if needed:

```sh
rustup toolchain install nightly --component rust-src
cargo install bpf-linker --locked
```

The build script adds `${CARGO_HOME:-$HOME/.cargo}/bin` to `PATH` for its
build, so a Cargo-installed `bpf-linker` can be found even if that directory
isn't already in the shell's `PATH`.

## Build

From the workspace root:

```sh
./build.sh
```

The default build uses the debug profile. To build release binaries:

```sh
./build.sh release
```

The script compiles `my-observability-tool-ebpf` for
`bpfel-unknown-none` using nightly and `-Z build-std=core`, checks that the
expected eBPF object exists and is non-empty, and then builds the userspace
crate. The resulting eBPF object is embedded in the userspace executable at
compile time; rerun the script after changing the eBPF crate.

## Run

Run on a Linux system with the required BPF permissions (root may be needed):

```sh
sudo ./target/debug/my-observability-tool
```

For the release build:

```sh
sudo ./target/release/my-observability-tool
```

Stop the monitor with Ctrl+C. It prints the CPU, process ID, and command name
for observed executions.

> **WSL2 note:** The kprobe target `__x64_sys_execve` is present in the WSL2
> kernel's symbol table, but loading the rebuilt kprobe there still returned
> `EINVAL` with a verifier log reporting zero processed instructions. A valid
> BPF ELF and an available symbol do not guarantee that the kernel accepts
> BPF program loading. If this occurs, test on a supported Linux kernel and
> inspect the verifier log; rebuilding alone may not resolve a kernel or
> environment limitation.
