use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;
use super::super::{ResponseError, Result};
use http::StatusCode;
use serde::Deserialize;
use std::time::Duration;
use backon::Retryable;
use crate::retry::{ExponentialBuilderHelperGet, ErrorFn};
use crate::types_v4::PerpetualMarketResponse;
pub use super::super::types::*;

#[readonly::make]
#[derive(Debug, Clone)]
pub struct Public<'a> {
    client: reqwest::Client,
    error_handler: Option<ErrorFn>,
    retry_backoff_getter: Arc<dyn ExponentialBuilderHelperGet>,
    pub host: &'a str,
}

impl<'a> Public<'a> {
    pub fn new(
        host: &str,
        api_timeout: u64,
        error_handler: Option<ErrorFn>,
        retry_backoff_getter: Arc<dyn ExponentialBuilderHelperGet>,
    ) -> Public {
        Public {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(api_timeout))
                .build()
                .expect("Client::new()"),
            host,
            error_handler,
            retry_backoff_getter,
        }
    }

    pub fn retry_notify(&self, operation_name: &str, error: &Box<dyn Error>, delay: Duration) {
        match self.error_handler {
            Some(ref error_handler) => {
                error_handler(operation_name, error, delay);
            }
            None => {
                eprintln!("Error fetching data from dYdX API::{operation_name}: {error:?}, retrying in {delay:?}");
            }
        }
    }

    pub async fn get_markets(&self, ticker: Option<&str>) -> Result<PerpetualMarketResponse> {
        let mut parameter = Vec::new();
        if let Some(local_var) = ticker {
            parameter.push(("ticker", local_var));
        }

        self.get_retry_wrapper("perpetualMarkets", parameter.clone(), Some("get_markets")).await
    }


    pub async fn get_orderbook(&self, market: &str) -> Result<OrderbookResponse> {
        let path = format!("orderbook/{}", market);
        let response = self.get_retry_wrapper(path.as_str(), Vec::new(), Some("get_orderbook")).await;
        response
    }

    pub async fn get_trades(
        &self,
        market: &str,
        starting_before_or_at: Option<&str>,
    ) -> Result<TradesResponse> {
        let path = format!("trades/{}", market);
        let mut parameter = Vec::new();
        if let Some(local_var) = starting_before_or_at {
            parameter.push(("startingBeforeOrAt", local_var));
        }

        let response = self.get_retry_wrapper(path.as_str(), parameter, Some("get_trades")).await;
        response
    }

    pub async fn get_fast_withdrawal(
        &self,
        credit_asset: Option<&str>,
        credit_amount: Option<&str>,
        debit_amount: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut parameter = Vec::new();
        if let Some(local_var) = credit_asset {
            parameter.push(("creditAsset", local_var));
        }
        if let Some(local_var) = credit_amount {
            parameter.push(("creditAmount", local_var));
        }
        if let Some(local_var) = debit_amount {
            parameter.push(("debitAmount", local_var));
        }
        let response: serde_json::Value = self.get_retry_wrapper("fast-withdrawals", parameter, Some("get_fast_withdrawal")).await?;
        Ok(response)
    }

    pub async fn get_stats(&self, market: &str, days: Option<&str>) -> Result<MarketStatsResponse> {
        let path = format!("stats/{}", market);
        let mut parameter = Vec::new();
        if let Some(local_var) = days {
            parameter.push(("days", local_var));
        }
        let response = self.get_retry_wrapper(path.as_str(), parameter, Some("get_stats")).await;
        response
    }

    pub async fn get_historical_funding(
        &self,
        market: &str,
        effective_before_or_at: Option<&str>,
    ) -> Result<HistoricalFundingResponse> {
        let path = format!("historical-funding/{}", market);
        let mut parameter = Vec::new();
        if let Some(local_var) = effective_before_or_at {
            parameter.push(("effectiveBeforeOrAt", local_var));
        }
        let response = self.get_retry_wrapper(path.as_str(), parameter, Some("get_historical_funding")).await;
        response
    }

    pub async fn get_candles(
        &self,
        market: &str,
        resolution: Option<&str>,
        from_iso: Option<&str>,
        to_iso: Option<&str>,
        limit: Option<&str>,
    ) -> Result<CandlesResponse> {
        let path = format!("candles/{}", market);
        let mut parameters = Vec::new();
        if let Some(local_var) = resolution {
            parameters.push(("resolution", local_var));
        }
        if let Some(local_var) = from_iso {
            parameters.push(("fromISO", local_var));
        }
        if let Some(local_var) = to_iso {
            parameters.push(("toISO", local_var));
        }
        if let Some(local_var) = limit {
            parameters.push(("limit", local_var));
        }

        let response = self.get_retry_wrapper(path.as_str(), parameters, Some("get_candles")).await;
        response
    }

    pub async fn get_config(&self) -> Result<ConfigResponse> {
        let response = self.get_retry_wrapper("config", Vec::new(), Some("get_config")).await;
        response
    }

    pub async fn check_if_user_exists(&self, ethereum_address: &str) -> Result<UserExistsResponse> {
        let parameters = vec![("ethereumAddress", ethereum_address)];
        let response = self.get_retry_wrapper("users/exists", parameters, Some("check_if_user_exists")).await;
        response
    }

    pub async fn check_if_username_exists(&self, username: &str) -> Result<UsernameExistsResponse> {
        let parameters = vec![("username", username)];
        let response = self.get_retry_wrapper("usernames", parameters, Some("check_if_username_exists")).await;
        response
    }

    pub async fn get_time(&self) -> Result<GetTimeResponse> {
        let response = self.get_retry_wrapper("time", Vec::new(), Some("get_time")).await;
        response
    }

    pub async fn get_leaderboard_pnls(
        &self,
        period: &str,
        starting_before_or_at: &str,
        sort_by: &str,
        limit: Option<&str>,
    ) -> Result<LeaderboardPnlResponse> {
        let mut parameters = vec![
            ("period", period),
            ("startingBeforeOrAt", starting_before_or_at),
            ("sortBy", sort_by),
        ];
        if let Some(local_var) = limit {
            parameters.push(("limit", local_var));
        }
        let response = self.get_retry_wrapper("leaderboard-pnl", parameters, Some("get_leaderboard_pnls")).await;
        response
    }

    pub async fn get_public_retroactive_mining_rewards(
        &self,
        ethereum_address: &str,
    ) -> Result<PublicRetroactiveMiningRewardsResponse> {
        let parameters = vec![("ethereumAddress", ethereum_address)];
        let response = self
            .get_retry_wrapper("rewards/public-retroactive-mining", parameters, Some("get_public_retroactive_mining_rewards"))
            .await;
        response
    }

    pub async fn get_currently_revealed_hedgies(&self) -> Result<CurrentlyRevealedHedgies> {
        let response = self.get_retry_wrapper("hedgies/current", Vec::new(), Some("get_currently_revealed_hedgies")).await;
        response
    }

    pub async fn get_historically_revealed_hedgies(
        &self,
        nft_reveal_type: &str,
        start: Option<&str>,
        end: Option<&str>,
    ) -> Result<HedgiePeriodResponse> {
        let mut parameters = Vec::new();
        parameters.push(("nftRevealType", nft_reveal_type));
        if let Some(local_var) = start {
            parameters.push(("start", local_var));
        }
        if let Some(local_var) = end {
            parameters.push(("end", local_var));
        }
        let response = self.get_retry_wrapper("hedgies/history", parameters, Some("get_historically_revealed_hedgies")).await;
        response
    }

    pub async fn get_insurance_fund_balance(&self) -> Result<InsuranceFundBalanceResponse> {
        let response = self.get_retry_wrapper("insurance-fund/balance", Vec::new(), Some("get_insurance_fund_balance")).await;
        response
    }

    pub async fn get_profile(&self, public_id: &str) -> Result<ProfilePublicResponse> {
        let path = format!("profile/{}", public_id);
        let response = self.get_retry_wrapper(path.as_str(), Vec::new(), Some("get_profile")).await;
        response
    }

    pub async fn verify_email(&self, token: &str) -> Result<StatusCode> {
        let param = vec![("token", token)];
        let response = self.put("emails/verify-email", &param).await;
        response
    }

    async fn get_retry_wrapper<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        parameters: Vec<(&str, &str)>,
        operation_name: Option<&str>,
    ) -> Result<T> {
        let retry_snippet = match operation_name {
            Some(local_var) => local_var,
            None => return self.get(path, parameters).await,
        };
        let backoff = self.retry_backoff_getter.get(retry_snippet);

        // println!("Will use backoff: {backoff:?} for {retry_snippet:?}");
        let closure = || async { self.get(path, parameters.clone()).await };
        let result = closure
            .retry(backoff)
            .notify(|err: &Box<dyn Error>, dur: Duration| {
                self.retry_notify(retry_snippet, err, dur);
            })
            .await;
        result
    }

    async fn get<S: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        parameters: Vec<(&str, &str)>,
    ) -> Result<S> {
        let url = format!("{}/v4/{}", &self.host, path);
        let req_builder = self.client.get(url.clone()).query(&parameters);
        // let another_req_builder = self.client.get(url.clone()).query(&parameters);
        // let another_response = another_req_builder.send().await;
        // let text_response = another_response.unwrap().text().await.unwrap();
        // println!("text_response: {text_response}");
        let response = req_builder.send().await;

        match response {
            Ok(response) => match response.status() {
                StatusCode::OK | StatusCode::CREATED => {
                    // return Ok(response.json::<S>().await.unwrap())
                    return match response.json::<S>().await {
                        Ok(r) => Ok(r),
                        Err(e) => Err(Box::new(e)),
                    };
                }
                _ => {
                    eprintln!("Error for url {url}");
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

    async fn put(&self, path: &str, parameters: &[(&str, &str)]) -> Result<StatusCode> {
        let url = format!("{}/v4/{}", &self.host, path);
        let req_builder = self.client.put(url).query(parameters);
        let result = req_builder.send().await?;
        Ok(result.status())
    }
}
