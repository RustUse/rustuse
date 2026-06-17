# rustuse

[![CI](https://github.com/RustUse/rustuse/actions/workflows/ci.yml/badge.svg)](https://github.com/RustUse/rustuse/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/rustuse?label=crates.io)](https://crates.io/crates/rustuse)
[![Docs.rs](https://docs.rs/rustuse/badge.svg)](https://docs.rs/rustuse)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![Rust 2024](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
[![Unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](#)

Composable facade for RustUse utility crates.

`rustuse` provides a single top-level entry point for published RustUse domain facades. Each enabled feature exposes one domain facade crate at the root of this crate, such as `rustuse::use_geometry`, `rustuse::use_web`, or `rustuse::use_math`.

The root `rustuse` crate does not expose every focused child crate directly. Focused child crates are accessed through their domain facade re-exports or by depending on the focused child crate directly.

## Installation

Enable the domain facades you need:

```bash
cargo add rustuse --features geometry
```

Enable multiple domains:

```bash
cargo add rustuse --features geometry,web,math
```

Enable all published RustUse domain facades:

```bash
cargo add rustuse --features full
```

Or configure features manually:

```toml
[dependencies]
rustuse = { version = "0.2", features = ["geometry", "web", "math"] }
```

The default feature set is empty, so `rustuse` only exposes the facades you explicitly enable.

## Usage

Import an enabled domain facade from the crate root:

```rust,ignore
use rustuse::use_geometry as geometry;
```

Use multiple facades together:

```rust,ignore
use rustuse::use_geometry as geometry;
use rustuse::use_math as math;
use rustuse::use_web as web;
```

Enable `full` for broad exploration:

```rust,ignore
use rustuse::use_color as color;
use rustuse::use_physics as physics;
use rustuse::use_time as time;
use rustuse::use_wave as wave;
```

These imports require the corresponding Cargo features to be enabled.

## Architecture

The `rustuse` crate maintains a facade-only dependency structure:

1. **Top-level inventory**: one published facade crate per domain.
2. **Feature-gated access**: each domain facade is enabled explicitly through a Cargo feature.
3. **Root-level re-exports**: enabled facades are exposed as `rustuse::use_*`.
4. **Child crate access**: focused child crates are reached through domain facade re-exports or direct dependencies.
5. **No child-crate flattening**: `rustuse` does not expose every focused crate directly.

This keeps the root crate small, predictable, and stable as domain workspaces grow.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0, in `LICENSE-APACHE`
- MIT license, in `LICENSE-MIT`
