#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Thin facade re-exports for the available RustUse crate sets.

#[cfg(feature = "use-accessibility")]
pub use use_accessibility;
#[cfg(feature = "use-acoustics")]
pub use use_acoustics;
#[cfg(feature = "use-astronomical-constants")]
pub use use_astronomical_constants;
#[cfg(feature = "use-astronomy")]
pub use use_astronomy;
#[cfg(feature = "use-bioinformatics")]
pub use use_bioinformatics;
#[cfg(feature = "use-biology")]
pub use use_biology;
#[cfg(feature = "use-calendar")]
pub use use_calendar;
#[cfg(feature = "use-chemical-constants")]
pub use use_chemical_constants;
#[cfg(feature = "use-cli")]
pub use use_cli;
#[cfg(feature = "use-color")]
pub use use_color;
#[cfg(feature = "use-computing-constants")]
pub use use_computing_constants;
#[cfg(feature = "use-config")]
pub use use_config;
#[cfg(feature = "use-constants")]
pub use use_constants;
#[cfg(feature = "use-control")]
pub use use_control;
#[cfg(feature = "use-data")]
pub use use_data;
#[cfg(feature = "use-diagnostic")]
pub use use_diagnostic;
#[cfg(feature = "use-earth-constants")]
pub use use_earth_constants;
#[cfg(feature = "use-ecology")]
pub use use_ecology;
#[cfg(feature = "use-electronics")]
pub use use_electronics;
#[cfg(feature = "use-encoding")]
pub use use_encoding;
#[cfg(feature = "use-fs")]
pub use use_fs;
#[cfg(feature = "use-geography")]
pub use use_geography;
#[cfg(feature = "use-geology")]
pub use use_geology;
#[cfg(feature = "use-graph")]
pub use use_graph;
#[cfg(feature = "use-id")]
pub use use_id;
#[cfg(feature = "use-locale")]
pub use use_locale;
#[cfg(feature = "use-materials")]
pub use use_materials;
#[cfg(feature = "use-math-constants")]
pub use use_math_constants;
#[cfg(feature = "use-measure")]
pub use use_measure;
#[cfg(feature = "use-media")]
pub use use_media;
#[cfg(feature = "use-meteorology")]
pub use use_meteorology;
#[cfg(feature = "use-optics")]
pub use use_optics;
#[cfg(feature = "use-optimization")]
pub use use_optimization;
#[cfg(feature = "use-os")]
pub use use_os;
#[cfg(feature = "use-physical-constants")]
pub use use_physical_constants;
#[cfg(feature = "use-physics")]
pub use use_physics;
#[cfg(feature = "use-presence")]
pub use use_presence;
#[cfg(feature = "use-quant")]
pub use use_quant;
#[cfg(feature = "use-robotics")]
pub use use_robotics;
#[cfg(feature = "use-rustacean")]
pub use use_rustacean;
#[cfg(feature = "use-signal")]
pub use use_signal;
#[cfg(feature = "use-simulation")]
pub use use_simulation;
#[cfg(feature = "use-stats")]
pub use use_stats;
#[cfg(feature = "use-text")]
pub use use_text;
#[cfg(feature = "use-time")]
pub use use_time;
#[cfg(feature = "use-typography")]
pub use use_typography;
#[cfg(feature = "use-units")]
pub use use_units;
#[cfg(feature = "use-validate")]
pub use use_validate;
#[cfg(feature = "use-wave")]
pub use use_wave;

#[cfg(feature = "use-cookie")]
pub use use_cookie;
#[cfg(feature = "use-css")]
pub use use_css;
#[cfg(feature = "use-header")]
pub use use_header;
#[cfg(feature = "use-html")]
pub use use_html;
#[cfg(feature = "use-http")]
pub use use_http;
#[cfg(feature = "use-method")]
pub use use_method;
#[cfg(feature = "use-mime")]
pub use use_mime;
#[cfg(feature = "use-origin")]
pub use use_origin;
#[cfg(feature = "use-query")]
pub use use_query;
#[cfg(feature = "use-route")]
pub use use_route;
#[cfg(feature = "use-status")]
pub use use_status;
#[cfg(feature = "use-uri")]
pub use use_uri;
#[cfg(feature = "use-url")]
pub use use_url;
#[cfg(feature = "use-web")]
pub use use_web;

