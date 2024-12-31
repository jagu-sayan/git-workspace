use git_workspace::config::{all_config_files, Config, ProviderSource};
use git_workspace::providers::{GithubProvider, GitlabProvider};
use insta::assert_debug_snapshot;
use std::fs;
use tempfile::TempDir;

// Succeed to read config

// Select good config file

// Write config file

#[test]
fn test_config_read_write() {
    let temp = TempDir::new().unwrap();
    let config_path = temp.path().join("workspace.toml");
    let providers = vec![
        ProviderSource::Github(GithubProvider::new(
            "test-user".to_string(),
            "github".to_string(),
            "GITHUB_TOKEN".to_string(),
            false,
            vec![],
            vec![],
            false,
            "https://api.github.com/graphql".to_string(),
        )),
        ProviderSource::Gitlab(GitlabProvider::new(
            "test-group".to_string(),
            "gitlab".to_string(),
            "GITLAB_TOKEN".to_string(),
            vec![],
            vec![],
            false,
            "https://gitlab.com".to_string(),
        )),
    ];

    let config = Config::new(vec![config_path.clone()]);
    config.write(providers, &config_path).unwrap();

    let read_providers = config.read().unwrap();
    assert_debug_snapshot!(read_providers);
}

#[test]
fn test_all_config_files() {
    let temp = TempDir::new().unwrap();

    // Create test files
    fs::write(temp.path().join("workspace.toml"), "").unwrap();
    fs::write(temp.path().join("workspace-dev.toml"), "").unwrap();
    fs::write(temp.path().join("workspace-lock.toml"), "").unwrap();
    fs::write(temp.path().join("other.toml"), "").unwrap();

    let files = all_config_files(temp.path()).unwrap();
    let file_names: Vec<_> = files
        .iter()
        .map(|p| p.file_name().unwrap().to_str().unwrap().to_string())
        .collect();

    assert_debug_snapshot!(file_names);
}
