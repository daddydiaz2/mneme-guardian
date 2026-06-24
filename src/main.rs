mod cli;
mod config;
mod diff;
mod hooks;
mod providers;
mod review;
mod mneme;

use clap::Parser;

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
        cli::Commands::Version => {
            println!("mneme-guardian v{}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}
