use super::map_repositories;
use crate::commands::get_all_repositories_to_archive;
use crate::display::DisplayType;
use crate::lockfile::Lockfile;
use anyhow::Context;
use console::style;
use std::path::Path;

/// Update our workspace. This clones any new repositories and print old repositories to archives.
pub fn update(workspace: &Path, display: DisplayType, threads: usize) -> anyhow::Result<()> {
    // Load our lockfile
    let lockfile = Lockfile::new(workspace.join("workspace-lock.toml"));
    let repositories = lockfile.read().with_context(|| "Error reading lockfile")?;

    println!("Updating {} repositories", repositories.len());

    // Initialize display
    let display = display.create_display();

    map_repositories(
        &repositories,
        threads,
        |r| {
            // Only clone repositories that don't exist
            if !r.exists(workspace) {
                r.clone(workspace, display.clone())?;
                // Maybe this should always be run, but whatever. It's fine for now.
                r.set_upstream(workspace)?;
            }
            Ok(())
        },
        display.clone(),
    )?;

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
