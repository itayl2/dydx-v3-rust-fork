use std::fmt;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DydxErrorWithSend {
    pub message: String,
}

impl DydxErrorWithSend {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

// so that we could create wrapper methods for methods of the dydx client and use this error to allow these methods to also be called from tokio spawn
unsafe impl Send for DydxErrorWithSend {}

impl fmt::Display for DydxErrorWithSend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DydxErrorWithSend {}

impl From<Box<dyn std::error::Error>> for DydxErrorWithSend {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        DydxErrorWithSend::new(&format!("{err:?}"))
    }
}

impl From<anyhow::Error> for DydxErrorWithSend {
    fn from(err: anyhow::Error) -> Self {
        DydxErrorWithSend::new(&err.to_string())
    }
}