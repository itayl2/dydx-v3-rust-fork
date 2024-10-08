pub mod constants;
pub mod dydx_client;
pub mod error;
pub mod helper;
pub mod modules;
pub mod types;
pub mod retry;
pub mod types_v4;
pub mod error_with_send;

pub use dydx_client::ClientOptions;
pub use dydx_client::DydxClient;
pub use error::ResponseError;
use crate::error_with_send::DydxErrorWithSend;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type ResultWithSend<T> = std::result::Result<T, DydxErrorWithSend>;