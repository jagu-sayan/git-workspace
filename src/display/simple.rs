use super::{Display, DisplayableResult};

pub struct SimpleDisplay {}

impl SimpleDisplay {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: DisplayableResult> Display<T> for SimpleDisplay {
    // Not needed for SimpleDisplay
    fn init(&self, _total: usize) {}
    fn inc_progress(&self) {}
    fn show_results(&self, _label: &str, _results: &[T]) {}
    fn finish(&self) {}

    fn show_message(&self, message: &str) {
        println!("{}", message)
    }
}
