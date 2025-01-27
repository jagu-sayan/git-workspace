use crate::display::DisplayRef;
use crate::lockfile::Lockfile;
use crate::processing::ParallelTaskProcessing;
use std::path::Path;

/// Execute a command on all our repositories
pub fn execute_cmd(
    workspace: &Path,
    threads: usize,
    display: DisplayRef,
    cmd: String,
    args: Vec<String>,
) -> anyhow::Result<()> {
    // Read the lockfile
    let lockfile = Lockfile::new(workspace.join("workspace-lock.toml"));
    let repositories = lockfile.read()?;

    // We only care about repositories that exist
    let mut processor = ParallelTaskProcessing::new(repositories, threads, display);
    processor.filter(|r| r.exists(workspace));
    println!(
        "Running {} {} on {} repositories",
        cmd,
        args.join(" "),
        processor.len(),
    );

    // Run fetch on them
    // processor.process(|repo, display| {
    //     display.update(
    //         repo,
    //         OperationStatus::InProgress,
    //         &format!("Running {} {}", cmd, args.join(" ")),
    //     )?;

    //     repo.execute_cmd(workspace, &cmd, &args)?;

    //     display.update(
    //         repo,
    //         OperationStatus::Success,
    //         &format!("Completed {} {}", cmd, args.join(" ")),
    //     )?;

    //     Ok(())
    // })?;

    Ok(())
}
