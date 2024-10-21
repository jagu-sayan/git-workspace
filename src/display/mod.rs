use crate::repository::Repository;
use anyhow::Result;
use std::{str::FromStr, sync::Arc};

mod progress_bar;
mod table;

pub use progress_bar::ProgressBarDisplay;
pub use table::TableDisplay;

pub trait Display {
    fn init(&self, total: u64) -> Result<()>;
    fn create_step(&self, repo: &Repository) -> Result<()>;
    fn finish_step(&self, repo: &Repository) -> Result<()>;
    fn set_message(&self, message: String) -> Result<()>;
    fn finish(&self) -> Result<()>;
}

#[derive(clap::Parser, Clone)]
pub enum DisplayType {
    ProgressBar,
    Table,
}

impl DisplayType {
    pub fn create_display(self) -> Arc<dyn Display + Sync + Send> {
        match self {
            DisplayType::ProgressBar => Arc::new(ProgressBarDisplay::new()),
            DisplayType::Table => Arc::new(TableDisplay::new()),
        }
    }
}

impl FromStr for DisplayType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "progress" => Ok(DisplayType::ProgressBar),
            "table" => Ok(DisplayType::Table),
            _ => Err(format!("Invalid display type: {}", s)),
        }
    }
}
