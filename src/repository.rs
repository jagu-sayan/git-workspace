use crate::display::DisplayableResult;
use crate::processing::Identifiable;
use anyhow::{Context, Result};
use git2::build::CheckoutBuilder;
use git2::{Repository as Git2Repository, StatusOptions};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Serialize)]
pub struct GitCommandResult {
    pub repo: Repository,
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

impl DisplayableResult for GitCommandResult {
    fn is_success(&self) -> bool {
        self.success
    }

    // fn get_name(&self) -> &str {
    //     &self.name
    // }

    // fn get_message(&self) -> &str {
    //     self.output.as_str()
    // }
}

// Eq, Ord and friends are needed to order the list of repositories
#[derive(Deserialize, Serialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Repository {
    path: String,
    url: String,
    pub upstream: Option<String>,
    pub branch: Option<String>,
}

impl Repository {
    pub fn new(
        path: String,
        url: String,
        branch: Option<String>,
        upstream: Option<String>,
    ) -> Repository {
        // We have to normalize repository names here. On windows if you do `path.join(self.name())`
        // it will cause issues if the name contains a forward slash. So here we just normalize it
        // to the path separator on the system.
        let norm_path = if cfg!(windows) {
            path.replace('/', std::path::MAIN_SEPARATOR.to_string().as_str())
        } else {
            path
        };

        Repository {
            path: norm_path,
            url,
            branch,
            upstream,
        }
    }

    pub fn set_upstream(&self, root: &Path) -> Result<GitCommandResult> {
        let upstream = match &self.upstream {
            Some(upstream) => upstream,
            None => {
                return Ok(GitCommandResult {
                    repo: self.to_owned(),
                    success: true,
                    output: "No upstream configured".to_string(),
                    error: None,
                })
            }
        };

        let mut command = Command::new("git");
        let child = command
            .arg("-C")
            .arg(root.join(self.name()))
            .arg("remote")
            .arg("rm")
            .arg("upstream")
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        child.status()?;

        let mut command = Command::new("git");
        let child = command
            .arg("-C")
            .arg(root.join(self.name()))
            .arg("remote")
            .arg("add")
            .arg("upstream")
            .arg(upstream);

        let output = child.output()?;
        if !output.status.success() {
            let stderr =
                std::str::from_utf8(&output.stderr).with_context(|| "Error decoding git output")?;
            return Ok(GitCommandResult {
                repo: self.to_owned(),
                success: false,
                output: String::new(),
                error: Some(format!("Failed to set upstream: {}", stderr.trim())),
            });
        }

        Ok(GitCommandResult {
            repo: self.to_owned(),
            success: true,
            output: "Upstream configured successfully".to_string(),
            error: None,
        })
    }

    pub fn execute_cmd(&self, root: &Path, cmd: &str, args: &[String]) -> Result<GitCommandResult> {
        let mut command = Command::new(cmd);
        command
            .args(args)
            .current_dir(root.join(self.name()))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let output = command
            .output()
            .with_context(|| format!("Failed to execute command '{}' with args {:?}", cmd, args))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Ok(GitCommandResult {
                repo: self.to_owned(),
                success: false,
                output: String::new(),
                error: Some(stderr.to_string()),
            });
        }

        Ok(GitCommandResult {
            repo: self.to_owned(),
            success: true,
            output: String::from_utf8_lossy(&output.stdout).trim().to_string(),
            error: None,
        })
    }

    pub fn switch_to_primary_branch(&self, root: &Path) -> Result<GitCommandResult> {
        let branch = match &self.branch {
            None => {
                return Ok(GitCommandResult {
                    repo: self.to_owned(),
                    success: true,
                    output: "No primary branch configured".to_string(),
                    error: None,
                })
            }
            Some(b) => b,
        };
        let repo = Git2Repository::init(root.join(self.name()))?;
        let status = repo.statuses(Some(&mut StatusOptions::default()))?;
        if !status.is_empty() {
            return Ok(GitCommandResult {
                repo: self.to_owned(),
                success: false,
                output: String::new(),
                error: Some(format!(
                    "Repository is dirty, cannot switch to branch {}",
                    branch
                )),
            });
        }
        repo.set_head(&format!("refs/heads/{}", branch))
            .with_context(|| format!("Cannot find branch {}", branch))?;
        repo.checkout_head(Some(CheckoutBuilder::default().safe().force()))
            .with_context(|| format!("Error checking out branch {}", branch))?;

        Ok(GitCommandResult {
            repo: self.to_owned(),
            success: true,
            output: format!("Switched to branch {}", branch),
            error: None,
        })
    }

    pub fn clone(&self, root: &Path) -> Result<GitCommandResult> {
        self.execute_cmd(
            root,
            "git",
            &[
                "clone".to_string(),
                "--recurse-submodules".to_string(),
                "--progress".to_string(),
                self.url.clone(),
                root.join(self.name()).to_string_lossy().into_owned(),
            ],
        )
    }

    pub fn pull(&self, root: &Path) -> Result<GitCommandResult> {
        match (&self.upstream, &self.branch) {
            (Some(_), Some(branch)) => self.execute_cmd(
                root,
                "git",
                &[
                    "pull".to_string(),
                    "upstream".to_string(),
                    branch.to_string(),
                ],
            ),
            _ => self.execute_cmd(root, "git", &["pull".to_string()]),
        }
    }

    pub fn name(&self) -> &String {
        &self.path
    }

    pub fn get_path(&self, root: &Path) -> Result<PathBuf> {
        let joined = root.join(self.name());
        joined
            .canonicalize()
            .with_context(|| format!("Cannot resolve {}", joined.display()))
    }

    pub fn exists(&self, root: &Path) -> bool {
        match self.get_path(root) {
            Ok(path) => {
                let git_dir = root.join(path).join(".git");
                git_dir.exists() && git_dir.is_dir()
            }
            Err(_) => false,
        }
    }
}

impl Identifiable for Repository {
    fn name(&self) -> String {
        self.name().to_owned()
    }
}
