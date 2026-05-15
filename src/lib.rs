#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Thin facade re-exports for the available RustUse crate sets.

#[cfg(feature = "use-astronomical-constants")]
pub use use_astronomical_constants;
#[cfg(feature = "use-chemical-constants")]
pub use use_chemical_constants;
#[cfg(feature = "use-computing-constants")]
pub use use_computing_constants;
#[cfg(feature = "use-constants")]
pub use use_constants;
#[cfg(feature = "use-earth-constants")]
pub use use_earth_constants;
#[cfg(feature = "use-math-constants")]
pub use use_math_constants;
#[cfg(feature = "use-physical-constants")]
pub use use_physical_constants;

#[cfg(feature = "use-atomic-mass")]
pub use use_atomic_mass;
#[cfg(feature = "use-atomic-number")]
pub use use_atomic_number;
#[cfg(feature = "use-chemistry")]
pub use use_chemistry;
#[cfg(feature = "use-electron-shell")]
pub use use_electron_shell;
#[cfg(feature = "use-element")]
pub use use_element;
#[cfg(feature = "use-periodic-table")]
pub use use_periodic_table;

#[cfg(feature = "use-algebra")]
pub use use_algebra;
#[cfg(feature = "use-calculus")]
pub use use_calculus;
#[cfg(feature = "use-catalan")]
pub use use_catalan;
#[cfg(feature = "use-combinatorics")]
pub use use_combinatorics;
#[cfg(feature = "use-complex")]
pub use use_complex;
#[cfg(feature = "use-geometry")]
pub use use_geometry;
#[cfg(feature = "use-integer")]
pub use use_integer;
#[cfg(feature = "use-linear")]
pub use use_linear;
#[cfg(feature = "use-logic")]
pub use use_logic;
#[cfg(feature = "use-math")]
pub use use_math;
#[cfg(feature = "use-number")]
pub use use_number;
#[cfg(feature = "use-probability")]
pub use use_probability;
#[cfg(feature = "use-rational")]
pub use use_rational;
#[cfg(feature = "use-real")]
pub use use_real;
#[cfg(feature = "use-series")]
pub use use_series;
#[cfg(feature = "use-set")]
pub use use_set;
#[cfg(feature = "use-statistics")]
pub use use_statistics;
#[cfg(feature = "use-trigonometry")]
pub use use_trigonometry;

#[cfg(feature = "use-crate")]
pub use use_crate;
#[cfg(feature = "use-rust")]
pub use use_rust;
#[cfg(feature = "use-version")]
pub use use_version;

/// Grouped access to the RustUse constants crates.
#[cfg(any(
    feature = "use-astronomical-constants",
    feature = "use-chemical-constants",
    feature = "use-computing-constants",
    feature = "use-earth-constants",
    feature = "use-constants",
    feature = "use-math-constants",
    feature = "use-physical-constants"
))]
pub mod constants {
    #[cfg(feature = "use-astronomical-constants")]
    pub use crate::use_astronomical_constants;
    #[cfg(feature = "use-astronomical-constants")]
    pub use crate::use_astronomical_constants as astronomical;

    #[cfg(feature = "use-chemical-constants")]
    pub use crate::use_chemical_constants;
    #[cfg(feature = "use-chemical-constants")]
    pub use crate::use_chemical_constants as chemical;

    #[cfg(feature = "use-computing-constants")]
    pub use crate::use_computing_constants;
    #[cfg(feature = "use-computing-constants")]
    pub use crate::use_computing_constants as computing;

    #[cfg(feature = "use-earth-constants")]
    pub use crate::use_earth_constants;
    #[cfg(feature = "use-earth-constants")]
    pub use crate::use_earth_constants as earth;

    #[cfg(feature = "use-constants")]
    pub use crate::use_constants;
    #[cfg(feature = "use-constants")]
    pub use crate::use_constants as facade;

