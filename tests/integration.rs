use std::process::Command;

/// Path to the mneme-g binary (uses CARGO_BIN_EXE at test time)
fn mneme_g_path() -> std::path::PathBuf {
    std::env::var("CARGO_BIN_EXE_MNEME_G")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            // Fallback: assume we're in the workspace
            let mut path = std::env::current_dir().unwrap_or_default();
            // Tests run from workspace root, binary is in target/debug/mneme-g
            if cfg!(debug_assertions) {
                path.push("target");
                path.push("debug");
            } else {
                path.push("target");
                path.push("release");
            }
            path.push("mneme-g");
            path
        })
}

#[test]
fn test_cli_version() {
    let output = Command::new(mneme_g_path())
        .arg("--version")
        .output()
        .expect("Failed to run mneme-g --version");
    assert!(output.status.success());
    let version = String::from_utf8_lossy(&output.stdout);
    assert!(!version.is_empty(), "Version output should not be empty");
}

#[test]
fn test_cli_help() {
    let output = Command::new(mneme_g_path())
        .arg("--help")
        .output()
        .expect("Failed to run mneme-g --help");
    assert!(output.status.success());
    let help = String::from_utf8_lossy(&output.stdout);
    assert!(help.contains("init"));
    assert!(help.contains("install"));
    assert!(help.contains("run"));
    assert!(help.contains("config"));
}

#[test]
fn test_init_creates_config() {
    let tmpdir = tempfile::tempdir().expect("Failed to create temp dir");
    let config_dir = tmpdir.path().join("config");

    let output = Command::new(mneme_g_path())
        .arg("init")
        .env("XDG_CONFIG_HOME", config_dir.as_os_str().to_str().unwrap())
        .output()
        .expect("Failed to run mneme-g init");
    assert!(output.status.success());

    let config_path = config_dir.join("mneme-guardian").join("config.toml");
    assert!(config_path.exists(), "Config file should be created");
}

#[test]
fn test_run_in_non_git_dir_fails_gracefully() {
    let tmpdir = tempfile::tempdir().expect("Failed to create temp dir");

    let output = Command::new(mneme_g_path())
        .arg("run")
        .env("MNEME_G_MNEME", "false")
        .current_dir(tmpdir.path())
        .output()
        .expect("Failed to run mneme-g run");
    assert!(!output.status.success());
}
