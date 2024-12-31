extern crate mockito;

use git_workspace::providers::{
    create_exclude_regex_set, create_include_regex_set, GithubProvider, GitlabProvider, Provider,
};
use insta::assert_debug_snapshot;
use std::env;

#[test]
fn test_github_provider() {
    let provider = GithubProvider::new(
        "test-user".to_string(),
        "github".to_string(),
        "GITHUB_TOKEN".to_string(),
        false,
        vec![],
        vec![],
        false,
        "https://api.github.com/graphql".to_string(),
    );

    assert_debug_snapshot!(provider);
}

#[test]
fn test_gitlab_provider() {
    let provider = GitlabProvider::new(
        "test-group".to_string(),
        "gitlab".to_string(),
        "GITLAB_TOKEN".to_string(),
        vec![],
        vec![],
        false,
        "https://gitlab.com".to_string(),
    );

    assert_debug_snapshot!(provider);
}

#[test]
fn test_regex_sets() {
    let include = vec!["test.*".to_string(), "demo.*".to_string()];
    let exclude = vec!["temp.*".to_string()];

    let include_set = create_include_regex_set(&include).unwrap();
    let exclude_set = create_exclude_regex_set(&exclude).unwrap();

    assert!(include_set.is_match("test-repo"));
    assert!(include_set.is_match("demo-repo"));
    assert!(!include_set.is_match("other-repo"));

    assert!(exclude_set.is_match("temp-repo"));
    assert!(!exclude_set.is_match("test-repo"));
}

#[test]
fn test_provider_fetch_with_mock() {
    let mut server = mockito::Server::new();

    // Mock GitHub API response
    let mock = server
        .mock("POST", "/graphql")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(include_str!("fixtures/github_response.json"))
        .create();

    let provider = GithubProvider::new(
        "test-user".to_string(),
        "github".to_string(),
        "GITHUB_TOKEN".to_string(),
        false,
        vec![],
        vec![],
        false,
        server.url(),
    );

    env::set_var("TEST_GITHUB_TOKEN", "test-token");
    let repos = provider.fetch_repositories().unwrap();

    assert_debug_snapshot!(repos);
    mock.assert();
}
