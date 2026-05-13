# rustuse

Unified facade crate for the published RustUse sets and child crates.

`rustuse` gives one dependency surface that exposes the current RustUse sets on `main`:

- chemistry primitives from `use-chemistry`
- math primitives from `use-math`
- Rust ecosystem primitives from `use-rust`

The crate re-exports every concrete child crate directly at the top level and also groups them by set so consumers can choose the style they prefer.

## Installation

```toml
[dependencies]
rustuse = "0"
```

Disable default features if you only want one set:

```toml
[dependencies]
rustuse = { version = "0", default-features = false, features = ["chemistry"] }
```

## Usage

### Access child crates directly

```rust
use rustuse::use_element::element_by_symbol;
use rustuse::use_combinatorics::factorial;
use rustuse::use_version::parse_version;

let oxygen = element_by_symbol("O").unwrap();
let count = factorial(5).unwrap();
let version = parse_version("0.1.0").unwrap();

assert_eq!(oxygen.atomic_number, 8);
assert_eq!(count, 120);
assert_eq!(version.to_string(), "0.1.0");
```

### Access crates through set modules

```rust
use rustuse::chemistry::element::element_by_symbol;
use rustuse::math::combinatorics::factorial;
use rustuse::rust::version::next_minor;

let oxygen = element_by_symbol("O").unwrap();
let count = factorial(5).unwrap();
let version = next_minor(&rustuse::rust::version::parse_version("0.1.0").unwrap());

assert_eq!(oxygen.atomic_number, 8);
assert_eq!(count, 120);
assert_eq!(version.to_string(), "0.2.0");
```

## Set modules

- `rustuse::chemistry` groups `use-chemistry` and the published chemistry child crates
- `rustuse::math` groups `use-math` and all published `use-math` child crates
- `rustuse::rust` groups `use-rust`, `use-crate`, and `use-version`

## Release scope

The published `rustuse` `0.1.0` release covered math and rust. The next follow-up release on
`main` adds the published chemistry crates now that the `use-chemistry` workspace is live on
crates.io.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0, in `LICENSE-APACHE`
- MIT license, in `LICENSE-MIT`
