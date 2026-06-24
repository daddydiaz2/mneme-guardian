<!-- gentle-ai:mneme-guardian -->
# mneme-guardian — Code Review Guardian

Provider-agnostic pre-commit code review tool. Pure Bash, zero dependencies.

## Architecture

```
mneme-guardian/
├── bin/mneme-g          # Main script
├── install.sh           # Install to $HOME/.local/bin
├── uninstall.sh         # Uninstall
├── docs/                # Documentation
├── .github/workflows/   # CI
└── README.md
```

## Commands

- `init` — create default config at `~/.config/mneme-guardian/config.sh`
- `install` — install pre-commit hook in current repo
- `uninstall` — remove hook
- `run` — review staged files via AI provider
- `run --ci` — review last commit (for CI pipelines)
- `config` — show current config

## Provider Integration

Each provider is a function that receives a review prompt and returns findings:

```bash
review_with_opencode() { opencode run "..." 2>/dev/null; }
review_with_claude()   { claude -p "..." 2>/dev/null; }
review_with_gemini()   { gemini -p "..." 2>/dev/null; }
review_with_codex()    { codex run -p "..." 2>/dev/null; }
review_with_ollama()   { ollama run <model> "..." 2>/dev/null; }
```

## Adding a Provider

1. Add a function `review_with_<name>()` in `bin/mneme-g`
2. Add the case in `run_review()`
3. Document in README

## Mneme Integration

When `MNEME_ENABLED=true` and `mneme` is on PATH, review results are saved as memories:

```bash
mneme save --project <name> --title "Code review: <date>" --type review --content "<results>"
```

This lets you search past reviews: `mneme search "code review" --project <name>`.
