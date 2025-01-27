use super::{Display, OperationResult};
use serde::Serialize;

#[derive(Serialize)]
struct JsonResult {
    success_count: usize,
    error_count: usize,
    operations: Vec<OperationResult>,
}

pub struct JsonDisplay {}

impl JsonDisplay {
    pub fn new() -> Self {
        Self {}
    }
}

impl Display for JsonDisplay {
    fn init(&self, _total: usize) {}

    fn show_message(&self, _message: &str) {
        // No-op for JSON display
    }

    fn inc_progress(&self) {
        // No-op for JSON display
    }

    fn show_results(&self, results: &[OperationResult]) {
        let result = JsonResult {
            success_count: 1,
            error_count: 0,
            operations: results.to_owned(),
        };
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
        // serde_json::to_writer(std::io::stdout(), &results)
        //         .context("Failed to write final JSON output")?;
    }

    fn finish(&self) {}
}
