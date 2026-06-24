# mneme-guardian 😇

**AI code review guardian** — pre-commit reviews que protegen tu código.

Provider-agnostic (Claude, Gemini, OpenCode, Codex, Ollama), cero dependencias, Bash puro. Opcionalmente guarda resultados de review en **mneme** como memoria persistente.

```
┌──────────────┐     ┌────────────────┐     ┌──────────────────┐
│  git commit  │ ──▶ │  AI Review     │ ──▶ │  ✅ Pass/Fail    │
│ (staged)     │     │  (any LLM)     │     │  (+ mneme save)  │
└──────────────┘     └────────────────┘     └──────────────────┘
```

## Instalación

```bash
git clone https://github.com/daddydiaz2/mneme-guardian.git
cd mneme-guardian
./install.sh
```

O directo:

```bash
sudo ln -s "$PWD/bin/mneme-g" /usr/local/bin/mneme-g
```

## Quick Start

```bash
cd tu-proyecto

# Crear config
mneme-g init

# Instalar hook pre-commit
mneme-g install

# Review manual
mneme-g run
```

A partir de ahora, cada `git commit` revisa los staged files automáticamente.

## Providers

| Provider | Variable | Comando requerido |
|----------|----------|-------------------|
| **OpenCode** | `MNEME_G_PROVIDER=opencode` (default) | `opencode` |
| **Claude** | `MNEME_G_PROVIDER=claude` | `claude` |
| **Gemini** | `MNEME_G_PROVIDER=gemini` | `gemini` |
| **Codex** | `MNEME_G_PROVIDER=codex` | `codex` |
| **Ollama** | `MNEME_G_PROVIDER=ollama` | `ollama` |

```bash
# Review con Claude
MNEME_G_PROVIDER=claude mneme-g run

# Review con Ollama local
MNEME_G_PROVIDER=ollama MNEME_G_MODEL=qwen2.5-coder:7b mneme-g run
```

## Mneme Integration

Si tenés **mneme** instalado, los resultados de cada review se guardan automáticamente como memorias:

```bash
mneme search "code review" --project tu-proyecto
```

Para desactivarlo: `MNEME_G_MNEME=false mneme-g run`

## Comandos

| Comando | Descripción |
|---------|-------------|
| `mneme-g init` | Crear config por defecto |
| `mneme-g install` | Instalar hook pre-commit |
| `mneme-g uninstall` | Remover hook |
| `mneme-g run` | Review staged files |
| `mneme-g run --ci` | Review último commit (CI) |
| `mneme-g config` | Mostrar configuración |
| `mneme-g version` | Versión |

## Config

El archivo de config se crea en `~/.config/mneme-guardian/config.sh` con `mneme-g init`:

```bash
PROVIDER="opencode"
MODEL=""
RULES_FILE="./AGENTS.md"
MNEME_ENABLED=true
EXIT_ON_ISSUES=true
```

Todo se sobreescribe con variables de entorno `MNEME_G_*`.

## Reglas de Review

El guardian usa `AGENTS.md` del proyecto como reglas de review. Si no existe, usa su propio prompt por defecto. Creá un `AGENTS.md` en tu proyecto para reglas personalizadas:

```markdown
## Code Review Rules
- No secrets hardcodeados
- Tests obligatorios para nueva lógica
- Nombres de funciones en inglés
```

## Licencia

MIT
