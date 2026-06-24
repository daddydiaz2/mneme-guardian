use mneme_guardian::diff;

#[test]
fn test_get_changed_files_empty_diff() {
    let files = diff::get_changed_files("");
    assert!(files.is_empty());
}

#[test]
fn test_get_changed_files_parses_additions() {
    let diff = "diff --git a/src/main.rs b/src/main.rs
new file mode 100644
index 0000000..abc1234
--- /dev/null
+++ b/src/main.rs
@@ -0,0 +1,3 @@
+fn main() {}
+// test
+fn add(a: i32, b: i32) -> i32 { a + b }";
    let files = diff::get_changed_files(diff);
    assert_eq!(files, vec!["src/main.rs"]);
}

#[test]
fn test_get_changed_files_parses_modifications() {
    let diff = "diff --git a/src/lib.rs b/src/lib.rs
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1 +1,2 @@
-pub fn old() {}
+pub fn new() {}
+// comment";
    let files = diff::get_changed_files(diff);
    assert_eq!(files, vec!["src/lib.rs"]);
}

#[test]
fn test_get_changed_files_multiple_files() {
    let diff = "diff --git a/a.rs b/a.rs
--- /dev/null
+++ b/a.rs
@@ -0,0 +1 @@
// a
diff --git a/b.rs b/b.rs
--- /dev/null
+++ b/b.rs
@@ -0,0 +1 @@
// b";
    let files = diff::get_changed_files(diff);
    assert_eq!(files.len(), 2);
    assert!(files.contains(&"a.rs".to_string()));
    assert!(files.contains(&"b.rs".to_string()));
}
