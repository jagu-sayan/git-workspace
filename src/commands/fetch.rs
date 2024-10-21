use super::execute_cmd;
use crate::display::DisplayType;
use std::path::Path;

/// Run `git fetch` on all our repositories
pub fn fetch(workspace: &Path, display: DisplayType, threads: usize) -> anyhow::Result<()> {
    let cmd = [
        "fetch",
        "--all",
        "--prune",
        "--recurse-submodules=on-demand",
        "--progress",
    ];
    execute_cmd(
        workspace,
        display,
        threads,
        "git".to_string(),
        cmd.iter().map(|s| (*s).to_string()).collect(),
    )?;
    Ok(())
}
