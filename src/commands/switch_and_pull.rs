use super::read_lock_file;
use crate::{display::OutputFormat, processing::ParallelTaskProcessing};
use std::path::Path;

pub fn pull_all_repositories(
    workspace: &Path,
    threads: usize,
    output: OutputFormat,
) -> anyhow::Result<()> {
    let repositories = read_lock_file(workspace)?;
    let display = output.create_display();

    println!(
        "Switching to the primary branch and pulling {} repositories",
        repositories.len()
    );

    // Run fetch on them
    let task_name = "Update repositories".to_string();
    let processor = ParallelTaskProcessing::new(task_name, repositories, threads, display);
    processor.map_with_display(|repo, _display| {
        repo.switch_to_primary_branch(workspace)?;
        repo.pull(workspace)
    });

    Ok(())
}
