use crate::repository::Repository;
use anyhow::Result;
use rayon::prelude::*;

pub struct RepositoryProcessor {
    threads: usize,
}

impl RepositoryProcessor {
    pub fn new(threads: usize) -> Self {
        Self { threads }
    }

    pub fn process<'a, F>(
        &self,
        repositories: &'a [Repository],
        f: F,
    ) -> Result<Vec<(&'a Repository, anyhow::Error)>>
    where
        F: Fn(&Repository) -> Result<()> + Send + Sync,
    {
        // Create our thread pool. We do this rather than use `.par_iter()` on any iterable as it
        // allows us to customize the number of threads.
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.threads)
            .build()?;

        let results: Vec<(&Repository, anyhow::Error)> = pool.install(|| {
            repositories
                .par_iter()
                .map(|repo| match f(repo) {
                    Ok(_) => Ok(()),
                    Err(e) => Err((repo, e)),
                })
                .filter_map(Result::err)
                .collect()
        });

        Ok(results)
    }
}
