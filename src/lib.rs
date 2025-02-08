extern crate atomic_counter;
extern crate clap;
extern crate console;
#[cfg(unix)]
extern crate expanduser;
extern crate fs_extra;
extern crate graphql_client;
extern crate indicatif;
extern crate serde;
extern crate serde_json;
extern crate thiserror;
extern crate ureq;
extern crate walkdir;

pub mod commands;
pub mod config;
pub mod display;
pub mod lockfile;
pub mod processing;
pub mod providers;
pub mod repository;
pub mod utils;

#[derive(thiserror::Error, Debug)]
pub enum WorkspaceError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("Invalid repository path: {0}")]
    InvalidPath(String),

    #[error("Command execution failed: {0}")]
    CommandFailed(String),

    #[error("Repository error: {0}")]
    Repository(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Thread pool error: {0}")]
    ThreadPool(String),

    #[error("Lock file error: {0}")]
    Lockfile(String),
}

pub type WorkspaceResult<T> = Result<T, WorkspaceError>;