    #[cfg(feature = "use-math-constants")]
    pub use crate::use_math_constants;
    #[cfg(feature = "use-math-constants")]
    pub use crate::use_math_constants as math;

    #[cfg(feature = "use-physical-constants")]
    pub use crate::use_physical_constants;
    #[cfg(feature = "use-physical-constants")]
    pub use crate::use_physical_constants as physical;
}

/// Grouped access to the RustUse chemistry crates.
#[cfg(any(
    feature = "use-atomic-mass",
    feature = "use-atomic-number",
    feature = "use-chemistry",
    feature = "use-electron-shell",
    feature = "use-element",
    feature = "use-periodic-table"
))]
pub mod chemistry {
    #[cfg(feature = "use-atomic-mass")]
    pub use crate::use_atomic_mass;
    #[cfg(feature = "use-atomic-mass")]
    pub use crate::use_atomic_mass as atomic_mass;

    #[cfg(feature = "use-atomic-number")]
    pub use crate::use_atomic_number;
    #[cfg(feature = "use-atomic-number")]
    pub use crate::use_atomic_number as atomic_number;

    #[cfg(feature = "use-chemistry")]
    pub use crate::use_chemistry;
    #[cfg(feature = "use-chemistry")]
    pub use crate::use_chemistry as facade;

    #[cfg(feature = "use-electron-shell")]
    pub use crate::use_electron_shell;
    #[cfg(feature = "use-electron-shell")]
    pub use crate::use_electron_shell as electron_shell;

    #[cfg(feature = "use-element")]
    pub use crate::use_element;
    #[cfg(feature = "use-element")]
    pub use crate::use_element as element;

    #[cfg(feature = "use-periodic-table")]
    pub use crate::use_periodic_table;
    #[cfg(feature = "use-periodic-table")]
    pub use crate::use_periodic_table as periodic_table;
}

/// Grouped access to the RustUse math crates.
#[cfg(any(
    feature = "use-algebra",
    feature = "use-calculus",
    feature = "use-catalan",
    feature = "use-combinatorics",
    feature = "use-complex",
    feature = "use-geometry",
    feature = "use-integer",
    feature = "use-linear",
    feature = "use-logic",
    feature = "use-math",
    feature = "use-number",
    feature = "use-probability",
    feature = "use-rational",
    feature = "use-real",
    feature = "use-series",
    feature = "use-set",
    feature = "use-statistics",
    feature = "use-trigonometry"
))]
pub mod math {
    #[cfg(feature = "use-algebra")]
    pub use crate::use_algebra;
    #[cfg(feature = "use-algebra")]
    pub use crate::use_algebra as algebra;

    #[cfg(feature = "use-calculus")]
    pub use crate::use_calculus;
    #[cfg(feature = "use-calculus")]
    pub use crate::use_calculus as calculus;

    #[cfg(feature = "use-catalan")]
    pub use crate::use_catalan;
    #[cfg(feature = "use-catalan")]
    pub use crate::use_catalan as catalan;

    #[cfg(feature = "use-combinatorics")]
    pub use crate::use_combinatorics;
    #[cfg(feature = "use-combinatorics")]
    pub use crate::use_combinatorics as combinatorics;

    #[cfg(feature = "use-complex")]
    pub use crate::use_complex;
    #[cfg(feature = "use-complex")]
    pub use crate::use_complex as complex;

    #[cfg(feature = "use-geometry")]
    pub use crate::use_geometry;
    #[cfg(feature = "use-geometry")]
    pub use crate::use_geometry as geometry;

    #[cfg(feature = "use-integer")]
    pub use crate::use_integer;
    #[cfg(feature = "use-integer")]
    pub use crate::use_integer as integer;

    #[cfg(feature = "use-linear")]
    pub use crate::use_linear;
    #[cfg(feature = "use-linear")]
    pub use crate::use_linear as linear;

