use super::execute_cmd;
use crate::display::OutputFormat;
use std::path::Path;

/// Run `git fetch` on all our repositories
pub fn fetch(workspace: &Path, threads: usize, output: OutputFormat) -> anyhow::Result<()> {
    let cmd = [
        "fetch",
        "--all",
        "--prune",
        "--recurse-submodules=on-demand",
        "--progress",
    ];
    execute_cmd(
        workspace,
        threads,
        output,
        "git".to_string(),
        cmd.iter().map(|s| (*s).to_string()).collect(),
    )?;
    Ok(())
}