#[cfg(feature = "use-cidr")]
pub use use_cidr;
#[cfg(feature = "use-dns")]
pub use use_dns;
#[cfg(feature = "use-domain")]
pub use use_domain;
#[cfg(feature = "use-host")]
pub use use_host;
#[cfg(feature = "use-ip")]
pub use use_ip;
#[cfg(feature = "use-mac")]
pub use use_mac;
#[cfg(feature = "use-net")]
pub use use_net;
#[cfg(feature = "use-port")]
pub use use_port;
#[cfg(feature = "use-socket")]
pub use use_socket;
#[cfg(feature = "use-tcp")]
pub use use_tcp;
#[cfg(feature = "use-udp")]
pub use use_udp;

#[cfg(feature = "use-glob")]
pub use use_glob;
#[cfg(feature = "use-match")]
pub use use_match;
#[cfg(feature = "use-pattern")]
pub use use_pattern;
#[cfg(feature = "use-regex")]
pub use use_regex;
#[cfg(feature = "use-wildcard")]
pub use use_wildcard;

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
#[cfg(feature = "use-isotope")]
pub use use_isotope;
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

/// Grouped access to the RustUse accessibility crates.
#[cfg(feature = "use-accessibility")]
pub mod accessibility {
    pub use crate::use_accessibility;
    pub use crate::use_accessibility as facade;
}

/// Grouped access to the RustUse acoustics crates.
#[cfg(feature = "use-acoustics")]
pub mod acoustics {
    pub use crate::use_acoustics;
    pub use crate::use_acoustics as facade;
}

/// Grouped access to the RustUse astronomy crates.
#[cfg(feature = "use-astronomy")]
pub mod astronomy {
    pub use crate::use_astronomy;
    pub use crate::use_astronomy as facade;
}

/// Grouped access to the RustUse bioinformatics crates.
#[cfg(feature = "use-bioinformatics")]
pub mod bioinformatics {
    pub use crate::use_bioinformatics;
    pub use crate::use_bioinformatics as facade;
}

/// Grouped access to the RustUse biology crates.
#[cfg(feature = "use-biology")]
pub mod biology {
    pub use crate::use_biology;
    pub use crate::use_biology as facade;
}

/// Grouped access to the RustUse calendar crates.
#[cfg(feature = "use-calendar")]
pub mod calendar {
    pub use crate::use_calendar;
    pub use crate::use_calendar as facade;
}

/// Grouped access to the RustUse chemistry crates.
#[cfg(any(
    feature = "use-atomic-mass",
    feature = "use-atomic-number",
    feature = "use-chemistry",
    feature = "use-electron-shell",
    feature = "use-element",
    feature = "use-isotope",
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

    #[cfg(feature = "use-isotope")]
    pub use crate::use_isotope;
    #[cfg(feature = "use-isotope")]
    pub use crate::use_isotope as isotope;

    #[cfg(feature = "use-periodic-table")]
    pub use crate::use_periodic_table;
    #[cfg(feature = "use-periodic-table")]
    pub use crate::use_periodic_table as periodic_table;
}

/// Grouped access to the RustUse CLI crates.
#[cfg(feature = "use-cli")]
pub mod cli {
    pub use crate::use_cli;
    pub use crate::use_cli as facade;
}

/// Grouped access to the RustUse color crates.
#[cfg(feature = "use-color")]
pub mod color {
    pub use crate::use_color;
    pub use crate::use_color as facade;
}

/// Grouped access to the RustUse configuration crates.
#[cfg(feature = "use-config")]
pub mod config {
    pub use crate::use_config;
    pub use crate::use_config as facade;
}

/// Grouped access to the RustUse control crates.
#[cfg(feature = "use-control")]
pub mod control {
    pub use crate::use_control;
    pub use crate::use_control as facade;
}

