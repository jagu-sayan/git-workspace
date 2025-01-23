use crate::{config::Config, group::GroupConfig};
use anyhow::{Context, Result};
use std::path::Path;

pub fn add_group_to_config(workspace: &Path, group: GroupConfig, file: &Path) -> Result<()> {
    // if !group.correctly_configured() {
    //     return Err(anyhow!("Provider is not correctly configured"));
    // }

    let path_to_config = workspace.join(file);

    // Load and parse our configuration files
    let config = Config::new(vec![path_to_config]);
    let mut sources = config.read().with_context(|| "Error reading config file")?;

    Ok(())
}
