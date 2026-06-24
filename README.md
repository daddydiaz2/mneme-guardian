# mneme-guardian 😇

**Provider-agnostic AI code review guardian.** Written in Rust.

Pre-commit reviews that catch issues before they hit the repo. Works with any AI provider and optionally saves results to [mneme](https://github.com/daddydiaz2/mneme) for searchable review history.

Inspired by [Gentleman Guardian Angel](https://github.com/Gentleman-Programming/gentleman-guardian-angel) but built in Rust with mneme integration.

## Quick Start

```bash
cargo install mneme-guardian

cd your-project

# Create config
mneme-g init

# Install pre-commit hook
mneme-g install

# Manual review
mneme-g run
```

Now every `git commit` automatically reviews staged files.

## Providers

Set via `MNEME_G_PROVIDER` or `~/.config/mneme-guardian/config.toml`:

| Provider | Value | Required CLI |
|----------|-------|-------------|
| OpenCode (default) | `opencode` | `opencode` |
| Claude Code | `claude` | `claude` |
| Gemini CLI | `gemini` | `gemini` |
| Codex CLI | `codex` | `codex` |
| Ollama | `ollama` | `ollama` + model |

```bash
MNEME_G_PROVIDER=claude mneme-g run
MNEME_G_PROVIDER=ollama MNEME_G_MODEL=qwen2.5-coder:7b mneme-g run
```

## Mneme Integration

If mneme is installed, review results are saved as memories automatically:

```bash
mneme search "code review" --project my-project
```

Disable with: `MNEME_G_MNEME=false mneme-g run`

## Commands

| Command | Description |
|---------|-------------|
| `mneme-g init` | Create default config |
| `mneme-g install` | Install pre-commit hook |
| `mneme-g install --hook-type commit-msg` | Install commit-msg hook |
| `mneme-g uninstall` | Remove hooks |
| `mneme-g run` | Review staged changes |
| `mneme-g run --ci` | Review last commit (CI mode) |
| `mneme-g config` | Show configuration |
| `mneme-g version` | Show version |

## CI Integration

```bash
# GitHub Actions step
- name: Code Review
  run: mneme-g run --ci
```

## Exit Codes

- **0**: No issues found (or warnings only)
- **1**: BLOCKER issues found (exit configured via `exit_on_issues`)

## License

MIT