/// Grouped access to the RustUse data crates.
#[cfg(feature = "use-data")]
pub mod data {
    pub use crate::use_data;
    pub use crate::use_data as facade;
}

/// Grouped access to the RustUse diagnostic crates.
#[cfg(feature = "use-diagnostic")]
pub mod diagnostic {
    pub use crate::use_diagnostic;
    pub use crate::use_diagnostic as facade;
}

/// Grouped access to the RustUse ecology crates.
#[cfg(feature = "use-ecology")]
pub mod ecology {
    pub use crate::use_ecology;
    pub use crate::use_ecology as facade;
}

/// Grouped access to the RustUse electronics crates.
#[cfg(feature = "use-electronics")]
pub mod electronics {
    pub use crate::use_electronics;
    pub use crate::use_electronics as facade;
}

/// Grouped access to the RustUse encoding crates.
#[cfg(feature = "use-encoding")]
pub mod encoding {
    pub use crate::use_encoding;
    pub use crate::use_encoding as facade;
}

/// Grouped access to the RustUse filesystem crates.
#[cfg(feature = "use-fs")]
pub mod fs {
    pub use crate::use_fs;
    pub use crate::use_fs as facade;
}

/// Grouped access to the RustUse geography crates.
#[cfg(feature = "use-geography")]
pub mod geography {
    pub use crate::use_geography;
    pub use crate::use_geography as facade;
}

/// Grouped access to the RustUse geology crates.
#[cfg(feature = "use-geology")]
pub mod geology {
    pub use crate::use_geology;
    pub use crate::use_geology as facade;
}

/// Grouped access to the RustUse geometry crates.
#[cfg(feature = "use-geometry")]
pub mod geometry {
    pub use crate::use_geometry;
    pub use crate::use_geometry as facade;
}

/// Grouped access to the RustUse graph crates.
#[cfg(feature = "use-graph")]
pub mod graph {
    pub use crate::use_graph;
    pub use crate::use_graph as facade;
}

/// Grouped access to the RustUse identity crates.
#[cfg(feature = "use-id")]
pub mod id {
    pub use crate::use_id;
    pub use crate::use_id as facade;
}

/// Grouped access to the RustUse locale crates.
#[cfg(feature = "use-locale")]
pub mod locale {
    pub use crate::use_locale;
    pub use crate::use_locale as facade;
}

/// Grouped access to the RustUse materials crates.
#[cfg(feature = "use-materials")]
pub mod materials {
    pub use crate::use_materials;
    pub use crate::use_materials as facade;
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

/// Grouped access to the RustUse measurement crates.
#[cfg(feature = "use-measure")]
pub mod measure {
    pub use crate::use_measure;
    pub use crate::use_measure as facade;
}

/// Grouped access to the RustUse media crates.
#[cfg(feature = "use-media")]
pub mod media {
    pub use crate::use_media;
    pub use crate::use_media as facade;
}

/// Grouped access to the RustUse meteorology crates.
#[cfg(feature = "use-meteorology")]
pub mod meteorology {
    pub use crate::use_meteorology;
    pub use crate::use_meteorology as facade;
}

/// Grouped access to the RustUse networking crates.
#[cfg(any(
    feature = "use-cidr",
    feature = "use-dns",
    feature = "use-domain",
    feature = "use-host",
    feature = "use-ip",
    feature = "use-mac",
    feature = "use-net",
    feature = "use-port",
    feature = "use-socket",
    feature = "use-tcp",
    feature = "use-udp"
))]
pub mod net {
    #[cfg(feature = "use-cidr")]
    pub use crate::use_cidr;
    #[cfg(feature = "use-cidr")]
    pub use crate::use_cidr as cidr;

    #[cfg(feature = "use-dns")]
    pub use crate::use_dns;
    #[cfg(feature = "use-dns")]
    pub use crate::use_dns as dns;

    #[cfg(feature = "use-domain")]
    pub use crate::use_domain;
    #[cfg(feature = "use-domain")]
    pub use crate::use_domain as domain;

    #[cfg(feature = "use-host")]
    pub use crate::use_host;
    #[cfg(feature = "use-host")]
    pub use crate::use_host as host;

