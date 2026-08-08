# AGENTS.md

Kleio CLI is a Rust command-line application for local Kleio authoring workflows.

## Scope and precedence

* These instructions apply to the entire `kleio-cli` crate.
* Broader Urania repository instructions still apply when this crate is edited as part of the workspace.
* Explicit task instructions take precedence over this file.

## File size and module boundaries

* Keep `src/main.rs` focused on process entry, module wiring, and top-level imports.
* Avoid adding substantial command handling, formatting, redaction, tree rendering, or authoring helpers directly to `main.rs`.
* Treat files approaching roughly 800 lines as a refactoring signal.
* Split files before they become difficult for LLM context windows, editor navigation, or review.
* Prefer responsibility-based modules such as CLI argument definitions, command dispatch, authoring helpers, listing output, tree output, redaction, reports, and errors.
* Do not split a cohesive helper solely to hit a line count, but avoid adding a second major concern to an already-large file.
* Keep generated, vendored, or data-table-like files exempt from the soft size target when splitting them would reduce clarity.

## Change discipline

* Preserve CLI command names, flags, aliases, defaults, and output unless a task explicitly changes behavior.
* Prefer small, reviewable refactors over broad rewrites.
* Keep command argument definitions separate from command execution logic when practical.
* Reuse helpers from `kleio` and `kleio-gedcom` instead of duplicating domain logic in the CLI.
* Use explicit, recoverable error handling; avoid adding panic paths for user input, filesystem, import, or workspace data failures.

## Validation

After Rust changes, run from the workspace root:

* `cargo fmt -p kleio-cli`
* `cargo check -p kleio-cli`

Run targeted CLI commands or tests when command behavior changes. Do not claim validation passed unless the commands actually ran successfully.
