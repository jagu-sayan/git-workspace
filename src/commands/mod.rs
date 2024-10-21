pub mod add_provider;
pub mod archive;
pub mod fetch;
pub mod list;
pub mod lock;
pub mod run;
pub mod switch_and_pull;
pub mod update;

pub use add_provider::add_provider_to_config;
pub use archive::archive;
pub use fetch::fetch;
pub use list::list;
pub use lock::lock;
pub use run::execute_cmd;
pub use switch_and_pull::pull_all_repositories;
pub use update::update;

use crate::display::Display;
use crate::processor::RepositoryProcessor;
use crate::repository::Repository;
use anyhow::{anyhow, Context, Result};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use walkdir::WalkDir;

/// Take any number of repositories and apply `f` on each one on multiple threads.
/// This method takes care of displaying result and any errors that may arise.
pub fn map_repositories<F>(
    repositories: &[Repository],
    threads: usize,
    f: F,
    display: Arc<dyn Display + Send + Sync>,
) -> Result<()>
where
    F: Fn(&Repository) -> Result<()> + Send + Sync,
{
    display.init(repositories.len() as u64)?;
    let processor = RepositoryProcessor::new(threads);
    let errors = processor.process(repositories, |repo| {
        display.create_step(repo)?;
        let result = f(repo);
        display.finish_step(repo)?;
        result
    })?;
    display.finish()?;

    // Print out each repository that failed to run.
    if !errors.is_empty() {
        eprintln!("{} repositories failed:", errors.len());
        for (repo, error) in errors {
            eprintln!("{}:", repo.name());
            error
                .chain()
                .for_each(|cause| eprintln!("because: {}", cause));
        }
    }
    // Handle and display errors
    // let errors: Vec<_> = results.into_iter().filter_map(Result::err).collect();
    // if !errors.is_empty() {
    //     eprintln!("{} repositories failed:", errors.len());
    //     for error in errors {
    //         eprintln!("Error: {}", error);
    //     }
    // }

    Ok(())
}

/// Find all projects that have been archived or deleted on our providers
pub fn get_all_repositories_to_archive(
    workspace: &Path,
    repositories: Vec<Repository>,
) -> anyhow::Result<Vec<(PathBuf, PathBuf)>> {
    // The logic here is as follows:
    // 1. Iterate through all directories. If it's a "safe" directory (one that contains a project
    //    in our lockfile), we skip it entirely.
    // 2. If the directory is not, and contains a `.git` directory, then we mark it for archival and
    //    skip processing.
    // This assumes nobody deletes a .git directory in one of their projects.

    // Windows doesn't like .archive.
    let archive_directory = if cfg!(windows) {
        workspace.join("_archive")
    } else {
        workspace.join(".archive")
    };

    // Create a set of all repository paths that currently exist.
    let mut repository_paths: HashSet<PathBuf> = repositories
        .iter()
        .filter(|r| r.exists(workspace))
        .map(|r| r.get_path(workspace))
        .filter_map(Result::ok)
        .collect();

    // If the archive directory does not exist then we create it
    if !archive_directory.exists() {
        fs_extra::dir::create(&archive_directory, false).with_context(|| {
            format!(
                "Error creating archive directory {}",
                archive_directory.display()
            )
        })?;
    }

    // Make sure we add our archive directory to the set of repository paths. This ensures that
    // it's not traversed below!
    repository_paths.insert(
        archive_directory
            .canonicalize()
            .with_context(|| "Error canoncalizing archive directory")?,
    );

    let mut to_archive = Vec::new();
    let mut it = WalkDir::new(workspace).into_iter();

    // Waldir provides a `filter_entry` method, but I couldn't work out how to use it
    // correctly here. So we just roll our own loop:
    loop {
        // Find the next directory. This can throw an error, in which case we bail out.
        // Perhaps we shouldn't bail here?
        let entry = match it.next() {
            None => break,
            Some(Err(err)) => return Err(anyhow!("Error iterating through directory: {}", err)),
            Some(Ok(entry)) => entry,
        };
        // If the current path is in the set of repository paths then we skip processing it entirely.
        if repository_paths.contains(entry.path()) {
            it.skip_current_dir();
            continue;
        }
        // If the entry has a .git directory inside it then we add it to the `to_archive` list
        // and skip the current directory.
        if entry.path().join(".git").is_dir() {
            let path = entry.path();
            // Find the relative path of the directory from the workspace. So if you have something
            // like `workspace/github/repo-name`, it will be `github/repo-name`.
            let relative_dir = path.strip_prefix(workspace).with_context(|| {
                format!(
                    "Failed to strip the prefix '{}' from {}",
                    workspace.display(),
                    path.display()
                )
            })?;
            // Join the relative directory (`github/repo-name`) with the archive directory.
            let to_dir = archive_directory.join(relative_dir);
            to_archive.push((path.to_path_buf(), to_dir));
            it.skip_current_dir();
            continue;
        }
    }

    Ok(to_archive)
}
