use super::map_repositories;
use crate::lockfile::Lockfile;
use anyhow::Context;
use std::path::Path;

pub fn pull_all_repositories(workspace: &Path, threads: usize) -> anyhow::Result<()> {
    let lockfile = Lockfile::new(workspace.join("workspace-lock.toml"));
    let repositories = lockfile.read().with_context(|| "Error reading lockfile")?;

    println!(
        "Switching to the primary branch and pulling {} repositories",
        repositories.len()
    );

    map_repositories(&repositories, threads, |r, _| {
        r.switch_to_primary_branch(workspace)?;
        r.pull(workspace)?;
        Ok(())
    })?;

    Ok(())
}
