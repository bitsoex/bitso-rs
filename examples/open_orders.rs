extern crate bitsors;

use bitsors::auth::BitsoCredentials;
use bitsors::client::Bitso;

#[tokio::main]
async fn main() {
    // Set API_KEY and API_SECRET in .env file or
    // export API_KEY="your api_key"
    // export API_SECRET="your_api_secret"
    let client_credential = BitsoCredentials::default().build();
    // Or set API_KEY and API_SECRET explictly
    // let client_credential = BitsoCredentials::default()
    //     .api_key("this-is-my-client-id")
    //     .api_secret("this-is-my-client-secret")
    //     .build();
    let bitso = Bitso::default()
        .client_credentials_manager(client_credential)
        .build();
    let result = bitso.get_open_orders(Some("btc_mxn"), None, None).await;
    println!("{:?}", result);
}
