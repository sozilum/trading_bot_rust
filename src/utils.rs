type HmacSha256 = Hmac<Sha256>;
use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};
use serde::Deserialize;

pub struct SecretKeys{

    #[serde(default = "publick_key")]
    publick_key: String,

    #[serde(default = "secret_key")]
    secret_key: String,
}

fn publick_key() -> String {
    dotenv::dotenv::ok();
    env::var("api_key_binance").expect("publick api key is not found")

}

fn secret_key() -> String {
    dotenv::dotenv::ok();
    env::var("secret_key_binance").expect("Secret api key is not found")

}

impl SecretKeys{    
    pub fn create_signature(&self, data: &String) -> String{
        let mut mac = HmacSha256::new_from_slice(&self.secret_key.as_bytes()).expect("key maybe any size");
        mac.update(data.as_bytes());

        let result = mac.finalize();
        let code_bytes = result.into_bytes();

        code_bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus{
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    PendingCancel,
    Rejected,
    Expired,

    #[serde(other)]
    Unknown,
}



pub struct NewOrder{
    symbol: String,
    order_id: u64,

    #[serde(default = "default_order_list_id")]
    order_list_id: i32

    client_order_id: String,
    transact_time: u64,
    working_time: Some(i64),
    price: String,
    orig_qty: String,
    executed_qty: String,
    cummulative_quote_qty: i32,
    status: OrderStatus,
    time_in_force: String,
    r#type: String,
    side: String,
}

fn default_order_list_id() -> i64{
    -1
}