    #[cfg(feature = "use-ip")]
    pub use crate::use_ip;
    #[cfg(feature = "use-ip")]
    pub use crate::use_ip as ip;

    #[cfg(feature = "use-mac")]
    pub use crate::use_mac;
    #[cfg(feature = "use-mac")]
    pub use crate::use_mac as mac;

    #[cfg(feature = "use-net")]
    pub use crate::use_net;
    #[cfg(feature = "use-net")]
    pub use crate::use_net as facade;

    #[cfg(feature = "use-port")]
    pub use crate::use_port;
    #[cfg(feature = "use-port")]
    pub use crate::use_port as port;

    #[cfg(feature = "use-socket")]
    pub use crate::use_socket;
    #[cfg(feature = "use-socket")]
    pub use crate::use_socket as socket;

    #[cfg(feature = "use-tcp")]
    pub use crate::use_tcp;
    #[cfg(feature = "use-tcp")]
    pub use crate::use_tcp as tcp;

    #[cfg(feature = "use-udp")]
    pub use crate::use_udp;
    #[cfg(feature = "use-udp")]
    pub use crate::use_udp as udp;
}

/// Grouped access to the RustUse optics crates.
#[cfg(feature = "use-optics")]
pub mod optics {
    pub use crate::use_optics;
    pub use crate::use_optics as facade;
}

/// Grouped access to the RustUse optimization crates.
#[cfg(feature = "use-optimization")]
pub mod optimization {
    pub use crate::use_optimization;
    pub use crate::use_optimization as facade;
}

/// Grouped access to the RustUse operating system crates.
#[cfg(feature = "use-os")]
pub mod os {
    pub use crate::use_os;
    pub use crate::use_os as facade;
}

/// Grouped access to the RustUse pattern crates.
#[cfg(any(
    feature = "use-glob",
    feature = "use-match",
    feature = "use-pattern",
    feature = "use-regex",
    feature = "use-wildcard"
))]
pub mod pattern {
    #[cfg(feature = "use-glob")]
    pub use crate::use_glob;
    #[cfg(feature = "use-glob")]
    pub use crate::use_glob as glob;

    #[cfg(feature = "use-match")]
    pub use crate::use_match;
    #[cfg(feature = "use-match")]
    pub use crate::use_match as matchers;

    #[cfg(feature = "use-pattern")]
    pub use crate::use_pattern;
    #[cfg(feature = "use-pattern")]
    pub use crate::use_pattern as facade;

    #[cfg(feature = "use-regex")]
    pub use crate::use_regex;
    #[cfg(feature = "use-regex")]
    pub use crate::use_regex as regex;

    #[cfg(feature = "use-wildcard")]
    pub use crate::use_wildcard;
    #[cfg(feature = "use-wildcard")]
    pub use crate::use_wildcard as wildcard;
}

/// Grouped access to the RustUse physics crates.
#[cfg(feature = "use-physics")]
pub mod physics {
    pub use crate::use_physics;
    pub use crate::use_physics as facade;
}

/// Grouped access to the RustUse presence crates.
#[cfg(feature = "use-presence")]
pub mod presence {
    pub use crate::use_presence;
    pub use crate::use_presence as facade;
}

/// Grouped access to the RustUse quantitative finance crates.
#[cfg(feature = "use-quant")]
pub mod quant {
    pub use crate::use_quant;
    pub use crate::use_quant as facade;
}

/// Grouped access to the RustUse robotics crates.
#[cfg(feature = "use-robotics")]
pub mod robotics {
    pub use crate::use_robotics;
    pub use crate::use_robotics as facade;
}

/// Grouped access to the RustUse web crates.
#[cfg(any(
    feature = "use-cookie",
    feature = "use-css",
    feature = "use-header",
    feature = "use-html",
    feature = "use-http",
    feature = "use-method",
    feature = "use-mime",
    feature = "use-origin",
    feature = "use-query",
    feature = "use-route",
    feature = "use-status",
    feature = "use-uri",
    feature = "use-url",
    feature = "use-web"
))]
pub mod web {
    #[cfg(feature = "use-cookie")]
    pub use crate::use_cookie;
    #[cfg(feature = "use-cookie")]
    pub use crate::use_cookie as cookie;

