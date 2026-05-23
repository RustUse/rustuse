# rustuse

Unified facade crate for the published RustUse set facades.

`rustuse` gives one dependency surface across the repository-level RustUse facade crates that are
already live on crates.io. It keeps direct child-crate reexports for the child-rich sets and adds
facade-only grouped modules for every published set facade.

- set-level facade modules for all published repository-level `use-*` crates
- child-rich set surfaces from `use-chemistry`, `use-constants`, `use-math`, `use-net`,
  `use-pattern`, `use-rust`, and `use-web`

The crate re-exports published crates at the top level and also groups them by set so consumers can
choose the style they prefer. Geometry is available both as its own set module and through the
existing `rustuse::math::geometry` compatibility path.

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

Or enable one of the facade-only set modules directly:

```toml
[dependencies]
rustuse = { version = "0", default-features = false, features = ["time", "wave"] }
```

## Usage

### Access child crates from the child-rich sets

```rust
# #[cfg(feature = "full")]
# fn main() {
use rustuse::use_earth_constants::EARTH_RADIUS_MEAN;
use rustuse::use_math_constants::TAU;
use rustuse::use_element::element_by_symbol;
use rustuse::use_isotope::isotope_by_symbol;
use rustuse::use_combinatorics::factorial;
use rustuse::use_ip::is_ipv4;
use rustuse::use_match::{MatchSpan, slice_match};
use rustuse::use_regex::count_regex_matches;
use rustuse::use_url::parse_url_basic;
use rustuse::use_version::parse_version;

let oxygen = element_by_symbol("O").unwrap();
let carbon_14 = isotope_by_symbol("C", 14).unwrap();
let count = factorial(5).unwrap();
let docs_url = parse_url_basic("https://rustuse.org/docs").unwrap();
let span = MatchSpan { start: 0, end: 4 };
let version = parse_version("0.1.0").unwrap();

assert!(EARTH_RADIUS_MEAN > 0.0);
assert_eq!(TAU, 2.0 * std::f64::consts::PI);
assert_eq!(oxygen.atomic_number, 8);
assert_eq!(carbon_14.neutron_count(), 8);
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

### Access published set facades through grouped modules

```rust
# #[cfg(feature = "full")]
# fn main() {
use rustuse::color::facade as color;
use rustuse::physics::facade as physics;
use rustuse::time::facade as time;
use rustuse::wave::facade as wave;

let color = color::parse_hex_rgb("#3366CC").unwrap();
let kinetic = physics::kinetic_energy(2.0, 3.0);
let duration = time::seconds_to_duration(3);
let period = wave::period_seconds(4.0).unwrap();

assert_eq!(color.to_hex_rgb(), "#3366CC");
assert_eq!(kinetic, 9.0);
assert_eq!(duration.as_secs(), 3);
assert_eq!(period, 0.25);
# }
# #[cfg(not(feature = "full"))]
# fn main() {}
```

## Published set modules

- `rustuse::accessibility`, `rustuse::acoustics`, `rustuse::astronomy`,
  `rustuse::bioinformatics`, `rustuse::biology`, `rustuse::calendar`, `rustuse::cli`,
  `rustuse::color`, `rustuse::config`, `rustuse::control`, `rustuse::data`,
  `rustuse::diagnostic`, `rustuse::ecology`, `rustuse::electronics`, `rustuse::encoding`,
  `rustuse::fs`, `rustuse::geography`, `rustuse::geology`, `rustuse::geometry`,
  `rustuse::graph`, `rustuse::id`, `rustuse::locale`, `rustuse::materials`,
  `rustuse::measure`, `rustuse::media`, `rustuse::meteorology`, `rustuse::optics`,
  `rustuse::optimization`, `rustuse::os`, `rustuse::physics`, `rustuse::presence`,
  `rustuse::quant`, `rustuse::robotics`, `rustuse::rustacean`, `rustuse::signal`,
  `rustuse::simulation`, `rustuse::stats`, `rustuse::text`, `rustuse::time`,
  `rustuse::typography`, `rustuse::units`, `rustuse::validate`, and `rustuse::wave` expose the
  published set-level facade crates.
- `rustuse::chemistry`, `rustuse::constants`, `rustuse::math`, `rustuse::net`,
  `rustuse::pattern`, `rustuse::rust`, and `rustuse::web` keep the richer grouped access for
  the published child crates that `rustuse` already re-exports directly.
- `rustuse::math::geometry` remains available as a compatibility alias for `use-geometry`.

## Release scope

The published `rustuse` `0.1.3` release covers the earlier chemistry, constants, math, net,
pattern, rust, web, and initial set-facade surface. The current `main` branch expands the default
`full` feature to the full set of repository-level RustUse facade crates that are already published
on crates.io.

The child-rich sets continue to use local path dependencies for the focused crates that `rustuse`
re-exports directly. The facade-only set modules use crates.io version pins so the published
integration surface can coexist cleanly with local sibling workspaces that reuse crate names in
different domains.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0, in `LICENSE-APACHE`
- MIT license, in `LICENSE-MIT`
