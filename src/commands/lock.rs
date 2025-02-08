use crate::config::Config;
use crate::display::{DisplayableResult, OutputFormat};
use crate::lockfile::Lockfile;
use crate::processing::{Identifiable, ParallelTaskProcessing};
use crate::repository::Repository;
use anyhow::Context;
use serde::Serialize;
use std::path::Path;

#[derive(Serialize)]
pub struct LockResult {
    provider: String,
    repos: Option<Vec<Repository>>,
    // source: String,
    success: bool,
    error: Option<String>,
}

impl DisplayableResult for LockResult {
    fn is_success(&self) -> bool {
        self.success
    }

    // fn get_name(&self) -> &str {
    //     &self.name
    // }

    // fn get_message(&self) -> &str {
    //     self.source.as_str()
    // }
}

/// Update our lockfile
pub fn lock(workspace: &Path, output: OutputFormat) -> anyhow::Result<()> {
    // Read the configuration sources
    let config = Config::from_workspace(workspace)?;
    let sources = config
        .read()
        .with_context(|| "Error reading config files")?;

    // For each source, fetch repositories
    // println!("Fetching repositories...");
    let display = output.create_display();
    display.show_message(&format!("Updating lock file by fetching repositories from cloud git providers (Github/Gitlab/Gitea)"));
    let task_name = "lock".to_string();
    let _msg = "Fetching repositories from cloud git providers".to_string();
    let processor = ParallelTaskProcessing::new(task_name, sources.clone(), 8, display);
    let results =
        processor.map_with_display(|source, _display| match source.fetch_repositories() {
            Ok(repositories) => Ok(LockResult {
                provider: source.name(),
                repos: Some(repositories),
                // source: source.to_string(),
                success: true,
                error: None,
            }),
            Err(e) => Ok(LockResult {
                provider: source.name(),
                repos: None,
                // source: source.to_string(),
                success: false,
                error: Some(e.to_string()),
            }),
        });
    let mut all_repositories: Vec<Repository> = results
        .into_iter()
        .filter_map(|result| result.repos)
        .flatten()
        .collect();

    // let all_repositories: Vec<Repository> = all_repository_results.iter().collect::<anyhow::Result<Vec<Repository>>>()?;
    // We may have duplicated repositories here. Make sure they are unique based on the full path.
    all_repositories.sort();
    all_repositories.dedup();

    // Write the lockfile out
    let lockfile = Lockfile::new(workspace.join("workspace-lock.toml"));
    lockfile.write(&all_repositories)?;
    Ok(())
}
