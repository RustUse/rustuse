# rustuse

Unified facade crate for the published RustUse sets and child crates.

`rustuse` gives one crates.io dependency that exposes the current first-wave RustUse sets:

- math primitives from `use-math`
- Rust ecosystem primitives from `use-rust`

The crate re-exports every concrete child crate directly at the top level and also groups them by set so consumers can choose the style they prefer.

## Installation

```toml
[dependencies]
rustuse = "0.1.0"
```

Disable default features if you only want one set:

```toml
[dependencies]
rustuse = { version = "0.1.0", default-features = false, features = ["rust"] }
```

## Usage

### Access child crates directly

```rust
use rustuse::use_combinatorics::factorial;
use rustuse::use_version::parse_version;

let count = factorial(5).unwrap();
let version = parse_version("0.1.0").unwrap();

assert_eq!(count, 120);
assert_eq!(version.to_string(), "0.1.0");
```

### Access crates through set modules

```rust
use rustuse::math::combinatorics::factorial;
use rustuse::rust::version::next_minor;

let count = factorial(5).unwrap();
let version = next_minor(&rustuse::rust::version::parse_version("0.1.0").unwrap());

assert_eq!(count, 120);
assert_eq!(version.to_string(), "0.2.0");
```

## Set modules

- `rustuse::math` groups `use-math` and all published `use-math` child crates
- `rustuse::rust` groups `use-rust`, `use-crate`, and `use-version`

## First release scope

The first public `rustuse` release intentionally excludes chemistry while the
chemistry crates are still unpublished on crates.io.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0, in `LICENSE-APACHE`
- MIT license, in `LICENSE-MIT`
