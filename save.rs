
let org_dir = workspace.join(&org_name).join("repo1").join(".git");
    println!("Files in {}:", org_dir.display());
    for entry in std::fs::read_dir(org_dir).unwrap() {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }


#[rstest]
fn test_run_switch_and_pull_commands(gitea_container: &GiteaContainer) {
    // Setup environment
    let (tmp_dir, org_name) = gitea_container.setup();
    let workspace = tmp_dir.path();
    let repos = ["repo1", "repo2"];
    gitea_container.add_repos(&org_name, &repos);
    update_command(workspace);

    // Test execute command
    execute_command(workspace, "echo", "Hello > README.md");

    // std::thread::sleep(std::time::Duration::from_secs(5));

    let org_dir = workspace.join(&org_name).join("repo1");
    println!("Files in {}:", org_dir.display());
    for entry in std::fs::read_dir(org_dir).unwrap() {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }
    execute_command(workspace, "git", "add README.md");
    execute_command(workspace, "git", "commit -m 'chore: initial commit'");
    execute_command(workspace, "git", "push -u origin main");
    execute_command(workspace, "git", "checkout -b chore/test-branch");

    // Check that we are on the chore/test-branch with no README.md file
    for repo in repos {
        let repo_path = workspace.join(format!("{}/{}/.git/HEAD", org_name, repo));
        let readme_path = workspace.join(format!("{}/{}/README.md", org_name, repo));
        let branch = read_to_string(repo_path).unwrap();
        assert_eq!(branch.trim(), "ref: refs/heads/chore/test-branch");
        assert!(
            !readme_path.exists(),
            "README.md should not exist on chore/test-branch"
        )
    }

    // Test switch and pull command
    pull_all_repositories(workspace, 8).unwrap();

    // Check that we are on the main branch with README.me file
    for repo in repos {
        let repo_path = workspace.join(format!("{}/{}/.git/HEAD", org_name, repo));
        let readme_path = workspace.join(format!("{}/{}/README.md", org_name, repo));
        let branch = read_to_string(repo_path).unwrap();
        println!("BR {}", branch);
        assert_eq!(branch.trim(), "ref: refs/heads/main");
        assert!(
            !readme_path.exists(),
            "README.md should exist on main branch"
        )
    }
    assert!(false);

    gitea_container.reset(tmp_dir);
}

/// Resets the test environment
    ///
    /// This method:
    /// 1. Removes all test repositories
    /// 2. Removes all environment variables set during setup
    /// 3. Cleans up all temporary files
    pub fn reset(&self) {
        // List and remove all repositories
        #[derive(Deserialize, Debug)]
        struct Repo {
            name: String,
        }
        let url = format!("{}/api/v1/users/{}/repos", self.url, self.username);
        let response = self
            .http_client
            .get(&url)
            .bearer_auth(&self.token)
            .send()
            .expect(&format!("list repos of {} user", self.username))
            .error_for_status()
            .expect(&format!(
                "got 2xx http response for listing repos of {} user",
                self.username
            ));
        let repos: Vec<Repo> = response.json().expect("deserialize list respos");
        self.delete_repos(repos.into_iter().map(|r| r.name));

        // Clean up environment variables
        std::env::remove_var("SSL_CERT_FILE");
        std::env::remove_var("GIT_SSL_NO_VERIFY");
        std::env::remove_var("GITEA_TOKEN");
        std::env::remove_var("XDG_CONFIG_HOME");
        std::env::remove_var("GIT_CONFIG_NOSYSTEM");

        // Clean up temporary files
        let tmp_dir = self.tmp_dir.path();
        for entry in read_dir(tmp_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                remove_dir_all(path).unwrap();
            } else {
                remove_file(path).unwrap();
            }
        }
