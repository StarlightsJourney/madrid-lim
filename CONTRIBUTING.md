# Contributing

`STYLE.md` is the binding product and visual specification. Read `AGENTS.md` and `README.md` before changing code.

## Authorship policy

Madrid Lim (`StarlightsJourney`) is the only contributor to this repository.

- AI assistants, coding agents, and bots such as Devin, Claude, Copilot, ChatGPT, Codex, Cursor, Windsurf, and Gemini must not appear as contributors.
- Do not add `Co-authored-by` trailers for any AI assistant, agent, or bot account.
- Do not add generated-by lines, such as `Generated with ...`, to commit messages.
- Commit author and committer identities must be the repository owner. Bot identities, including `*[bot]` accounts and `noreply@anthropic.com`, are rejected.

These rules keep the GitHub contributor graph accurate.

## Git hooks

The policy is enforced by the hooks in `.githooks/`. Enable them once per clone:

```sh
git config core.hooksPath .githooks
```

- `commit-msg` rejects commit messages that credit an AI assistant or bot.
- `pre-push` checks every outgoing commit message, author, and committer.
- `.github/workflows/contributors.yml` runs the same check on every push and pull request.

The shared rules live in `.githooks/check-contributors`. Run it directly to audit a range:

```sh
.githooks/check-contributors range origin/main..HEAD
```

Do not bypass the hooks with `--no-verify`.

## Validation

Run before pushing:

```sh
cargo fmt --check
cargo clippy --target wasm32-unknown-unknown -- -D warnings
cargo test
trunk build --release
```
