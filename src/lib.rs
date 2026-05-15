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
    #[cfg(feature = "use-pattern")]
    use crate::use_pattern::prelude::MatchSpan as FacadeMatchSpan;
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
