use crate::display::DisplayRef;

use super::execute_cmd;
use std::path::Path;

/// Run `git fetch` on all our repositories
pub fn fetch(workspace: &Path, threads: usize, display: DisplayRef) -> anyhow::Result<()> {
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
        display,
        "git".to_string(),
        cmd.iter().map(|s| (*s).to_string()).collect(),
    )?;
    Ok(())
}
