use super::{Display, DisplayableResult};

pub struct JsonDisplay {}

impl JsonDisplay {
    pub fn new() -> Self {
        Self {}
    }
}
impl<T: DisplayableResult> Display<T> for JsonDisplay {
    // Not needed for JsonDisplay
    fn init(&self, _total: usize) {}
    fn show_message(&self, _message: &str) {}
    fn inc_progress(&self) {}
    fn finish(&self) {}

    fn show_results(&self, label: &str, results: &[T]) {
        let json_output = serde_json::json!({
            "command": label,
            "results": results,
        });
        println!("{}", serde_json::to_string_pretty(&json_output).unwrap());
        // serde_json::to_writer(std::io::stdout(), &results)
        //         .context("Failed to write final JSON output")?;
    }
}
