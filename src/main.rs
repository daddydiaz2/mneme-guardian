use clap::Parser;
use mneme_guardian::cli;
use mneme_guardian::config;
use mneme_guardian::hooks;
use mneme_guardian::review;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Init => config::init_config()?,
        cli::Commands::Install { hook_type } => hooks::install_hook(&hook_type)?,
        cli::Commands::Uninstall => hooks::uninstall_hook()?,
        cli::Commands::Run { ci, no_cache } => {
            let cfg = config::load_config()?;
            review::run_review(&cfg, ci, !no_cache)?;
        }
        cli::Commands::Config => config::show_config()?,
        cli::Commands::Tui => mneme_guardian::tui::run_tui()?,
        cli::Commands::Version => {
            println!("mneme-guardian v{}", env!("CARGO_PKG_VERSION"));
            mneme_guardian::update::print_update_status(
                "mneme-guardian",
                env!("CARGO_PKG_VERSION"),
            );
        }
    }

    Ok(())
}
