# Releasing

RustUse/rustuse releases are cut from the canonical GitHub repository.

## Release flow

1. Ensure CI, publish-readiness, and security workflows are green on `main`.
2. Let `release-plz` open or update the release PR.
3. Review the generated version bump and release notes for accuracy.
4. Merge the release PR from the canonical repository.
5. Let the release workflow publish the crate from GitHub.

## Pre-release checks

Run the local readiness path before merging release-affecting changes:

```sh
cargo xcheck
cargo xexamples
cargo xtest-minimal
cargo publish --dry-run --allow-dirty -p rustuse
```

The publish dry-run is intentionally crates.io-gated. It is only expected to succeed after every
staged child crate wired into `rustuse` is already visible on crates.io.

## Notes

- `rustuse` is a facade crate, so releases should stay aligned with the
  currently published child crates and feature wiring.
- Mirror CI is validation only. Tags, GitHub releases, and crates.io publishes
  are created from the canonical GitHub repository.
- `rustuse` `0.1.0` has already been published, so follow-up releases should
  use the checked-in `release-plz` automation rather than a first-publish
  bootstrap path.
- Published crates.io versions are permanent. Verify crate metadata, README
  examples, and changelog entries before any real publish.
- The current `main` branch wires in staged `use-net`, `use-web`, and
  `use-pattern` surfaces. Until those focused crates and facades are live on
  crates.io, the publish-readiness dry-run is expected to remain blocked.
