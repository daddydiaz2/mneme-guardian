use std::process::Command;

/// Build review prompt from diff content and optional rules
pub fn build_prompt(diff: &str, rules: Option<&str>) -> String {
    let mut prompt = String::new();
    prompt.push_str("You are a strict code reviewer.\n");
    prompt.push_str("Review the following git diff and report issues with severity:\n");
    prompt.push_str("- BLOCKER: security, data loss, broken behavior\n");
    prompt.push_str("- CRITICAL: incorrect logic, missing edge cases\n");
    prompt.push_str("- WARNING: code smells, readability, maintainability\n");
    prompt.push_str("- SUGGESTION: minor improvements\n\n");

    if let Some(r) = rules {
        if !r.is_empty() {
            prompt.push_str("Project rules:\n");
            prompt.push_str(r);
            prompt.push_str("\n\n");
        }
    }

    prompt.push_str("Diff to review:\n```diff\n");
    prompt.push_str(diff);
    prompt.push_str("\n```\n");

    prompt
}

/// Review via OpenCode
pub fn review_opencode(prompt: &str) -> anyhow::Result<String> {
    let output = Command::new("opencode")
        .args(["run", prompt])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run opencode: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("opencode review failed: {}", stderr);
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Review via Claude Code
pub fn review_claude(prompt: &str) -> anyhow::Result<String> {
    let output = Command::new("claude")
        .args(["-p", prompt])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run claude: {}", e))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Review via Gemini CLI
pub fn review_gemini(prompt: &str) -> anyhow::Result<String> {
    let output = Command::new("gemini")
        .args(["-p", prompt])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run gemini: {}", e))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Review via Codex CLI
pub fn review_codex(prompt: &str) -> anyhow::Result<String> {
    let output = Command::new("codex")
        .args(["run", "-p", prompt])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run codex: {}", e))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Review via Ollama
pub fn review_ollama(prompt: &str, model: &str) -> anyhow::Result<String> {
    let output = Command::new("ollama")
        .args(["run", model, prompt])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run ollama: {}", e))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Check if a CLI tool is available
pub fn check_provider(provider: &str) -> bool {
    let cmd = match provider {
        "opencode" => "opencode",
        "claude" => "claude",
        "gemini" => "gemini",
        "codex" => "codex",
        "ollama" => "ollama",
        _ => return false,
    };

    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
