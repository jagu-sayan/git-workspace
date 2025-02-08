use super::{Display, DisplayableResult};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::{sync::Arc, time::Duration};

pub struct ProgressDisplay {
    bar: Arc<ProgressBar>,
    is_interactive: bool,
}

impl ProgressDisplay {
    pub fn new() -> Self {
        let bar = Arc::new(ProgressBar::new(16));
        let style = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {percent}% [{wide_bar:.cyan/blue}] {pos}/{len} (ETA: {eta_precise})\n{msg}")
            .expect("Invalid template")
            .progress_chars("#>-");
        bar.set_style(style);
        bar.enable_steady_tick(Duration::from_millis(10));

        Self {
            bar,
            is_interactive: console::user_attended(),
        }
    }
}

impl ProgressDisplay {
    fn get_results_message<T: DisplayableResult>(&self, label: &str, results: &[T]) -> String {
        let (success_count, error_count) = results.iter().fold((0, 0), |(s, e), result| {
            if result.is_success() {
                (s + 1, e)
            } else {
                (s, e + 1)
            }
        });
        format!(
            "{}: {} succeeded, {} failed",
            label,
            style(success_count).green(),
            style(error_count).red(),
        )
    }
}

impl<T: DisplayableResult> Display<T> for ProgressDisplay {
    fn init(&self, total: usize) {
        self.bar.set_length(total as u64);
    }

    fn show_message(&self, message: &str) {
        if !self.is_interactive {
            println!("{}", message);
        } else {
            self.bar.set_message(message.to_owned());
        }
    }

    fn inc_progress(&self) {
        self.bar.inc(1);
    }

    fn show_results(&self, label: &str, results: &[T]) {
        println!("{}", self.get_results_message(label, results));
    }

    fn finish(&self) {
        self.bar.finish_and_clear();
    }
}
