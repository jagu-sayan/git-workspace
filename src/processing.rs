use crate::display::{Display, DisplayRef, DisplayableResult};
use anyhow::Result;
// use rand::Rng;
use rayon::{prelude::*, ThreadPool};
use std::sync::Arc;

pub trait Identifiable {
    fn name(&self) -> String;
}

/// Take any number of T and apply `f` on each one.
/// This method takes care of displaying progress bars and displaying
/// any errors that may arise.
pub struct ParallelTaskProcessing<T, R> {
    task_name: String,
    items: Vec<T>,
    thread_pool: ThreadPool,
    display: Arc<dyn Display<R>>,
}

impl<T, R> ParallelTaskProcessing<T, R>
where
    T: Send + Sync + Identifiable,
    R: Send + DisplayableResult,
{
    pub fn new(task_name: String, items: Vec<T>, threads: usize, display: DisplayRef<R>) -> Self {
        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .expect("to create thread pool");
        Self {
            task_name,
            items,
            thread_pool,
            display,
        }
    }

    pub fn map<F>(&self, operation: F) -> Vec<R>
    where
        F: Fn(&T) -> Result<R> + Send + Sync,
    {
        self.thread_pool.install(|| {
            self.items
                .par_iter()
                .filter_map(|item| operation(item).ok())
                .collect()
        })
    }

    pub fn map_with_display<F>(&self, operation: F) -> Vec<R>
    where
        F: Fn(&T, &dyn Display<R>) -> Result<R> + Send + Sync,
    {
        let total = self.items.len();

        self.display.init(total);

        let results = self.thread_pool.install(|| {
            let results = self
                .items
                .par_iter()
                .map(|item| {
                    // self.display
                    //     .show_message(&format!("Starting {}", item.name()));

                    let result = operation(item, self.display.as_ref());
                    // let mut rng = rand::thread_rng();
                    // std::thread::sleep(std::time::Duration::from_millis(
                    //     rng.gen_range(1000..10000),
                    // ));

                    // self.display
                    //     .show_message(&format!("Finished {}", item.name()));
                    self.display.inc_progress();

                    result
                })
                .filter_map(Result::ok)
                .collect::<Vec<_>>();

            self.display.finish();
            self.display.show_results(&self.task_name, &results);
            results
        });

        results
    }

    // pub fn map_display_sync<F, R>(&self, operation: F) -> Vec<R>
    // where
    //     F: Fn(&T, &dyn Display) -> Result<R>,
    //     R: Send,
    // {
    //     let total = self.items.len();
    //     self.display.init(total);

    //     let results = self
    //         .items
    //         .iter()
    //         .map(|item| {
    //             // self.display
    //             //     .show_message(&format!("Starting {}", item.name()));

    //             let result = operation(item, self.display.as_ref());
    //             let op_result = match &result {
    //                 Ok(_) => OperationResult {
    //                     name: item.name(),
    //                     status: OperationStatus::Success,
    //                     message: "Success".to_string(),
    //                 },
    //                 Err(e) => OperationResult {
    //                     name: item.name(),
    //                     status: OperationStatus::Error,
    //                     message: e.to_string(),
    //                 },
    //             };

    //             // self.display
    //             //     .show_message(&format!("Finished {}", item.name()));
    //             self.display.inc_progress();

    //             (op_result, result)
    //         })
    //         .collect::<Vec<_>>();

    //     let (operation_results, items): (Vec<_>, Vec<_>) = results
    //         .into_iter()
    //         .filter_map(|(op, result)| result.ok().map(|item| (op, item)))
    //         .unzip();

    //     self.display.finish();
    //     self.display
    //         .show_results(&self.task_name, &operation_results);

    //     items
    // }

    // pub fn flat_map_with_display<F, R>(&self, f: F) -> Vec<R>
    // where
    //     F: Fn(&T, &dyn Display) -> Result<R> + Send + Sync,
    //     R: Send + IntoIterator,
    // {
    //     self.map_with_display(f).into_iter().flatten().collect()
    // }

    pub fn filter<F>(&mut self, predicate: F) -> &mut Self
    where
        F: Fn(&T) -> bool,
    {
        self.items.retain(predicate);
        self
    }

    pub fn get_items(&self) -> &[T] {
        &self.items
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
