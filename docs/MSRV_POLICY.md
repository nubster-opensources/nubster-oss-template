# Minimum Supported Rust Version (MSRV) policy

{{project-name}} targets Rust **{{msrv}}** as its Minimum Supported Rust Version.

This means every release is guaranteed to compile on stable Rust {{msrv}} without any nightly or unstable features.

## How the MSRV evolves

- {{project-name}} does not commit to supporting Rust versions older than {{msrv}}.
- An MSRV bump is treated as a **minor** version bump per the [semver policy](SEMVER_POLICY.md). For example, raising the MSRV from {{msrv}} to a newer version ships in a `0.X.0` release (or `X.0.0` once at 1.0).
- The current MSRV is documented in `CHANGELOG.md` under the `Changed` section of the release that bumps it.

## Why we pick the floor we pick

- **{{msrv}}** is required because {{project-name}} uses Rust edition 2024.
- Future bumps will be driven by concrete features the project needs, not by chasing the latest stable.

## How we verify the MSRV in CI

The `msrv-check` job in `.github/workflows/ci.yml` runs:

```sh
cargo +{{msrv}} check --workspace --all-features
```

This job is part of the required status checks and blocks merging if it fails.

## Downstream impact

If you depend on {{project-name}} and are pinned to an older Rust toolchain, an MSRV bump is a breaking change for your build pipeline. We announce MSRV bumps in the CHANGELOG and treat them as minor version bumps to give downstream users time to upgrade.
