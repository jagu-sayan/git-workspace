use super::{Display, OperationResult};
use crate::display::OperationStatus;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::{sync::Arc, time::Duration};

pub struct ProgressDisplay {
    // manager = Arc::new(MultiProgress::new());
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

impl Display for ProgressDisplay {
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

    // fn set_progress(&self, )

    fn show_results(&self, results: &[OperationResult]) {
        let mut success_count = 0;
        let mut error_count = 0;
        for result in results {
            match &result.status {
                OperationStatus::Success => {
                    success_count += 1;
                }
                OperationStatus::Error => {
                    error_count += 1;
                    eprintln!("{} {} - {}", result.status, result.name, result.message);
                }
                OperationStatus::Skipped => {}
            }
        }
        println!(
            "\nSummary: {} succeeded, {} failed",
            style(success_count).green(),
            style(error_count).red(),
        );
    }

    fn finish(&self) {
        self.bar.finish_and_clear();
    }
}
