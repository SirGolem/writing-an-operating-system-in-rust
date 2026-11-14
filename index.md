# Writing an Operating System in Rust

![operating-system](https://img.shields.io/badge/operating--system-gray) ![rust](https://img.shields.io/badge/rust-gray)
![writing-an-operating-system-in-rust](https://img.shields.io/badge/writing--an--operating--system--in--rust-gray)

A [Rust](https://rust-lang.org) operating system implementation following [Philip Oppermann](https://github.com/phil-opp)'s
series '[Writing an OS in Rust](https://os.phil-opp.com)'.

## Key Differences

- Code structure is slightly different due to personal preferences.
- Some crates and features of nightly Rust have changed since the posts were last updated.

## Repository Structure

Because the series is linear, this repository is too. \
The code for each post has its own commit and tag, with the same name as the corresponding post. \
All other commit messages are prefixed to denote their type.

## Usage

### Prerequisites

The following tools are required to use this project. \
The versions listed are those used during development &ndash; newer versions _may_ work, but may not.

- **Nightly Rust Toolchain:** 1.102.0-nightly
    - `cargo`
    - `llvm-tools`
    - `rust-src`
    - `rustc`
    - `rustfmt`
- **pnpm:** 12.18.0
- **Quick Emulator (QEMU):** 11.1.3
    - `x86_64`

To install dependencies, run:

```
pnpm install
```

### Build

To build in development mode, run:

```
cargo build
```

The binary will be located at `target/target/debug/operating-system`.

To build in release mode, run:

```
cargo build --release
```

The binary will be located at `target/target/release/operating-system`.

### Check

To check the code for quality issues, run:

```
cargo check
```

### Format

To format all supported files, run:

```
pnpm format
```

This runs `cargo fmt` and then `prettier`.

### Refresh

To refresh the local environment, run:

```
pnpm refresh
```

This cleans artefacts, updates dependencies, formats files and checks code quality.

### Run

To build and then run the system in development mode, run:

```
cargo run
```

To build and then run the system in release mode, run:

```
cargo run --release
```

A window will appear once the build has completed.

### Test

To run the automated integration and unit tests, run:

```
cargo test
```
