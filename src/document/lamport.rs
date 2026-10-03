pub struct Lamport {
    counter: u64,
}

impl Lamport {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    /// Advances the clock past `request_time` and returns the new value.
    pub fn tick(&mut self, request_time: u64) -> u64 {
        self.counter = self.counter.max(request_time) + 1;
        self.counter
    }
}