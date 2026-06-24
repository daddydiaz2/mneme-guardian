use std::path::PathBuf;
use std::process::Command;

/// Find the mneme binary on PATH
pub fn find_mneme() -> Option<PathBuf> {
    if let Ok(output) = Command::new("which").arg("mneme").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Some(PathBuf::from(path));
            }
        }
    }
    None
}

/// Save review result to mneme as a memory
pub fn save_review(
    project: &str,
    title: &str,
    content: &str,
    importance: &str,
) -> anyhow::Result<()> {
    let mneme = find_mneme().ok_or_else(|| anyhow::anyhow!("mneme not found on PATH"))?;

    let output = Command::new(&mneme)
        .arg("save")
        .arg("--project")
        .arg(project)
        .arg("--title")
        .arg(title)
        .arg("--type")
        .arg("review")
        .arg("--importance")
        .arg(importance)
        .arg("--tags")
        .arg("mneme-guardian")
        .arg(content)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("mneme save failed: {}", stderr);
    }

    Ok(())
}
