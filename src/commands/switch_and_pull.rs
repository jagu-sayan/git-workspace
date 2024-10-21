use super::map_repositories;
use crate::{display::DisplayType, lockfile::Lockfile};
use anyhow::Context;
use std::path::Path;

pub fn pull_all_repositories(
    workspace: &Path,
    display: DisplayType,
    threads: usize,
) -> anyhow::Result<()> {
    let lockfile = Lockfile::new(workspace.join("workspace-lock.toml"));
    let repositories = lockfile.read().with_context(|| "Error reading lockfile")?;

    println!(
        "Switching to the primary branch and pulling {} repositories",
        repositories.len()
    );

    // Initialize display
    let display = display.create_display();

    map_repositories(
        &repositories,
        threads,
        |r| {
            r.switch_to_primary_branch(workspace)?;
            let pull_args = match (&r.upstream, &r.branch) {
                // This fucking sucks, but it's because my abstractions suck ass.
                // I need to learn how to fix this.
                (Some(_), Some(branch)) => vec![
                    "pull".to_string(),
                    "upstream".to_string(),
                    branch.to_string(),
                ],
                _ => vec!["pull".to_string()],
            };
            r.execute_cmd(workspace, display.clone(), "git", &pull_args)?;
            Ok(())
        },
        display.clone(),
    )?;

    Ok(())
}
