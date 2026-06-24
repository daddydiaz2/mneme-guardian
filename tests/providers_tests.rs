use mneme_guardian::providers;

#[test]
fn test_build_prompt_contains_diff() {
    let diff = "--- a/src/main.rs\n+++ b/src/main.rs\n+fn hello() {}";
    let prompt = providers::build_prompt(diff, None);
    assert!(prompt.contains(diff));
    assert!(prompt.contains("BLOCKER"));
    assert!(prompt.contains("CRITICAL"));
    assert!(prompt.contains("WARNING"));
    assert!(prompt.contains("SUGGESTION"));
}

#[test]
fn test_build_prompt_with_rules() {
    let diff = "+fn test() {}";
    let rules = "No unsafe code allowed.";
    let prompt = providers::build_prompt(diff, Some(rules));
    assert!(prompt.contains(rules));
}
