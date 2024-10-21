use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use super::Display;
use crate::repository::Repository;
use anyhow::Result;
use atomic_counter::{AtomicCounter, RelaxedCounter};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub struct ProgressBarDisplay {
    multi_progress: MultiProgress,
    total_bar: ProgressBar,
    step_bar: Arc<Mutex<Option<ProgressBar>>>,
    counter: RelaxedCounter,
    pub is_attented: bool,
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
            // Use a counter here if there is no tty, to show a stream of progress messages rather than
            // a dynamic progress bar.
            counter: RelaxedCounter::new(1),
            // user_attended() means a tty is attached to the output.
            is_attented: console::user_attended(),
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
        if !self.is_attented {
            let total_repositories = self.total_bar.length().unwrap();
            let idx = self.counter.inc();
            println!("[{}/{}] Starting {}", idx, total_repositories, repo.name());
        }
        Ok(())
    }

    fn finish_step(&self, repo: &Repository) -> Result<()> {
        if !self.is_attented {
            let total_repositories = self.total_bar.length().unwrap();
            let idx = self.counter.get();
            println!("[{}/{}] Finished {}", idx, total_repositories, repo.name());
        }
        let mut step_bar = self.step_bar.lock().unwrap();
        if let Some(bar) = step_bar.take() {
            bar.finish_and_clear();
        }
        self.total_bar.inc(1);
        Ok(())
    }

    fn set_message(&self, message: String) -> Result<()> {
        self.total_bar.set_message(message);
        Ok(())
    }

    fn finish(&self) -> Result<()> {
        self.total_bar.finish_and_clear();
        Ok(())
    }
}
