use crate::config::Config;
use crate::diff;
use crate::mneme;
use crate::providers;

/// Run a full review cycle
pub fn run_review(config: &Config, ci_mode: bool, use_cache: bool) -> anyhow::Result<()> {
    // 1. Get diff
    let diff = if ci_mode {
        diff::get_last_commit_diff()?
    } else {
        diff::get_staged_diff()?
    };

    let files = diff::get_changed_files(&diff);
    let project = diff::detect_project();

    println!("🔍 Reviewing {} ({} files)", project, files.len());
    println!("   Provider: {}", config.provider);

    // 2. Check cache (TODO)
    if use_cache {
        // Placeholder for cache logic
    }

    // 3. Load rules
    let rules = std::fs::read_to_string(&config.rules_file).ok();

    // 4. Build prompt
    let prompt = providers::build_prompt(&diff, rules.as_deref());

    // 5. Run review via provider
    println!("   Running review...");
    let result = match config.provider.as_str() {
        "opencode" => providers::review_opencode(&prompt)?,
        "claude" => providers::review_claude(&prompt)?,
        "gemini" => providers::review_gemini(&prompt)?,
        "codex" => providers::review_codex(&prompt)?,
        "ollama" => {
            let model = config.model.as_deref().unwrap_or("qwen2.5-coder:7b");
            providers::review_ollama(&prompt, model)?
        }
        p => anyhow::bail!(
            "Unknown provider: {}. Supported: opencode, claude, gemini, codex, ollama",
            p
        ),
    };

    // 6. Display results
    println!();
    println!("{}", "─".repeat(50));
    println!("📋 Review Results");
    println!("{}", "─".repeat(50));
    println!("{}", result);

    // 7. Check for blockers
    let has_blockers = result.to_lowercase().contains("blocker");
    let has_critical = result.to_lowercase().contains("critical");

    // 8. Save to mneme
    if config.mneme_enabled {
        let importance = if has_blockers {
            "high"
        } else if has_critical {
            "medium"
        } else {
            "low"
        };
        let title = format!(
            "Code review: {} — {}",
            project,
            chrono::Utc::now().format("%Y-%m-%d %H:%M")
        );
        match mneme::save_review(&project, &title, &result, importance) {
            Ok(_) => println!("   ✓ Saved to mneme memory"),
            Err(e) => println!("   ⚠  mneme save skipped: {}", e),
        }
    }

    // 9. Exit with status
    println!("{}", "─".repeat(50));
    if has_blockers && config.exit_on_issues {
        anyhow::bail!("❌ BLOCKER issues found. Fix before commit.");
    }

    println!("✅ Review complete.");
    Ok(())
}
