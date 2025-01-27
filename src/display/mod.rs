use console::style;
use json::JsonDisplay;
use progress::ProgressDisplay;
use serde::Serialize;
use std::sync::Arc;

pub mod json;
pub mod progress;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OperationStatus {
    Success,
    Error,
    Skipped,
}

impl std::fmt::Display for OperationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationStatus::Success => write!(f, "{}", style("✓").green()),
            OperationStatus::Error => write!(f, "{}", style("✗").red()),
            OperationStatus::Skipped => write!(f, "{}", style("-").yellow()),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct OperationResult {
    pub name: String,
    pub status: OperationStatus,
    pub message: String,
}

pub trait Display: Send + Sync {
    /// Initialize the display with a total count of operations to be processed.
    fn init(&self, total: usize);

    /// Increment progress counter and update display
    fn inc_progress(&self);

    // fn set_progress(&self, pos: usize);

    /// Display message
    fn show_message(&self, message: &str);

    /// Show final results of all operations
    fn show_results(&self, results: &[OperationResult]);

    /// Clean up and finalize display output
    fn finish(&self);
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    #[value(name = "progress")]
    Progress,
    #[value(name = "json")]
    Json,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Progress
    }
}

pub type DisplayRef = Arc<dyn Display + Sync + Send>;

impl OutputFormat {
    pub fn create_display(self) -> DisplayRef {
        match self {
            OutputFormat::Progress => Arc::new(ProgressDisplay::new()),
            OutputFormat::Json => Arc::new(JsonDisplay::new()),
        }
    }
}
