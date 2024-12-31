use crate::repository::Repository;
use anyhow::Result;
use std::{str::FromStr, sync::Arc};
use structopt::StructOpt;

mod progress_bar;
mod simple;
mod table;

pub use progress_bar::ProgressBarDisplay;
pub use simple::SimpleDisplay;
pub use table::TableDisplay;

pub trait Display {
    fn init(&self, total: u64) -> Result<()>;
    fn create_step(&self, repo: &Repository) -> Result<()>;
    fn finish_step(&self, repo: &Repository) -> Result<()>;
    fn set_message(&self, message: String) -> Result<()>;
    fn finish(&self) -> Result<()>;
}

#[derive(StructOpt)]
pub enum DisplayType {
    ProgressBar,
    Simple,
    Table,
}

impl DisplayType {
    pub fn create_display(self) -> Arc<dyn Display + Sync + Send> {
        match self {
            DisplayType::ProgressBar => Arc::new(ProgressBarDisplay::new()),
            DisplayType::Simple => Arc::new(SimpleDisplay::new()),
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
