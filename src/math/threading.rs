use rayon::ThreadPool;
use rayon::ThreadPoolBuilder;
use std::cmp::max;
use std::sync::{Arc, OnceLock};

static LARGE_POOL: OnceLock<Arc<ThreadPool>> = OnceLock::new();

pub fn get_large_pool() -> Arc<ThreadPool> {
    let num_cpus = num_cpus::get() - 1;
    LARGE_POOL
        .get_or_init(|| {
            Arc::new(
                ThreadPoolBuilder::new()
                    // .stack_size(28 * 1024 * 1024)
                    .num_threads((num_cpus).max(1))
                    .build()
                    .expect("Failed to create large pool"),
            )
        })
        .clone()
}

pub fn get_small_pool() -> Arc<ThreadPool> {
    let num_cpus = num_cpus::get() / 2;
    LARGE_POOL
        .get_or_init(|| {
            Arc::new(
                ThreadPoolBuilder::new()
                    // .stack_size(28 * 1024 * 1024)
                    .num_threads((num_cpus).max(1))
                    .build()
                    .expect("Failed to create large pool"),
            )
        })
        .clone()
}
