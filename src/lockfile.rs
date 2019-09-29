use crate::repository::Repository;
use failure::Error;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub struct Lockfile {
    path: PathBuf,
}

#[derive(Deserialize, Serialize, Debug)]
struct LockfileContents {
    #[serde(rename = "repo")]
    repos: Vec<Repository>,
}

impl Lockfile {
    pub fn new(path: PathBuf) -> Lockfile {
        Lockfile { path }
    }

    pub fn read(&self) -> Result<Vec<Repository>, Error> {
        let config_data = fs::read_to_string(&self.path)?;
        let config: LockfileContents = toml::from_str(config_data.as_str())?;
        Ok(config.repos)
    }

    pub fn write(&self, repositories: &[Repository]) -> Result<(), Error> {
        let mut sorted_repositories = repositories.to_owned();
        sorted_repositories.sort();

        let toml = toml::to_string(&LockfileContents {
            repos: sorted_repositories,
        })?;
        fs::write(&self.path, toml)?;

        Ok(())
    }
}
