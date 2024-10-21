use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use super::Display;
use crate::repository::Repository;
use anyhow::Result;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub struct ProgressBarDisplay {
    multi_progress: MultiProgress,
    total_bar: ProgressBar,
    step_bar: Arc<Mutex<Option<ProgressBar>>>,
}

impl ProgressBarDisplay {
    pub fn new() -> Self {
        let multi_progress = MultiProgress::new();
        let total_bar = multi_progress.add(ProgressBar::new(0));
        total_bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {percent}% [{wide_bar:.cyan/blue}] {pos}/{len} (ETA: {eta_precise})")
                .expect("Invalid template")
                .progress_chars("#>-"),
        );
        Self {
            multi_progress,
            total_bar,
            step_bar: Arc::new(Mutex::new(None)),
        }
    }
}

impl Display for ProgressBarDisplay {
    fn init(&self, total: u64) -> Result<()> {
        self.total_bar.inc_length(total);
        Ok(())
    }

    fn create_step(&self, repo: &Repository) -> Result<()> {
        let bar = self.multi_progress.add(ProgressBar::new_spinner());
        bar.set_message("waiting...");
        bar.enable_steady_tick(Duration::from_millis(500));
        let mut step_bar = self.step_bar.lock().unwrap();
        *step_bar = Some(bar);
        Ok(())
    }

    fn finish_step(&self) -> Result<()> {
        let mut step_bar = self.step_bar.lock().unwrap();
        if let Some(bar) = step_bar.take() {
            bar.finish_and_clear();
        }
        self.total_bar.inc(1);
        Ok(())
    }
    // let bar = self.multi_progress.add(ProgressBar::new_spinner());
    // bar.set_message("waiting...");
    // bar.enable_steady_tick(Duration::from_millis(500));
    // bar

    fn set_message(&self, message: String) -> Result<()> {
        self.total_bar.set_message(message);
        Ok(())
    }

    fn finish(&self) -> Result<()> {
        self.total_bar.finish_and_clear();
        Ok(())
    }
}