    #[cfg(feature = "use-css")]
    pub use crate::use_css;
    #[cfg(feature = "use-css")]
    pub use crate::use_css as css;

    #[cfg(feature = "use-header")]
    pub use crate::use_header;
    #[cfg(feature = "use-header")]
    pub use crate::use_header as header;

    #[cfg(feature = "use-html")]
    pub use crate::use_html;
    #[cfg(feature = "use-html")]
    pub use crate::use_html as html;

    #[cfg(feature = "use-http")]
    pub use crate::use_http;
    #[cfg(feature = "use-http")]
    pub use crate::use_http as http;

    #[cfg(feature = "use-method")]
    pub use crate::use_method;
    #[cfg(feature = "use-method")]
    pub use crate::use_method as method;

    #[cfg(feature = "use-mime")]
    pub use crate::use_mime;
    #[cfg(feature = "use-mime")]
    pub use crate::use_mime as mime;

    #[cfg(feature = "use-origin")]
    pub use crate::use_origin;
    #[cfg(feature = "use-origin")]
    pub use crate::use_origin as origin;

    #[cfg(feature = "use-query")]
    pub use crate::use_query;
    #[cfg(feature = "use-query")]
    pub use crate::use_query as query;

    #[cfg(feature = "use-route")]
    pub use crate::use_route;
    #[cfg(feature = "use-route")]
    pub use crate::use_route as route;

    #[cfg(feature = "use-status")]
    pub use crate::use_status;
    #[cfg(feature = "use-status")]
    pub use crate::use_status as status;

    #[cfg(feature = "use-uri")]
    pub use crate::use_uri;
    #[cfg(feature = "use-uri")]
    pub use crate::use_uri as uri;

    #[cfg(feature = "use-url")]
    pub use crate::use_url;
    #[cfg(feature = "use-url")]
    pub use crate::use_url as url;

