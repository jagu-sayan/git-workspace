mod gitea;
mod github;
mod gitlab;

use crate::repository::Repository;
use anyhow::Context;
pub use gitea::GiteaProvider;
pub use github::GithubProvider;
pub use gitlab::GitlabProvider;
use serde::{Deserialize, Serialize};
use std::fmt;

pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub trait Provider: fmt::Display {
    /// Returns true if the provider should work, otherwise prints an error and return false
    fn correctly_configured(&self) -> bool;
    fn fetch_repositories(&self) -> anyhow::Result<Vec<Repository>>;
}

pub fn create_exclude_regex_set(items: &Vec<String>) -> anyhow::Result<regex::RegexSet> {
    if items.is_empty() {
        Ok(regex::RegexSet::empty())
    } else {
        Ok(regex::RegexSet::new(items).context("Error parsing exclude regular expressions")?)
    }
}

pub fn create_include_regex_set(items: &Vec<String>) -> anyhow::Result<regex::RegexSet> {
    if items.is_empty() {
        let all = vec![".*"];
        Ok(regex::RegexSet::new(all).context("Error parsing include regular expressions")?)
    } else {
        Ok(regex::RegexSet::new(items).context("Error parsing include regular expressions")?)
    }
}

#[derive(Deserialize, Serialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[serde(tag = "provider")]
#[serde(rename_all = "lowercase")]
#[derive(clap::Subcommand)]
pub enum ProviderSource {
    Gitea(GiteaProvider),
    Gitlab(GitlabProvider),
    Github(GithubProvider),
}

impl ProviderSource {
    pub fn provider(&self) -> &dyn Provider {
        match self {
            Self::Gitea(config) => config,
            Self::Gitlab(config) => config,
            Self::Github(config) => config,
        }
    }

    pub fn correctly_configured(&self) -> bool {
        self.provider().correctly_configured()
    }

    pub fn fetch_repositories(&self) -> anyhow::Result<Vec<Repository>> {
        self.provider().fetch_repositories()
    }
}

impl fmt::Display for ProviderSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.provider())
    }
}
