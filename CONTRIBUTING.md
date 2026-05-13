# Contributing

RustUse/rustuse is a facade crate. Contributions should keep the public surface
focused on feature gating, re-exports, grouping modules, and release alignment
with the published child crates.

## Development flow

1. Make the smallest useful change.
2. Add or update tests for public behavior changes.
3. Keep feature sets and re-exports aligned with what is actually published.
4. Avoid introducing new domain logic here when it belongs in a child crate.

## Local validation

```sh
cargo fmt --all --check
cargo check --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## Scope guidance

- `rustuse` is the unified facade for published RustUse sets, not a replacement
  for the child crates.
- Prefer explicit feature wiring and stable re-exports over broad abstraction
  layers.
- Keep README examples aligned with the currently published first-wave crates.
- Do not widen default feature scope or publish sequencing without maintainer
  approval.
