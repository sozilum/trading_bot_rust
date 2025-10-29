pub mod api_methods{
    
    use std::{
        process::Command, 
        str::from_utf8,
        result::Result,
        error::Error,
        env,
        };
    use chrono;
    use crate::utils::encryption::create_signature;



    pub fn about_wallet() -> Result<(), Box<dyn Error>>{
        dotenv::dotenv().ok();

        let api_key:String = env::var("api_key_binance").expect("Api key not found");
        let secret_api_key:String = env::var("secret_key_binance").expect("Secret key not found");


        let timestamp = chrono::Utc::now().timestamp_millis().to_string();
        let query = format!("timestamp={}",timestamp);
        let signature = create_signature(&secret_api_key, &query);

        let url = format!("https://api.binance.com/api/v3/account?timestamp={}&signature={}", timestamp, signature);

        let output = Command::new("curl")
            .arg("-H")
            .arg(format!("X-MBX-APIKEY: {}", api_key))
            .arg(&url)
            .output()?;

        if output.status.success(){
            match from_utf8(&output.stdout){
                Ok(response) => {
                    println!("Balance is {}", response);
                    Ok(())
                },
                Err(e) => {
                    println!("Failed to parse response as utd8 {}", e);
                    Err(Box::new(e))
                },
            }
        }else{
            match from_utf8(&output.stderr){
                Ok(error_msg) => {
                    println!("Api error {}", error_msg);
                    Err(error_msg.into())
                },
                Err(e) => {
                    println!("Faild to parse error {}", e);
                    Err(Box::new(e))
                },
            }
        }
    }
}
// pub fn change_pair() -> Result<()>{}

// pub fn about_pair() -> Result<()>{}

// pub fn create_order() -> Result<()>{}

// pub fn check_order() -> Result<()>{}