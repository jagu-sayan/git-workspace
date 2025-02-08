use super::read_lock_file;
use crate::display::OutputFormat;
use crate::processing::ParallelTaskProcessing;
use std::path::Path;

/// Execute a command on all our repositories
pub fn execute_cmd(
    workspace: &Path,
    threads: usize,
    output: OutputFormat,
    cmd: String,
    args: Vec<String>,
) -> anyhow::Result<()> {
    let repositories = read_lock_file(&workspace.join("workspace-lock.toml"))?;
    let display = output.create_display();

    // We only care about repositories that exist//
    let task_name = "Run command".to_string();
    let mut processor = ParallelTaskProcessing::new(task_name, repositories, threads, display);
    processor.filter(|r| r.exists(workspace));
    println!(
        "Running {} {} on {} repositories",
        cmd,
        args.join(" "),
        processor.len(),
    );

    // Run fetch on them
    processor.map_with_display(|repo, _display| repo.execute_cmd(workspace, &cmd, &args));

    Ok(())
}
