use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::time::Duration;
use serde::Deserializer;
use backon::ExponentialBuilder;

pub trait ErrorHandler {
    fn notify(&self, operation_name: &str, error: &Box<dyn Error>, delay: Duration);
}

pub type ErrorFn = fn(operation_name: &str, error: &Box<dyn Error>, delay: Duration);


pub fn default_error_handler(operation_name: &str, error: &Box<dyn Error>, delay: Duration) {
    eprintln!("Error fetching data from dYdX API::{operation_name}: {error:?}, retrying in {delay:?}");
}

pub trait ExponentialBuilderHelperGet: Send + Sync + Debug {
    fn get(&self, key: &str) -> &ExponentialBuilder;
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

#[derive(Debug, Clone)]
pub struct FallbackBackoffGetter {
    backoff: ExponentialBuilder,
}

impl FallbackBackoffGetter {
    pub fn new(
        factor: f32,
        min_delay_ms: u64,
        max_delay_ms: u64,
        max_times: usize,
    ) -> Self {
        FallbackBackoffGetter {
            backoff: ExponentialBuilder::default()
                .with_factor(factor)
                .with_min_delay(Duration::from_millis(min_delay_ms))
                .with_max_delay(Duration::from_millis(max_delay_ms))
                .with_max_times(max_times),
        }
    }
}

impl Default for FallbackBackoffGetter {
    fn default() -> Self {
        FallbackBackoffGetter {
            backoff: ExponentialBuilder::default(),
        }
    }
}

impl ExponentialBuilderHelperGet for FallbackBackoffGetter {
    fn get(&self, _: &str) -> &ExponentialBuilder {
        &self.backoff
    }

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone)]
pub struct NoBackoffGetter {
    backoff: ExponentialBuilder,
}

impl NoBackoffGetter {
    pub fn new() -> Self {
        NoBackoffGetter {
            backoff: ExponentialBuilder::default().with_max_times(0),
        }
    }
}

impl Default for NoBackoffGetter {
    fn default() -> Self {
        NoBackoffGetter {
            backoff: ExponentialBuilder::default().with_max_times(0),
        }
    }
}

impl ExponentialBuilderHelperGet for NoBackoffGetter {
    fn get(&self, _: &str) -> &ExponentialBuilder {
        &self.backoff
    }

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
