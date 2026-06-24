use std::path::PathBuf;

/// mneme-guardian configuration
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub provider: String,
    pub model: Option<String>,
    pub rules_file: String,
    pub mneme_enabled: bool,
    pub exit_on_issues: bool,
    pub max_lines: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            provider: "opencode".to_string(),
            model: None,
            rules_file: "./AGENTS.md".to_string(),
            mneme_enabled: true,
            exit_on_issues: true,
            max_lines: 0,
        }
    }
}

fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("mneme-guardian")
}

fn config_path() -> PathBuf {
    config_dir().join("config.toml")
}

/// Initialize default config file
pub fn init_config() -> anyhow::Result<()> {
    let dir = config_dir();
    std::fs::create_dir_all(&dir)?;
    let path = config_path();

    if path.exists() {
        println!("✓ Config already exists: {}", path.display());
        return Ok(());
    }

    let config = Config::default();
    let content = toml::to_string_pretty(&config)?;

    let header = format!(
        r#"# mneme-guardian configuration
# Created: {}
# Override any value with MNEME_G_<KEY> environment variables.

"#,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
    );

    std::fs::write(&path, header + &content)?;
    println!("✓ Created config: {}", path.display());
    Ok(())
}

/// Load configuration with env override support
pub fn load_config() -> anyhow::Result<Config> {
    let mut config = Config::default();

    // Try reading config file
    let path = config_path();
    if path.exists() {
        let content = std::fs::read_to_string(&path)?;
        let file_config: Config = toml::from_str(&content)?;
        config = file_config;
    }

    // Environment variable overrides
    if let Ok(val) = std::env::var("MNEME_G_PROVIDER") {
        config.provider = val;
    }
    if let Ok(val) = std::env::var("MNEME_G_MODEL") {
        config.model = Some(val);
    }
    if let Ok(val) = std::env::var("MNEME_G_RULES") {
        config.rules_file = val;
    }
    if let Ok(val) = std::env::var("MNEME_G_MNEME") {
        config.mneme_enabled = val == "true" || val == "1";
    }

    Ok(config)
}

/// Display current configuration
pub fn show_config() -> anyhow::Result<()> {
    let config = load_config()?;
    println!("{:<20} {}", "Setting", "Value");
    println!("{}", "-".repeat(50));
    println!("{:<20} {}", "Provider", config.provider);
    println!(
        "{:<20} {}",
        "Model",
        config.model.as_deref().unwrap_or("(default)")
    );
    println!("{:<20} {}", "Rules file", config.rules_file);
    println!("{:<20} {}", "Mneme sync", config.mneme_enabled);
    println!("{:<20} {}", "Exit on issues", config.exit_on_issues);
    println!("{:<20} {}", "Max lines", config.max_lines);
    println!();
    println!("Config path: {}", config_path().display());
    Ok(())
}
