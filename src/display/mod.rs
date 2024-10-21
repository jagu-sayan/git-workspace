use crate::repository::Repository;
use anyhow::Result;

mod progress_bar;
mod table;

pub use progress_bar::ProgressBarDisplay;
pub use table::TableDisplay;

pub trait Display {
    fn init(&self, total: u64) -> Result<()>;
    fn create_step(&self, repo: &Repository) -> Result<()>;
    fn finish_step(&self) -> Result<()>;
    fn set_message(&self, message: String) -> Result<()>;
    fn finish(&self) -> Result<()>;
}
