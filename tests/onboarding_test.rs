macro_rules! b {
        ($e:expr) => {
                tokio_test::block_on($e)
        };
}

use dotenv::dotenv;
use dydx_v3_rust::helper::*;
use dydx_v3_rust::structs::*;
use dydx_v3_rust::ClientOptions;
#[cfg(test)]
use dydx_v3_rust::DydxClient;
// use secp256k1::SecretKey;
use std::env;
use std::str::FromStr;

// use serde_json::json;
use speculate::speculate;

speculate! {
        describe "onboardingTest" {
                fn DydxClient() -> DydxClient<'static> {
                        dotenv().ok();
                        let transport = web3::transports::Http::new("https://mainnet.infura.io/v3/ce7426bf07f24fd59a2f7bbb6df217b4").unwrap();
                        let web3 = web3::Web3::new(transport);
                        let eth_private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
                        let options = ClientOptions {
                                network_id: Some(3),
                                api_timeout: None,
                                api_key_credentials: None,
                                stark_private_key: None,
                                web3: Some(web3),
                                eth_private_key: Some(eth_private_key),
                        };
                        // DydxClient::new("https://api.dydx.exchange", Some(options))
                        DydxClient::new("https://api.stage.dydx.exchange", options)

                }

                it "getOnboarding" {
                        b!(async {
                                let sign = dydx_v3_rust::OnboardingAction {
                                        network_id: "1"
                                };

                                // dbg!(signature);
                        });
                }
                it "createUser" {
                        b!(async {
                                let userData = CreateUserParams {
                                        stark_key:"0474df88a6f75dcce483a85d1cdd60034c820736a44cde52005610844fa2d2c7",
                                        stark_key_y_coordinate: "0276f9cc8377c22e272ab3147fc7bdc339ce087259bcfdd153c4d5f356783a4f",
                                        referred_by_affiliate_link: None,
                                        country: None

                                };

                                let response = DydxClient().onboarding.unwrap().create_user(userData ,"0x1e88f23864a8FE784eB152967AccDb394D3b88AD").await;
                                dbg!(&response);

                        });
                }

        }

}
