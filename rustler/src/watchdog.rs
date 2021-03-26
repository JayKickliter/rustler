use std::{ops::Drop, time::Instant};

/// Barks when it lives longer than 1 ms.
///
/// It helps to determine if a NIF needs to be marked dirty.
pub struct Watchdog(Instant, &'static str, u32);

impl Watchdog {
    pub fn new(name: &'static str, arity: u32) -> Self {
        Self(Instant::now(), name, arity)
    }
}

impl Drop for Watchdog {
    fn drop(&mut self) {
        let ms = self.0.elapsed().as_millis();
        if ms > 1 {
            eprintln!("{}/{} ran for {} ms. Should be marked dirty.\r", self.1, self.2, ms);
        }
    }
}
