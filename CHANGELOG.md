# Changelog

## Unreleased

## [0.1.4](https://github.com/RustUse/rustuse/compare/v0.1.3...v0.1.4) - 2026-05-23

### Added

- Added grouped facade modules and top-level reexports for `use-astronomy`, `use-bioinformatics`,
  `use-biology`, `use-cli`, `use-config`, `use-diagnostic`, `use-ecology`, `use-electronics`,
  `use-geography`, `use-geology`, `use-locale`, `use-meteorology`, `use-os`, `use-presence`,
  `use-quant`, and `use-robotics`.
- Added `use-isotope` to the chemistry feature group and grouped chemistry reexports.

### Changed

- Expanded the default `full` feature to cover all repository-level RustUse facade crates currently
  published on crates.io.
- Aligned the `use-geometry` and `use-text` facade dependency pins with their current published
  local versions.

### Fixed

- Aligned local `use-linear` and `use-math` path dependency pins with sibling `0.0.7` packages.

## [0.1.3](https://github.com/RustUse/rustuse/compare/v0.1.2...v0.1.3) - 2026-05-18

### Changed

- Add constants feature and facade reexports
- Add net, pattern, and web feature groups

### Added

- Added set-level facade groups and top-level reexports for the published `use-accessibility`,
  `use-acoustics`, `use-calendar`, `use-color`, `use-control`, `use-data`, `use-encoding`,
  `use-fs`, `use-graph`, `use-id`, `use-materials`, `use-measure`, `use-media`, `use-optics`,
  `use-optimization`, `use-physics`, `use-rustacean`, `use-signal`, `use-simulation`,
  `use-stats`, `use-text`, `use-time`, `use-typography`, `use-units`, `use-validate`, and
  `use-wave` facade crates.
- Added a first-class `geometry` set module while keeping the existing `rustuse::math::geometry`
  compatibility path.
- Added a `pattern` feature group and facade reexports for `use-pattern` plus its focused child
  crates on `main`.

### Changed

- Expanded the default `full` feature to cover every repository-level RustUse facade crate that is
  currently published on crates.io.
- Switched facade-only set dependencies to crates.io version pins so the published facade surface
  can coexist with local sibling workspaces that reuse package names.
- Updated the rustuse release notes and publish-readiness flow to treat the staged
  `use-pattern` surface like the existing crates.io-gated facade waves.

## [0.1.2](https://github.com/RustUse/rustuse/compare/v0.1.1...v0.1.2) - 2026-05-15

### Added

- Added a `constants` feature group and facade reexports for `use-constants` plus its focused
  child crates on `main`.
- Added `use-earth-constants` to the constants facade surface on `main`.

### Changed

- Updated the release documentation to reflect the published `use-constants` facade surface.

## [0.1.1](https://github.com/RustUse/rustuse/compare/v0.1.0...v0.1.1) - 2026-05-13

### Changed

- Add CI, tooling configs and project metadata

### Added

- Added a `chemistry` feature group and reexports for `use-chemistry` plus its published child
  crates.

## 0.1.0 - 2026-05-12

### Added

- Initial public `rustuse` release with the published math and rust facade surface.
