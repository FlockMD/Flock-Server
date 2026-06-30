/* A thread-safe implementation of a Lamport clock */

use std::cmp::max;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct Lamport {
    counter: AtomicU64,
}

impl Lamport {
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
        }

    }
    pub fn tick(&self, request_time: u64) -> u64 {
        let mut current = self.counter.load(Ordering::Acquire);

        loop {
            let next = max(current, request_time) + 1;
            match self.counter.compare_exchange(
                current,
                next, 
                Ordering::AcqRel,
                Ordering::Acquire
            ) {
                Ok(_) => return next,
                Err(actual) => current = actual,
            }
        }
    }
}