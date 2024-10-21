use super::execute_cmd;
use crate::display::Display;
use std::{path::Path, sync::Arc};

/// Run `git fetch` on all our repositories
pub fn fetch(
    workspace: &Path,
    display: Arc<dyn Display + Sync + Send>,
    threads: usize,
) -> anyhow::Result<()> {
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
