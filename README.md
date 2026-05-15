# rustuse

Unified facade crate for the published RustUse sets and child crates.

`rustuse` gives one dependency surface that exposes the current RustUse sets on `main`:

- constants primitives from `use-constants`
- chemistry primitives from `use-chemistry`
- math primitives from `use-math`
- networking primitives from `use-net`
- pattern primitives from `use-pattern`
- Rust ecosystem primitives from `use-rust`
- web and HTTP primitives from `use-web`

The crate re-exports every concrete child crate directly at the top level and also groups them by set so consumers can choose the style they prefer.

## Installation

```toml
[dependencies]
rustuse = "0"
```

Disable default features if you only want one set:

```toml
[dependencies]
rustuse = { version = "0", default-features = false, features = ["constants"] }
```

Or enable the networking surface directly:

```toml
[dependencies]
rustuse = { version = "0", default-features = false, features = ["net"] }
```

Or enable the pattern surface directly:

```toml
[dependencies]
rustuse = { version = "0", default-features = false, features = ["pattern"] }
```

## Usage

### Access child crates directly

```rust
# #[cfg(feature = "full")]
# fn main() {
use rustuse::use_earth_constants::EARTH_RADIUS_MEAN;
use rustuse::use_math_constants::TAU;
use rustuse::use_element::element_by_symbol;
use rustuse::use_combinatorics::factorial;
use rustuse::use_ip::is_ipv4;
use rustuse::use_match::{MatchSpan, slice_match};
use rustuse::use_regex::count_regex_matches;
use rustuse::use_url::parse_url_basic;
use rustuse::use_version::parse_version;

let oxygen = element_by_symbol("O").unwrap();
let count = factorial(5).unwrap();
let docs_url = parse_url_basic("https://rustuse.org/docs").unwrap();
let span = MatchSpan { start: 0, end: 4 };
let version = parse_version("0.1.0").unwrap();

assert!(EARTH_RADIUS_MEAN > 0.0);
assert_eq!(TAU, 2.0 * std::f64::consts::PI);
assert_eq!(oxygen.atomic_number, 8);
assert_eq!(count, 120);
assert!(is_ipv4("127.0.0.1"));
assert_eq!(slice_match("rustacean", &span), Some("rust"));
assert_eq!(count_regex_matches(r"\d+", "v1 v20"), Some(2));
assert_eq!(docs_url.host.as_deref(), Some("rustuse.org"));
assert_eq!(version.to_string(), "0.1.0");
# }
# #[cfg(not(feature = "full"))]
# fn main() {}
```

### Access crates through set modules

```rust
# #[cfg(feature = "full")]
# fn main() {
use rustuse::constants::earth;
use rustuse::constants::math;
use rustuse::chemistry::element::element_by_symbol;
use rustuse::math::combinatorics::factorial;
use rustuse::net::{ip, socket};
use rustuse::pattern::{matchers, regex};
use rustuse::rust::version::next_minor;
use rustuse::web::status::reason_phrase;
use rustuse::web::url::parse_url_basic;

let radius = earth::EARTH_RADIUS_MEAN;
let oxygen = element_by_symbol("O").unwrap();
let count = factorial(5).unwrap();
let docs_url = parse_url_basic("https://rustuse.org/docs").unwrap();
let version = next_minor(&rustuse::rust::version::parse_version("0.1.0").unwrap());
let circumference = math::TAU * 3.0;
let endpoint = socket::parse_socket_endpoint("localhost:8080").unwrap();
let span = matchers::MatchSpan { start: 0, end: 4 };

assert!(radius > 0.0);
assert!(circumference > 0.0);
assert_eq!(oxygen.atomic_number, 8);
assert_eq!(count, 120);
assert!(ip::is_ipv4("127.0.0.1"));
assert_eq!(endpoint.port, 8080);
assert_eq!(matchers::slice_match("rustacean", &span), Some("rust"));
assert_eq!(docs_url.host.as_deref(), Some("rustuse.org"));
assert_eq!(regex::count_regex_matches(r"\d+", "v1 v20"), Some(2));
assert_eq!(reason_phrase(404), Some("Not Found"));
assert_eq!(version.to_string(), "0.2.0");
# }
# #[cfg(not(feature = "full"))]
# fn main() {}
```

## Set modules

- `rustuse::constants` groups `use-constants` and the focused constants child crates
- `rustuse::chemistry` groups `use-chemistry` and the published chemistry child crates
- `rustuse::math` groups `use-math` and all published `use-math` child crates
- `rustuse::net` groups `use-net` and the focused networking child crates
- `rustuse::pattern` groups `use-pattern` and the focused pattern child crates
- `rustuse::rust` groups `use-rust`, `use-crate`, and `use-version`
- `rustuse::web` groups `use-web` and the focused `use-web` child crates

## Release scope

The published `rustuse` `0.1.2` release covers chemistry, constants, math, and rust. The current
`main` branch also stages the `use-net` workspace for the next facade release once those crates are
published. The constants surface includes the published `use-constants` umbrella crate plus the
focused constants child crates, including `use-earth-constants`.

The current `main` branch also wires in the `use-pattern` workspace. That pattern surface remains
crates.io-gated until the focused `use-pattern` crates publish first and the `use-pattern` facade
follows.

The current `main` branch also wires in the `use-web` workspace. That web surface remains
crates.io-gated until the focused `use-web` crates publish first and the `use-web` facade follows.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0, in `LICENSE-APACHE`
- MIT license, in `LICENSE-MIT`
