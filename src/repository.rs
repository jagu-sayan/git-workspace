use crate::processing::Identifiable;
use anyhow::{anyhow, Context, Result};
use git2::build::CheckoutBuilder;
use git2::{Repository as Git2Repository, StatusOptions};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

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

    pub fn set_upstream(&self, root: &Path) -> Result<()> {
        let upstream = match &self.upstream {
            Some(upstream) => upstream,
            None => return Ok(()),
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
            return Err(anyhow!(
                "Failed to set upstream on repo {}: {}",
                root.display(),
                stderr.trim()
            ));
        }
        Ok(())
    }

    pub fn execute_cmd(&self, root: &Path, cmd: &str, args: &[String]) -> Result<String> {
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
            return Err(anyhow!(
                "Command failed with status {}: {}",
                output.status,
                stderr.trim()
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    pub fn switch_to_primary_branch(&self, root: &Path) -> Result<()> {
        let branch = match &self.branch {
            None => return Ok(()),
            Some(b) => b,
        };
        let repo = Git2Repository::init(root.join(self.name()))?;
        let status = repo.statuses(Some(&mut StatusOptions::default()))?;
        if !status.is_empty() {
            return Err(anyhow!(
                "Repository is dirty, cannot switch to branch {}",
                branch
            ));
        }
        repo.set_head(&format!("refs/heads/{}", branch))
            .with_context(|| format!("Cannot find branch {}", branch))?;
        repo.checkout_head(Some(CheckoutBuilder::default().safe().force()))
            .with_context(|| format!("Error checking out branch {}", branch))?;
        Ok(())
    }

    pub fn clone(&self, root: &Path) -> Result<String> {
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

    pub fn pull(&self, root: &Path) -> Result<String> {
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
