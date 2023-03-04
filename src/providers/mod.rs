mod github;
mod gitlab;

use crate::repository::Repository;
use anyhow::Context;
pub use github::GithubProvider;
pub use gitlab::GitlabProvider;
use std::fmt;

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
