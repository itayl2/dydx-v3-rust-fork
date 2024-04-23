use std::fmt::Debug;
use self::types::ApiKeyCredentials;

pub use super::types;
use crate::modules::eth_private::EthPrivate;
use crate::modules::onboarding::Onboarding;
use crate::modules::private::Private;
use crate::modules::public::Public;
use crate::retry::{ErrorFn, ExponentialBuilderHelperGet, FallbackBackoffGetter, NoBackoffGetter};

#[derive(Debug)]
pub struct ClientOptions<'a> {
    pub network_id: Option<usize>,
    pub api_timeout: Option<u64>,
    pub api_key_credentials: Option<ApiKeyCredentials<'a>>,
    pub stark_private_key: Option<&'a str>,
    pub eth_private_key: Option<&'a str>,
    pub public_error_handler: Option<ErrorFn>, // Correct use of `dyn`
    pub private_error_handler: Option<ErrorFn>, // Correct use of `dyn`
    pub public_backoff_getter: Option<Box<dyn ExponentialBuilderHelperGet>>,
    pub private_backoff_getter: Option<Box<dyn ExponentialBuilderHelperGet>>,
}

#[readonly::make]
#[derive(Debug, Clone)]
pub struct DydxClient<'a> {
    #[readonly]
    pub api_timeout: Option<u64>,
    pub public: Public<'a>,
    pub private: Option<Box<Private<'a>>>,
    pub eth_private: Option<EthPrivate<'a>>,
    pub onboarding: Option<Onboarding<'a>>,
}

impl<'a> DydxClient<'a> {
    pub fn new(host: &'a str, mut _options: ClientOptions<'a>) -> Self {
        let mut _options = _options;
        let network_id = _options.network_id.unwrap_or(1);
        let api_timeout = _options.api_timeout.unwrap_or(10);
        DydxClient {
            api_timeout: None,

            public: Public::new(host, api_timeout, _options.public_error_handler, _options.public_backoff_getter.unwrap_or(DydxClient::get_fallback_backoff_getter())),
            private: match _options.api_key_credentials {
                Some(v) => Some(Private::new(
                    host,
                    network_id,
                    api_timeout,
                    v,
                    _options.stark_private_key,
                    _options.private_error_handler,
                    _options.private_backoff_getter.unwrap_or(DydxClient::get_fallback_backoff_getter()),
                )),
                None => None,
            },
            eth_private: match _options.eth_private_key {
                Some(v) => Some(EthPrivate::new(
                    host, network_id, api_timeout,v,
                )),
                None => None,
            },
            onboarding: match _options.eth_private_key {
                Some(r) => Some(Onboarding::new(host, network_id, api_timeout, r)),
                None => None,
            },
        }
    }

    pub fn get_fallback_backoff_getter() -> Box<FallbackBackoffGetter> {
        Box::new(FallbackBackoffGetter::default())
    }

    pub fn get_fallback_backoff_getter_with_args(
        factor: f32,
        min_delay_ms: u64,
        max_delay_ms: u64,
        max_times: usize,
    ) -> Box<FallbackBackoffGetter> {
        Box::new(FallbackBackoffGetter::new(factor, min_delay_ms, max_delay_ms, max_times))
    }

    pub fn get_no_backoff_getter() -> Box<NoBackoffGetter> {
        Box::new(NoBackoffGetter::default())
    }
}