    #[cfg(feature = "use-logic")]
    pub use crate::use_logic;
    #[cfg(feature = "use-logic")]
    pub use crate::use_logic as logic;

    #[cfg(feature = "use-math")]
    pub use crate::use_math;
    #[cfg(feature = "use-math")]
    pub use crate::use_math as facade;

    #[cfg(feature = "use-number")]
    pub use crate::use_number;
    #[cfg(feature = "use-number")]
    pub use crate::use_number as number;

    #[cfg(feature = "use-probability")]
    pub use crate::use_probability;
    #[cfg(feature = "use-probability")]
    pub use crate::use_probability as probability;

    #[cfg(feature = "use-rational")]
    pub use crate::use_rational;
    #[cfg(feature = "use-rational")]
    pub use crate::use_rational as rational;

    #[cfg(feature = "use-real")]
    pub use crate::use_real;
    #[cfg(feature = "use-real")]
    pub use crate::use_real as real;

    #[cfg(feature = "use-series")]
    pub use crate::use_series;
    #[cfg(feature = "use-series")]
    pub use crate::use_series as series;

    #[cfg(feature = "use-set")]
    pub use crate::use_set;
    #[cfg(feature = "use-set")]
    pub use crate::use_set as set;

    #[cfg(feature = "use-statistics")]
    pub use crate::use_statistics;
    #[cfg(feature = "use-statistics")]
    pub use crate::use_statistics as statistics;

    #[cfg(feature = "use-trigonometry")]
    pub use crate::use_trigonometry;
    #[cfg(feature = "use-trigonometry")]
    pub use crate::use_trigonometry as trigonometry;
}

/// Grouped access to the RustUse Rust ecosystem crates.
#[cfg(any(feature = "use-crate", feature = "use-rust", feature = "use-version"))]
pub mod rust {
    #[cfg(feature = "use-crate")]
    pub use crate::use_crate;
    #[cfg(feature = "use-crate")]
    pub use crate::use_crate as crate_tools;

    #[cfg(feature = "use-rust")]
    pub use crate::use_rust;
    #[cfg(feature = "use-rust")]
    pub use crate::use_rust as facade;

    #[cfg(feature = "use-version")]
    pub use crate::use_version;
    #[cfg(feature = "use-version")]
    pub use crate::use_version as version;
}

/// Commonly used facade items from set crates that already define a prelude.
pub mod prelude {
    #[cfg(feature = "use-chemistry")]
    pub use use_chemistry::prelude::*;
    #[cfg(feature = "use-math")]
    pub use use_math::prelude::*;
    #[cfg(feature = "use-rust")]
    pub use use_rust::prelude::*;
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "use-element")]
    use crate::chemistry::element::element_by_symbol;
    #[cfg(feature = "use-earth-constants")]
    use crate::constants::earth;
    #[cfg(feature = "use-math-constants")]
    use crate::constants::math;
    #[cfg(feature = "use-combinatorics")]
    use crate::math::combinatorics::factorial;
    #[cfg(feature = "use-version")]
    use crate::rust::version::parse_version;

    fn runtime(value: f64) -> f64 {
        value
    }

    #[test]
    fn set_modules_expose_child_crates() {
        #[cfg(feature = "use-earth-constants")]
        assert!(runtime(earth::EARTH_RADIUS_MEAN) > 0.0);

        #[cfg(feature = "use-math-constants")]
        assert_eq!(math::TAU, 2.0 * math::PI);

        #[cfg(feature = "use-combinatorics")]
        assert_eq!(factorial(5), Ok(120));

        #[cfg(feature = "use-version")]
        {
            let version = parse_version("0.1.0").expect("version should parse");
            assert_eq!(version.to_string(), "0.1.0");
        }

        #[cfg(feature = "use-element")]
        {
            let oxygen = element_by_symbol("O").expect("oxygen should exist");
            assert_eq!(oxygen.atomic_number, 8);
        }
    }
}
