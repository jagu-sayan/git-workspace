mod common;

use common::{create_test_config, GiteaContainer};
use git_workspace::commands::{lock, update};
use rstest::*;
use std::sync::OnceLock;
use tempfile::TempDir;

//
static GITEA_CONTAINER: OnceLock<GiteaContainer> = OnceLock::new();

// Fixture to get or start container if not started
#[fixture]
pub fn gitea_container() -> &'static GiteaContainer {
    GITEA_CONTAINER.get_or_init(|| GiteaContainer::start())
}

const WORKSPACE_FILE_CONTENT: &str = r#"[[provider]]
provider = "gitea"
name = "jagu"
url = "https://localhost"
path = "gitea"
env_var = "GITEA_TOKEN"
skip_forks = false
auth_http = true
include = []
exclude = []"#;

#[rstest]
fn test_update_command(gitea_container: &GiteaContainer) {
    let server = gitea_container;
    let temp_dir = TempDir::new().unwrap();
    let workspace_path = temp_dir.path();

    // Create test config files
    server.save_cert(&workspace_path).unwrap();
    create_test_config(workspace_path, "workspace.toml", WORKSPACE_FILE_CONTENT);

    std::env::set_var("SSL_CERT_FILE", workspace_path.join("bundle.pem"));
    std::env::set_var("GITEA_TOKEN", &server.token);

    // Test update command
    lock(workspace_path).unwrap();
    update(workspace_path, 1).unwrap();

    std::env::remove_var("SSL_CERT_FILE");
    std::env::remove_var("GITEA_TOKEN");
    assert_eq!(server.ca, "fregf");
}
