use std::fmt::Debug;
use std::sync::Arc;

pub use super::types;
use crate::modules::private::Private;
use crate::modules::public::Public;
use crate::retry::{ErrorFn, ExponentialBuilderHelperGet, FallbackBackoffGetter, NoBackoffGetter};

#[derive(Debug)]
pub struct ClientOptions<'a> {
    pub network_id: Option<usize>,
    pub api_timeout: Option<u64>,
    pub eth_address: &'a str,
    pub subaccount_number: &'a str,
    pub public_error_handler: Option<ErrorFn>, // Correct use of `dyn`
    pub private_error_handler: Option<ErrorFn>, // Correct use of `dyn`
    pub public_backoff_getter: Option<Arc<dyn ExponentialBuilderHelperGet>>,
    pub private_backoff_getter: Option<Arc<dyn ExponentialBuilderHelperGet>>,
}

#[readonly::make]
#[derive(Debug, Clone)]
pub struct DydxClient<'a> {
    #[readonly]
    pub api_timeout: Option<u64>,
    pub public: Public<'a>,
    pub private: Option<Arc<Private<'a>>>,
}

impl DydxClient<'_> {
    pub fn new<'a>(host: &'a str, internal_host: &'a str, mut _options: ClientOptions<'a>) -> DydxClient<'a> {
        let mut _options = _options;
        let api_timeout = _options.api_timeout.unwrap_or(10);
        DydxClient {
            api_timeout: None,

            public: Public::new(host, api_timeout, _options.public_error_handler, _options.public_backoff_getter.unwrap_or(DydxClient::get_fallback_backoff_getter())),
            private: Some(Private::new(
                host,
                internal_host,
                _options.eth_address,
                _options.subaccount_number,
                api_timeout,
                _options.private_error_handler,
                _options.private_backoff_getter.unwrap_or(DydxClient::get_fallback_backoff_getter()),

            )),
        }
    }

    pub fn get_fallback_backoff_getter() -> Arc<FallbackBackoffGetter> {
        Arc::new(FallbackBackoffGetter::default())
    }

    pub fn get_fallback_backoff_getter_with_args(
        factor: f32,
        min_delay_ms: u64,
        max_delay_ms: u64,
        max_times: usize,
    ) -> Arc<FallbackBackoffGetter> {
        Arc::new(FallbackBackoffGetter::new(factor, min_delay_ms, max_delay_ms, max_times))
    }

    pub fn get_no_backoff_getter() -> Arc<NoBackoffGetter> {
        Arc::new(NoBackoffGetter::default())
    }
}
