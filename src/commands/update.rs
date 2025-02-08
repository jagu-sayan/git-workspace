use super::read_lock_file;
use crate::commands::get_all_repositories_to_archive;
use crate::display::{DisplayRef, OutputFormat};
use crate::processing::ParallelTaskProcessing;
use crate::repository::GitCommandResult;
use console::style;
use std::path::Path;

/// Update our workspace. This clones any new repositories and print old repositories to archives.
pub fn update(workspace: &Path, threads: usize, output: OutputFormat) -> anyhow::Result<()> {
    let repositories = read_lock_file(workspace)?;
    let display: DisplayRef<GitCommandResult> = output.create_display();

    // Update repositories
    display.show_message(&format!("Updating {} repositories", repositories.len()));
    let task_name = "Update repositories".to_string();
    let mut processor =
        ParallelTaskProcessing::new(task_name, repositories.clone(), threads, display);
    processor
        .filter(|r| r.exists(workspace)) // Only clone repositories that don't exist
        .map_with_display(|repo, _display| {
            repo.clone(workspace)?;
            // Maybe this should always be run, but whatever. It's fine for now.
            repo.set_upstream(workspace)
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
