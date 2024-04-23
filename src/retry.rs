use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::time::Duration;
use serde::{Deserialize, Serialize, Deserializer};
use serde_derive::{Deserialize as Deserialize_derive, Serialize as SerializeDerive};
use backon::ExponentialBuilder;

pub trait ErrorHandler {
    fn notify(&self, operation_name: &str, error: &Box<dyn Error>, delay: Duration);
}

pub type ErrorFn = fn(operation_name: &str, error: &Box<dyn Error>, delay: Duration);


pub fn default_error_handler(operation_name: &str, error: &Box<dyn Error>, delay: Duration) {
    eprintln!("Error fetching data from dYdX API::{operation_name}: {error:?}, retrying in {delay:?}");
}

pub trait ExponentialBuilderHelperGet {
    fn get(&self, key: &str) -> &ExponentialBuilder;
    fn clone_box(&self) -> Box<dyn ExponentialBuilderHelperGet>;
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl Clone for Box<dyn ExponentialBuilderHelperGet> {
    fn clone(&self) -> Box<dyn ExponentialBuilderHelperGet> {
        self.clone_box()
    }
}

impl Debug for Box<dyn ExponentialBuilderHelperGet> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
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

    fn clone_box(&self) -> Box<dyn ExponentialBuilderHelperGet> {
        Box::new(self.clone())
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

    fn clone_box(&self) -> Box<dyn ExponentialBuilderHelperGet> {
        Box::new(self.clone())
    }

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
