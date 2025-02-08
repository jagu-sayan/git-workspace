use super::read_lock_file;
use crate::{
    display::{DisplayableResult, OutputFormat},
    processing::ParallelTaskProcessing,
};
use serde::Serialize;
use std::path::Path;

#[derive(Serialize)]
pub struct ListResult {
    repo: String,
}

impl DisplayableResult for ListResult {
    fn is_success(&self) -> bool {
        true
    }

    // fn get_name(&self) -> &str {
    //     &self.repo_name
    // }

    // fn get_message(&self) -> &str {
    //     self.output.as_str()
    // }
}

/// List the contents of our workspace
pub fn list(workspace: &Path, full: bool, output: OutputFormat) -> anyhow::Result<()> {
    let repositories = read_lock_file(workspace)?;
    let display = output.create_display();
    let mut processor = ParallelTaskProcessing::new("list".to_string(), repositories, 1, display);
    processor
        .filter(|r| r.exists(workspace))
        .map_with_display(|repo, display| {
            let path = repo.get_path(workspace).unwrap();
            let message = if full {
                format!("{}", path.display())
            } else {
                format!("{}", repo.name())
            };
            display.show_message(&message);
            Ok(ListResult { repo: message })
        });

    Ok(())
}
