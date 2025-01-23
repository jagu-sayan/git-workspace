pub mod add;
pub mod list;
pub mod remove;

use std::path::PathBuf;

pub use add::add_group_to_config;
pub use list::list_groups;
pub use remove::remove_group_from_config;

use serde::{Deserialize, Serialize};

use crate::repository::Repository;

// Move the Add parameters to a separate struct that can be both Clap and Serde
#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd, clap::Parser)]
#[serde(rename_all = "lowercase")]
pub struct GroupConfig {
    /// Name of the group to add
    pub name: String,

    /// Include repository in the group if paths exists
    #[arg(long)]
    pub exists: Vec<PathBuf>,
    // Optional path for the group
    // #[arg(long)]
    // pub path: Option<PathBuf>,

    // List of branches to include in this group
    // #[arg(long)]
    // pub branch: Vec<String>,
}

#[derive(clap::Subcommand)]
#[command(about = "Add/remove virtual group")]
pub enum GroupCommand {
    /// Add a new group
    // #[derive(Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd, clap::Parser)]
    // #[serde(rename_all = "lowercase")]
    Add(GroupConfig),

    /// Remove an existing group
    Remove {
        /// Name of the group to remove
        name: String,
    },

    /// List all groups
    List,
}

/// Group used in lock file
// Eq, Ord and friends are needed to order the list of repositories
#[derive(Deserialize, Serialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Group {
    pub name: String,
    pub repositories: Vec<Repository>,
}
