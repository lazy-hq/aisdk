# Contributing

We welcome contributions! To ensure smooth collaboration, please follow these guidelines:

1. **Open an Issue First**
   Before starting work on a pull request, please open an issue to discuss your idea or bug fix.

   * This helps us confirm the change is needed, avoids duplicate work, and ensures no one else is already working on it.
   * Once the issue is agreed upon, feel free to proceed with your contribution.

2. **Report Issues**
   Use GitHub issues to report bugs, request features, or ask questions.

3. **Submit PRs**
   - Fork the repository, make your changes
   - Write change log to describe your changes in CHANGELOG.md following the [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format.
   - Open a pull request **against the `dev` branch**, not `main` (see [Branches and Releases](#branches-and-releases)). Please reference the related issue in your PR description.

4. **Code Style**
   Follow Rust conventions. Before committing, run:

   ```bash
   cargo fmt --all
   cargo clippy --all-features
   ```

5. **Tests**
   Add or update tests for any new features or bug fixes. Always run the full test suite with all features enabled:

   ```bash
   cargo test --all-features
   ```

6. **Commits**
   Use clear and descriptive commit messages that explain the intent of your changes.

## Branches and Releases

**Rule: `main` only contains released code. Every merge into `main` is a release.**

| Branch | Purpose |
|---|---|
| `dev` | Development. All feature and fix PRs target `dev`. |
| `main` | Released code only. Always equals the latest version on crates.io. |
| `release-x.y.z` | Created by `./release.sh` from `dev` to release it into `main`. |
| `auto/update-models` | Created by the daily model-update job. Rewritten every day; do not push to it. |

### How a release happens

1. From `dev`, run `./release.sh patch|minor|major`. It branches `release-x.y.z` off `origin/dev`, bumps the version, dates the CHANGELOG and opens a PR into `main`.
   - `release.sh` only bumps `aisdk`. If `aisdk-macros` changed since its last release, bump its version in `macros/Cargo.toml` and in `aisdk`'s dependency on `dev` first. The release then publishes both crates.
2. A maintainer reviews and merges it.
3. The merge releases automatically: `tag-release.yml` tags `vx.y.z`, which triggers `release.yml` to publish to crates.io and create the GitHub release. It then opens a PR merging `main` back into `dev`. Merge that PR with a merge commit (not squash).

Never push tags by hand; they are created by the workflow.

### Automated model updates

Every day `update-models.yml` syncs model capabilities from [models.dev](https://models.dev) (see [scripts/README.md](./scripts/README.md)). If anything changed, it bumps the **patch** version and opens or refreshes one PR from `auto/update-models` into `main`. Updates only ever add models and capabilities, so they are always patch-compatible. Merging the PR releases it like any other release, and the sync PR brings the change into `dev`.

### What enforces the rule

PRs into `main` must pass `release-guard.yml`:

- **release rules**: the PR comes from `dev`, `release-*` or `auto/update-models` (not a fork), the version in `Cargo.toml` is higher than the latest `v*` tag, and `CHANGELOG.md` has a `## [x.y.z]` section for it.
- **semver checks**: `cargo-semver-checks` confirms the version bump matches the API changes (for example, a patch release must not break anything).

If `main` ever changes without a version bump, `tag-release.yml` fails instead of publishing.

For any questions, feel free to open an issue or continue the discussion in your pull request.
