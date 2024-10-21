use super::map_repositories;
use crate::display::Display;
use crate::lockfile::Lockfile;
use crate::repository::Repository;
use std::path::Path;
use std::sync::Arc;

/// Execute a command on all our repositories
pub fn execute_cmd(
    workspace: &Path,
    display: Arc<dyn Display + Sync + Send>,
    threads: usize,
    cmd: String,
    args: Vec<String>,
) -> anyhow::Result<()> {
    // Read the lockfile
    let lockfile = Lockfile::new(workspace.join("workspace-lock.toml"));
    let repositories = lockfile.read()?;

    // We only care about repositories that exist
    let repos_to_fetch: Vec<Repository> = repositories
        .iter()
        .filter(|r| r.exists(workspace))
        .cloned()
        .collect();

    println!(
        "Running {} {} on {} repositories",
        cmd,
        args.join(" "),
        repos_to_fetch.len()
    );

    // Run fetch on them
    map_repositories(
        &repos_to_fetch,
        threads,
        |r| r.execute_cmd(workspace, display.clone(), &cmd, &args),
        display.clone(),
    )?;
    Ok(())
}
