# Dying-Earth

## Agent skills

### Issue tracker

Issues live as GitHub issues in `whaleyjoshua2/Dying-Earth`, managed with the `gh` CLI. See `docs/agents/issue-tracker.md`.

### Triage labels

The five canonical triage roles, using their default label strings. See `docs/agents/triage-labels.md`.

### Domain docs

Single-context: one `CONTEXT.md` and `docs/adr/` at the repo root. See `docs/agents/domain.md`.

## Formatting

**This tree is formatted wide, by hand. Do not run `rustfmt` or `cargo fmt`** — there is no
`rustfmt.toml`, so they rewrite it to the narrow default (4,370 lines of `src/ui.rs` in one pass,
measured 2026-09-14). Match the surrounding code by hand.

## The linter gate

**The clippy gate is `cargo clippy --workspace --release --all-targets -- -D warnings`.** The
`--workspace` is load-bearing: this repo is a two-member workspace (`.` and `engine`), and without
it cargo checks only the root package, so the engine's own code and its test target go unchecked and
the command reports success with errors sitting in `engine/tests/formulas.rs`. Measured 2026-09-23,
when three `cloned_ref_to_slice_refs` errors had survived four commits behind a green
`cargo clippy --release --all-targets`. The trailing `-- -D warnings` is load-bearing too: without
it clippy prints its findings and exits zero.
