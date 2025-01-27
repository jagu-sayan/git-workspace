use crate::commands::get_all_repositories_to_archive;
use crate::display::DisplayRef;
use crate::lockfile::Lockfile;
use crate::processing::ParallelTaskProcessing;
use anyhow::Context;
use console::style;
use std::path::Path;

/// Update our workspace. This clones any new repositories and print old repositories to archives.
pub fn update(workspace: &Path, threads: usize, display: DisplayRef) -> anyhow::Result<()> {
    // Load our lockfile
    let lockfile = Lockfile::new(workspace.join("workspace-lock.toml"));
    let repositories = lockfile.read().with_context(|| "Error reading lockfile")?;

    // Update repositories
    // println!("Updating {} repositories", repositories.len());
    let processor = ParallelTaskProcessing::new(repositories.clone(), threads, display);

    processor.map_with_display(|repo, _display| {
        // Only clone repositories that don't exist
        if !repo.exists(workspace) {
            // display.update(repo, OperationStatus::InProgress, "Cloning repository...")?;
            repo.clone(workspace)?;
            // Maybe this should always be run, but whatever. It's fine for now.
            repo.set_upstream(workspace)?;
            // display.update(repo, OperationStatus::Success, "Clone completed")?;
        } else {
            // display.update(repo, OperationStatus::Skipped, "Already exists")?;
        }
        Ok(())
    });

    let repos_to_archive = get_all_repositories_to_archive(workspace, repositories)?;
    if !repos_to_archive.is_empty() {
        println!(
            "There are {} repositories that can be archived",
            repos_to_archive.len()
        );
        println!(
            "Run {} to archive them",
            style("`git workspace archive`").yellow()
        );
    }

    Ok(())
}
