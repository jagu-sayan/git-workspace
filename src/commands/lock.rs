use crate::config::Config;
use crate::display::DisplayRef;
use crate::lockfile::Lockfile;
use crate::processing::ParallelTaskProcessing;
use crate::repository::Repository;
use anyhow::Context;
use std::path::Path;

/// Update our lockfile
pub fn lock(workspace: &Path, display: DisplayRef) -> anyhow::Result<()> {
    // Read the configuration sources
    let config = Config::from_workspace(workspace)?;
    let sources = config
        .read()
        .with_context(|| "Error reading config files")?;

    // For each source, fetch repositories
    println!("Fetching repositories...");
    let processor = ParallelTaskProcessing::new(sources.clone(), 8, display);
    let mut all_repositories: Vec<Repository> = processor
        .map_with_display(|source, _display| {
            source
                .fetch_repositories()
                .with_context(|| format!("Error fetching repositories from {}", source))
        })
        .into_iter()
        .flatten()
        .collect();

    // let all_repositories: Vec<Repository> = all_repository_results.iter().collect::<anyhow::Result<Vec<Repository>>>()?;
    // We may have duplicated repositories here. Make sure they are unique based on the full path.
    all_repositories.sort();
    all_repositories.dedup();

    // Write the lockfile out
    let lockfile = Lockfile::new(workspace.join("workspace-lock.toml"));
    lockfile.write(&all_repositories)?;
    Ok(())
}
