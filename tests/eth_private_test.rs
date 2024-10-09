macro_rules! b {
        ($e:expr) => {
                tokio_test::block_on($e)
        };
}

use dydx_v3_rust::constants::*;
use dydx_v3_rust::ClientOptions;
use dydx_v3_rust::DydxClient;
use speculate::speculate;
//
// #[cfg(test)]
// speculate! {
//         describe "ethPrivateTest" {
//                 fn DydxClient() -> DydxClient<'static> {
//                         let options = ClientOptions {
//                                 api_timeout: None,
//                                 eth_address: "",
//                                 subaccount_number: "",
//                                 public_error_handler: None,
//                                 private_error_handler: None,
//                                 public_backoff_getter: None,
//                                 private_backoff_getter: None,
//                         };
//                         DydxClient::new(TESTNET_API_URL, "", options)
//
//                 }
//
//
//                 it "recovery" {
//                         b!(async {
//                                 let _response = DydxClient().eth_private.unwrap().recovery(TEST_ADDRESS).await.unwrap();
//                                 // dbg!(&_response);
//                         });
//                 }
//
//                 it "createAndDeleteApiKey" {
//                         b!(async {
//                                 let _response = DydxClient().eth_private.unwrap().create_api_key(TEST_ADDRESS).await.unwrap();
//                                 // dbg!(&_response);
//                                 let _delete__response = DydxClient().eth_private.unwrap().delete_api_key(_response.api_key.key.as_str(), TEST_ADDRESS).await;
//                                 // dbg!(&_delete__response);
//                         });
//                 }
//         }
//
// }
