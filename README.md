# rustuse

Unified facade crate for the published RustUse sets and child crates.

`rustuse` gives one crates.io dependency that exposes the currently implemented RustUse sets:

- chemistry primitives from `use-chemistry`
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
rustuse = { version = "0.1.0", default-features = false, features = ["chemistry"] }
```

## Usage

### Access child crates directly

```rust
use rustuse::use_element::element_by_symbol;
use rustuse::use_version::parse_version;

let carbon = element_by_symbol("C").unwrap();
let version = parse_version("0.1.0").unwrap();

assert_eq!(carbon.atomic_number, 6);
assert_eq!(version.to_string(), "0.1.0");
```

### Access crates through set modules

```rust
use rustuse::chemistry::element::element_by_atomic_number;
use rustuse::math::combinatorics::factorial;
use rustuse::rust::version::next_minor;

let oxygen = element_by_atomic_number(8).unwrap();
let count = factorial(5).unwrap();
let version = next_minor(&rustuse::rust::version::parse_version("0.1.0").unwrap());

assert_eq!(oxygen.symbol, "O");
assert_eq!(count, 120);
assert_eq!(version.to_string(), "0.2.0");
```

## Set modules

- `rustuse::chemistry` groups `use-element`, `use-periodic-table`, `use-atomic-number`, `use-atomic-mass`, and `use-electron-shell`
- `rustuse::math` groups `use-math` and all published `use-math` child crates
- `rustuse::rust` groups `use-rust` and its child crates

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
