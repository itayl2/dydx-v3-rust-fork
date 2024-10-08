use std::fmt::Debug;
use std::sync::Arc;
use super::super::types_v4::*;
use super::super::{ResponseError, Result};
use http::{Method, StatusCode};
use serde::Deserialize;
use serde::Serialize;
use serde_json::*;
use std::time::Duration;
use backon::Retryable;
use crate::retry::{ExponentialBuilderHelperGet, ErrorFn};

#[derive(Debug, Clone)]
pub struct Private<'a> {
    client: reqwest::Client,
    host: &'a str,
    internal_host: &'a str,
    eth_address: &'a str,
    subaccount_number: &'a str,
    error_handler: Option<ErrorFn>,
    retry_backoff_getter: Arc<dyn ExponentialBuilderHelperGet>
}

impl<'a> Private<'a> {
    pub fn new(
        host: &'a str,
        internal_host: &'a str,
        eth_address: &'a str,
        subaccount_number: &'a str,
        api_timeout: u64,
        error_handler: Option<ErrorFn>,
        retry_backoff_getter: Arc<dyn ExponentialBuilderHelperGet>,
    ) -> Arc<Private<'a>> {
        Arc::new(Private {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(api_timeout))
                .build()
                .expect("Client::new()"),
            host,
            eth_address,
            subaccount_number,
            internal_host,
            error_handler,
            retry_backoff_getter,
        })
    }

    pub async fn get_account(&self) -> Result<SubaccountResponseObject> {
        let path = format!("addresses/{}/subaccountNumber/{}", self.eth_address, self.subaccount_number);
        let response = self
            .retry_wrapper(path.as_str(), Vec::new(), json!({}), Some("get_account"))
            .await;
        response
    }

    pub async fn get_positions(
        &self,
        market: Option<&str>,
        status: Option<&str>,
        limit: Option<&str>,
        created_before_or_at_height: Option<&str>,
        created_before_or_at: Option<&str>,
    ) -> Result<PerpetualPositionResponse> {
        let mut parameters = Vec::new();
        parameters.push(("address", self.eth_address));
        parameters.push(("subaccountNumber", self.subaccount_number));
        
        if let Some(local_var) = market {
            parameters.push(("market", local_var));
        }
        if let Some(local_var) = status {
            parameters.push(("status", local_var));
        }
        if let Some(local_var) = limit {
            parameters.push(("limit", local_var));
        }
        if let Some(local_var) = created_before_or_at {
            parameters.push(("createdBeforeOrAt", local_var));
        }
        if let Some(local_var) = created_before_or_at_height {
            parameters.push(("createdBeforeOrAtHeight", local_var));
        }
        let response = self
            .retry_wrapper("perpetualPositions", parameters, json!({}), Some("get_positions"))
            .await;
        response
    }

    pub async fn create_order(&self, user_params: ApiOrderParams) -> Result<InternalApiResponse> {
        let response = self
            .internal_request("orders", Method::POST, Vec::new(), user_params)
            .await;
        response
    }

    pub async fn cancel_order(&self, market: &str, client_id: &str, good_til_block_time: i64) -> Result<InternalApiResponse> {
        let response = self
            .internal_request("cancel_order", Method::DELETE, Vec::new(), json!({
                "market": market,
                "client_id": client_id,
                "good_til_block_time": good_til_block_time,
            }))
            .await;
        response
    }

    pub async fn get_orders(
        &self,
        ticker: Option<&str>,
        status: Option<&str>,
        side: Option<&str>,
        type_field: Option<&str>,
        limit: Option<&str>,
        good_til_block_before_or_at: Option<&str>,
        good_til_block_time_before_or_at: Option<&str>,
        return_latest_orders: Option<&str>,
    ) -> Result<OrdersResponse> {
        let mut parameters = Vec::new();
        parameters.push(("address", self.eth_address));
        parameters.push(("subaccountNumber", self.subaccount_number));

        if let Some(local_var) = ticker {
            parameters.push(("ticker", local_var));
        }
        if let Some(local_var) = status {
            parameters.push(("status", local_var));
        }
        if let Some(local_var) = side {
            parameters.push(("side", local_var));
        }
        if let Some(local_var) = type_field {
            parameters.push(("type", local_var));
        }
        if let Some(local_var) = limit {
            parameters.push(("limit", local_var));
        }
        if let Some(local_var) = good_til_block_before_or_at {
            parameters.push(("goodTilBlockBeforeOrAt", local_var));
        }
        if let Some(local_var) = good_til_block_time_before_or_at {
            parameters.push(("goodTilBlockTimeBeforeOrAt", local_var));
        }
        if let Some(local_var) = return_latest_orders {
            parameters.push(("returnLatestOrders", local_var));
        }
        let response = self
            .retry_wrapper("orders", Vec::new(), json!({}), Some("get_orders"))
            .await;
        response
    }

    pub async fn get_order_by_id(&self, id: &str) -> Result<OrderResponse> {
        let path = format!("orders/{}", id);
        let response = self
            .retry_wrapper(path.as_str(), Vec::new(), json!({}), Some("get_order_by_id"))
            .await;
        response
    }

    pub async fn get_fills(
        &self,
        market: Option<&str>,
        market_type: Option<&str>,
        limit: Option<&str>,
        created_before_or_at_height: Option<&str>,
        created_before_or_at: Option<&str>,
    ) -> Result<FillResponse> {
        let mut parameters = Vec::new();
        parameters.push(("address", self.eth_address));
        parameters.push(("subaccountNumber", self.subaccount_number));

        if let Some(local_var) = market {
            parameters.push(("market", local_var));
        }
        if let Some(local_var) = market_type {
            parameters.push(("marketType", local_var));
        }
        if let Some(local_var) = limit {
            parameters.push(("limit", local_var));
        }
        if let Some(local_var) = created_before_or_at_height {
            parameters.push(("createdBeforeOrAtHeight", local_var));
        }
        if let Some(local_var) = created_before_or_at {
            parameters.push(("createdBeforeOrAt", local_var));
        }

        let response = self
            .retry_wrapper("fills", parameters, json!({}), Some("get_fills"))
            .await;
        response
    }

    async fn put(&self, path: &str) -> Result<StatusCode> {
        let url = format!("{}/v3/{}", &self.host, path);
        let req_builder = self.client.put(url);
        let result = req_builder.send().await?;
        Ok(result.status())
    }

    pub fn retry_notify(&self, operation_name: &str, error: &Box<dyn std::error::Error>, delay: Duration) {
        match self.error_handler {
            Some(ref error_handler) => {
                error_handler(operation_name, error, delay);
            }
            None => {
                eprintln!("Error fetching data from dYdX API::{operation_name}: {error:?}, retrying in {delay:?}");
            }
        }
    }

    async fn retry_wrapper<T: for<'de> Deserialize<'de>, V: Serialize + Clone>(
        &self,
        path: &str,
        parameters: Vec<(&str, &str)>,
        data: V,
        retry_snippet: Option<&str>,
    ) -> Result<T> {
        let retry_snippet = match retry_snippet {
            Some(local_var) => local_var,
            None => return self.request(path, Method::GET, parameters, data).await,
        };
        let backoff = self.retry_backoff_getter.get(retry_snippet);

        // println!("Will use backoff: {backoff:?} for {retry_snippet:?}");
        let closure = || async { self.request(path, Method::GET, parameters.clone(), data.clone()).await };
        let result = closure
            .retry(backoff)
            .notify(|err: &Box<dyn std::error::Error>, dur: Duration| {
                self.retry_notify(retry_snippet, err, dur);
            })
            .await;
        result
    }

    async fn internal_request<T: for<'de> Deserialize<'de>, V: Serialize>(
        &self,
        path: &str,
        method: Method,
        parameters: Vec<(&str, &str)>,
        data: V,
    ) -> Result<T> {
        let json = to_string(&data).unwrap();
        let url = format!("{}/{}", &self.internal_host, path);

        let req_builder = match method {
            Method::GET => self.client.get(url),
            Method::POST => self.client.post(url),
            Method::PUT => self.client.put(url),
            Method::DELETE => self.client.delete(url),
            _ => self.client.get(url),
        };

        let req_builder = req_builder
            .query(&parameters);

        let req_builder = if json != "{}" {
            req_builder.json(&data)
        } else {
            req_builder
        };
        let response = req_builder.send().await;

        match response {
            Ok(response) => match response.status() {
                StatusCode::OK | StatusCode::CREATED => {
                    // return Ok(response.json::<T>().await.unwrap())
                    return match response.json::<T>().await {
                        Ok(r) => Ok(r),
                        Err(e) => Err(Box::new(e)),
                    };
                }
                _ => {
                    let error = ResponseError {
                        code: response.status().to_string(),
                        // message: response.text().await.unwrap(),
                        message: response.text().await.unwrap_or_else(|e| e.to_string()),
                    };
                    return Err(Box::new(error));
                }
            },
            Err(err) => {
                return Err(Box::new(err));
            }
        };
    }

    async fn request<T: for<'de> Deserialize<'de>, V: Serialize>(
        &self,
        path: &str,
        method: Method,
        parameters: Vec<(&str, &str)>,
        data: V,
    ) -> Result<T> {
        let json = to_string(&data).unwrap();
        let url = format!("{}/v4/{}", &self.host, path);

        let req_builder = match method {
            Method::GET => self.client.get(url),
            Method::POST => self.client.post(url),
            Method::PUT => self.client.put(url),
            Method::DELETE => self.client.delete(url),
            _ => self.client.get(url),
        };

        let req_builder = req_builder
            .query(&parameters);

        let req_builder = if json != "{}" {
            req_builder.json(&data)
        } else {
            req_builder
        };
        let response = req_builder.send().await;

        match response {
            Ok(response) => match response.status() {
                StatusCode::OK | StatusCode::CREATED => {
                    // return Ok(response.json::<T>().await.unwrap())
                    return match response.json::<T>().await {
                        Ok(r) => Ok(r),
                        Err(e) => Err(Box::new(e)),
                    };
                }
                _ => {
                    let error = ResponseError {
                        code: response.status().to_string(),
                        // message: response.text().await.unwrap(),
                        message: response.text().await.unwrap_or_else(|e| e.to_string()),
                    };
                    return Err(Box::new(error));
                }
            },
            Err(err) => {
                return Err(Box::new(err));
            }
        };
    }
}
