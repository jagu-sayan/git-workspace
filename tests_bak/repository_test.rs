use git_workspace::repository::Repository;
use insta::assert_debug_snapshot;
use tempfile::TempDir;

#[test]
fn test_repository_new() {
    let repo = Repository::new(
        "test/repo".to_string(),
        "git@github.com:user/repo.git".to_string(),
        Some("main".to_string()),
        Some("git@github.com:upstream/repo.git".to_string()),
    );

    assert_debug_snapshot!(repo);
}

#[test]
fn test_repository_exists() {
    let temp = TempDir::new().unwrap();
    let repo = Repository::new(
        "test/repo".to_string(),
        "git@github.com:user/repo.git".to_string(),
        None,
        None,
    );

    assert!(!repo.exists(temp.path()));

    // Create .git directory
    std::fs::create_dir_all(temp.path().join("test/repo/.git")).unwrap();
    assert!(repo.exists(temp.path()));
}

#[test]
fn test_repository_name() {
    let repo = Repository::new(
        "test/repo".to_string(),
        "git@github.com:user/repo.git".to_string(),
        None,
        None,
    );

    assert_eq!(repo.name(), "test/repo");
}