    #[cfg(feature = "use-web")]
    pub use crate::use_web;
    #[cfg(feature = "use-web")]
    pub use crate::use_web as facade;
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

/// Grouped access to the RustUse Rustacean crates.
#[cfg(feature = "use-rustacean")]
pub mod rustacean {
    pub use crate::use_rustacean;
    pub use crate::use_rustacean as facade;
}

/// Grouped access to the RustUse signal crates.
#[cfg(feature = "use-signal")]
pub mod signal {
    pub use crate::use_signal;
    pub use crate::use_signal as facade;
}

/// Grouped access to the RustUse simulation crates.
#[cfg(feature = "use-simulation")]
pub mod simulation {
    pub use crate::use_simulation;
    pub use crate::use_simulation as facade;
}

/// Grouped access to the RustUse statistics crates.
#[cfg(feature = "use-stats")]
pub mod stats {
    pub use crate::use_stats;
    pub use crate::use_stats as facade;
}

/// Grouped access to the RustUse text crates.
#[cfg(feature = "use-text")]
pub mod text {
    pub use crate::use_text;
    pub use crate::use_text as facade;
}

/// Grouped access to the RustUse time crates.
#[cfg(feature = "use-time")]
pub mod time {
    pub use crate::use_time;
    pub use crate::use_time as facade;
}

/// Grouped access to the RustUse typography crates.
#[cfg(feature = "use-typography")]
pub mod typography {
    pub use crate::use_typography;
    pub use crate::use_typography as facade;
}

/// Grouped access to the RustUse units crates.
#[cfg(feature = "use-units")]
pub mod units {
    pub use crate::use_units;
    pub use crate::use_units as facade;
}

/// Grouped access to the RustUse validation crates.
#[cfg(feature = "use-validate")]
pub mod validate {
    pub use crate::use_validate;
    pub use crate::use_validate as facade;
}

/// Grouped access to the RustUse wave crates.
#[cfg(feature = "use-wave")]
pub mod wave {
    pub use crate::use_wave;
    pub use crate::use_wave as facade;
}

/// Commonly used facade items from set crates that already define a prelude.
pub mod prelude {
    #[cfg(feature = "use-chemistry")]
    pub use use_chemistry::prelude::*;
    #[cfg(feature = "use-math")]
    pub use use_math::prelude::*;
    #[cfg(feature = "use-pattern")]
    pub use use_pattern::prelude::*;
    #[cfg(feature = "use-rust")]
    pub use use_rust::prelude::*;
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "use-element")]
    use crate::chemistry::element::element_by_symbol;
    #[cfg(feature = "use-isotope")]
    use crate::chemistry::isotope::isotope_by_symbol;
    #[cfg(feature = "use-color")]
    use crate::color::facade as color;
    #[cfg(feature = "use-earth-constants")]
    use crate::constants::earth;
    #[cfg(feature = "use-math-constants")]
    use crate::constants::math;
    #[cfg(feature = "use-combinatorics")]
    use crate::math::combinatorics::factorial;
    #[cfg(feature = "use-ip")]
    use crate::net::ip;
    #[cfg(feature = "use-socket")]
    use crate::net::socket;
    #[cfg(feature = "use-match")]
    use crate::pattern::matchers::{MatchSpan, slice_match};
    #[cfg(feature = "use-regex")]
    use crate::pattern::regex::count_regex_matches;
    #[cfg(feature = "use-version")]
    use crate::rust::version::parse_version;
    #[cfg(feature = "use-time")]
    use crate::time::facade as time;
    #[cfg(feature = "use-pattern")]
    use crate::use_pattern::prelude::MatchSpan as FacadeMatchSpan;
    #[cfg(feature = "use-wave")]
    use crate::wave::facade as wave;
    #[cfg(feature = "use-html")]
    use crate::web::html::extract_title;
    #[cfg(feature = "use-status")]
    use crate::web::status::reason_phrase;
    #[cfg(feature = "use-url")]
    use crate::web::url::parse_url_basic;

    #[cfg(any(feature = "use-earth-constants", feature = "use-math-constants"))]
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

        #[cfg(feature = "use-isotope")]
        {
            let carbon_14 = isotope_by_symbol("C", 14).expect("carbon-14 should construct");
            assert_eq!(carbon_14.neutron_count(), 8);
        }

        #[cfg(feature = "use-ip")]
        assert!(ip::is_ipv4("127.0.0.1"));

        #[cfg(feature = "use-socket")]
        {
            let endpoint = socket::parse_socket_endpoint("localhost:8080")
                .expect("socket endpoint should parse");
            assert_eq!(endpoint.port, 8080);
        }

        #[cfg(feature = "use-match")]
        {
            let span = MatchSpan { start: 0, end: 4 };
            assert_eq!(slice_match("rustacean", &span), Some("rust"));
        }

        #[cfg(feature = "use-pattern")]
        assert_eq!(FacadeMatchSpan { start: 0, end: 1 }.end, 1);

        #[cfg(feature = "use-regex")]
        assert_eq!(count_regex_matches(r"\d+", "v1 v20"), Some(2));

        #[cfg(feature = "use-color")]
        assert_eq!(
            color::parse_hex_rgb("#3366CC")
                .expect("color should parse")
                .to_hex_rgb(),
            "#3366CC"
        );

        #[cfg(feature = "use-time")]
        assert_eq!(time::minutes_to_seconds(5), Some(300));

        #[cfg(feature = "use-wave")]
        assert_eq!(
            wave::period_seconds(4.0).expect("period should compute"),
            0.25
        );

        #[cfg(feature = "use-url")]
        {
            let parts = parse_url_basic("https://example.com/docs").expect("url should parse");
            assert_eq!(parts.host.as_deref(), Some("example.com"));
        }

        #[cfg(feature = "use-status")]
        assert_eq!(reason_phrase(404), Some("Not Found"));

        #[cfg(feature = "use-html")]
        assert_eq!(
            extract_title("<title>Docs</title>").as_deref(),
            Some("Docs")
        );
    }
}
