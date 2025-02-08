use console::style;
use json::JsonDisplay;
use progress::ProgressDisplay;
use serde::Serialize;
use simple::SimpleDisplay;
use std::sync::Arc;

pub mod json;
pub mod progress;
pub mod simple;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OperationStatus {
    Success,
    Error,
}

impl std::fmt::Display for OperationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationStatus::Success => write!(f, "{}", style("✓").green()),
            OperationStatus::Error => write!(f, "{}", style("✗").red()),
            // OperationStatus::Skipped => write!(f, "{}", style("-").yellow()),
        }
    }
}

pub trait DisplayableResult: Serialize {
    fn is_success(&self) -> bool;
    // fn get_name(&self) -> &str;
    // fn get_message(&self) -> &str;
}

pub trait Display<R: DisplayableResult>: Send + Sync {
    /// Initialize the display with a total count of operations to be processed.
    fn init(&self, total: usize);

    /// Increment progress counter and update display
    fn inc_progress(&self);

    /// Display message
    fn show_message(&self, message: &str);

    /// Show final results of all operations
    fn show_results(&self, label: &str, results: &[R]);

    /// Clean up and finalize display output
    fn finish(&self);
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    #[value(name = "progress")]
    Progress,
    #[value(name = "json")]
    Json,
    #[value(name = "simple")]
    Simple,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Progress
    }
}

pub type DisplayRef<R> = Arc<dyn Display<R> + Sync + Send>;

impl OutputFormat {
    pub fn create_display<R: DisplayableResult + 'static>(self) -> DisplayRef<R> {
        match self {
            OutputFormat::Progress => Arc::new(ProgressDisplay::new()),
            OutputFormat::Json => Arc::new(JsonDisplay::new()),
            OutputFormat::Simple => Arc::new(SimpleDisplay::new()),
        }
    }
}
