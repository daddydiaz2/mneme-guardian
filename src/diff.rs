use std::process::Command;

/// Get git diff for staged changes (pre-commit)
pub fn get_staged_diff() -> anyhow::Result<String> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--"])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run git diff: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git diff failed: {}", stderr);
    }

    let diff = String::from_utf8_lossy(&output.stdout).to_string();

    if diff.trim().is_empty() {
        anyhow::bail!("No staged changes to review");
    }

    Ok(diff)
}

/// Get git diff for the last commit (CI mode)
pub fn get_last_commit_diff() -> anyhow::Result<String> {
    let output = Command::new("git")
        .args(["diff", "HEAD~1", "--"])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run git diff HEAD~1: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git diff HEAD~1 failed: {}", stderr);
    }

    let diff = String::from_utf8_lossy(&output.stdout).to_string();

    if diff.trim().is_empty() {
        anyhow::bail!("No diff found for last commit");
    }

    Ok(diff)
}

/// Get unique changed file list from diff
pub fn get_changed_files(diff: &str) -> Vec<String> {
    let mut files: Vec<String> = diff
        .lines()
        .filter(|l| l.starts_with("+++ b/"))
        .filter_map(|l| {
            let path = l.trim_start_matches("+++ b/");
            if path != "/dev/null" {
                Some(path.to_string())
            } else {
                None
            }
        })
        .collect();
    files.sort();
    files.dedup();
    files
}

/// Detect project name from git remote or cwd
pub fn detect_project() -> String {
    // Try git remote origin URL
    if let Ok(output) = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
    {
        if output.status.success() {
            let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Some(repo) = url.rsplit('/').next() {
                return repo.trim_end_matches(".git").to_string();
            }
        }
    }

    // Fallback: directory name
    std::env::current_dir()
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or_else(|| "unknown".to_string())
}
