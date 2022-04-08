macro_rules! b {
        ($e:expr) => {
                tokio_test::block_on($e)
        };
}

use dydx_v3_rust::helper::*;
use dydx_v3_rust::structs::*;
use dydx_v3_rust::ClientOptions;
#[cfg(test)]
use dydx_v3_rust::DydxClient;

// use serde_json::json;
use speculate::speculate;

speculate! {
        describe "privateTest" {
                fn DydxClient() -> DydxClient<'static> {
                        let api_key = ApiKeyCredentials {
                                // account2 testnet
                                key: "ed85a071-c6b4-b4f1-c965-efb238d16c5e",
                                secret: "1iDz27dyq4RspTkP-rfTcFN6ouxTgHmTT_sKJogU",
                                passphrase: "CfbXaq6O-Yd3jKOqh10i"
                                // passphrase: "CfbXaq6O-Yd3jKOqh10iaa"
                        };
                        let options = ClientOptions {
                                network_id: Some(3),
                                api_timeout: None,
                                api_key_credentials: Some(api_key),
                                stark_private_key: Some("0657eaa201ba872f72c0e6e2db278d8cda1b60de4313f02213aaf2b3421bff56")
                        };
                        // DydxClient::new("https://api.dydx.exchange", Some(options))
                        DydxClient::new("https://api.stage.dydx.exchange", options)
                    }

                it "getClient" {
                        // dbg!(DydxClient().host);
                        // dbg!(DydxClient().network_id);
                        // dbg!(DydxClient().api_key_credentials);
                }

                it "getAccountId" {
                        b!(async {
                                let uuid = get_account_id("0x0De1C59f3AA4938B0bDcC070B4Fa9F395A4D6d25");
                                // println!("{:?}", response);
                                // dbg!(uuid);
                        });
                }

                it "getAccount" {
                        b!(async {
                                let response = DydxClient().private.unwrap().get_account("0x1e88f23864a8FE784eB152967AccDb394D3b88AD").await.unwrap();
                                // println!("{:?}", response);
                                dbg!(response);
                        });
                }

                // it "getAccountUnauthorized" {
                //         b!(async {
                //                 let response = DydxClient().private.unwrap().get_account("").await;
                //                 match response {
                //                         Ok(v) => println!("{:?}", v),
                //                         Err(e) => println!("{:?}", e),
                //                     }
                //                 // println!("{:?}", response);
                //                 // dbg!(response);
                //         });
                // }

                it "getAccounts" {
                        b!(async {
                                let response = DydxClient().private.unwrap().get_accounts().await.unwrap();
                                // println!("{:?}", response);
                                // dbg!(response);
                        });
                }

                it "getPositionsWithNoParameters" {
                        b!(async {
                                let response = DydxClient().private.unwrap().get_positions(None, None, None, None).await.unwrap();
                                // println!("{:?}", response);
                                // dbg!(response);
                        });
                }

                it "getPositions" {
                        b!(async {
                                let response = DydxClient().private.unwrap().get_positions(Some(DydxMarket::BTC_USD), None, None, Some("2022-04-01T02:43:02.946Z")).await.unwrap();
                                println!("{:?}", response);
                                // dbg!(response);
                        });
                }

                it "generateClientId" {
                        b!(async {
                                let client_id = generate_random_client_id();
                                dbg!(client_id);
                        });
                }

                it "createOrder" {
                        b!(async {
                                let client_id = create_order();
                                dbg!(client_id);
                        });
                }
        }
}