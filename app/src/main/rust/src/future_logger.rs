/**
    This file is a solution to have LogError which supports newest `log` crate.
    `android_logger` crate requires newest `log` crate, so I am having clash.
    Port this file was easier solution.
*/

use std::fmt::Display;

use futures::{Async, Future};
use log::Level;
use void::Void;

/// Wraps a future which returns `()` and logs its error if it fails. `LogError` itself cannot fail
/// and will always resolve to `()`.
pub struct LogError<F> {
    future: F,
    level: Level,
    description: &'static str,
}

impl<F> LogError<F> {
    pub fn new(
        future: F,
        level: Level,
        description: &'static str,
    ) -> LogError<F> {
        LogError {
            future,
            level,
            description,
        }
    }
}

impl<F> Future for LogError<F>
    where
        F: Future<Item=()>,
        F::Error: Display,
{
    type Item = ();
    type Error = Void;

    fn poll(&mut self) -> Result<Async<()>, Void> {
        match self.future.poll() {
            Ok(x) => Ok(x),
            Err(e) => {
                log!(self.level, "{}: {}", self.description, e);
                Ok(Async::Ready(()))
            }
        }
    }
}

