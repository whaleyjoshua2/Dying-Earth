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
