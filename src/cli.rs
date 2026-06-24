use clap::{Parser, Subcommand};

/// mneme-guardian — Provider-agnostic AI code review.
///
/// Pre-commit guardian that reviews staged files using AI,
/// with optional mneme memory integration for tracking review history.
#[derive(Parser)]
#[command(name = "mneme-g", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create default config (~/.config/mneme-guardian/config.toml)
    Init,

    /// Install git hook (pre-commit or commit-msg)
    Install {
        /// Hook type: pre-commit (default) or commit-msg
        #[arg(long, default_value = "pre-commit")]
        hook_type: String,
    },

    /// Remove git hook
    Uninstall,

    /// Review code changes
    Run {
        /// Review last commit instead of staged changes (CI mode)
        #[arg(long)]
        ci: bool,

        /// Skip cache
        #[arg(long)]
        no_cache: bool,
    },

    /// Show current configuration
    Config,

    /// Launch interactive TUI
    Tui,

    /// Show version
    Version,
}
