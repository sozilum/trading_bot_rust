
pub mod handling_errors{
    use std::str::utd8;
    use chrono;
    use crate::utils::SecretKeys;


    pub trait TypicalErrors{
        pub fn api_error(&output) ->  Box<dyn Error>{
            match from_utf8(&output.stderr){
                Ok(error_msg) => {
                    println!("Api error {}", error_msg);
                    Err(error_msg.into())
                },
                Err(e) => {
                    println!("Failed to parse error {}", e);
                    Err(Box::new(e))
                },
            }
            
        }
    }

    pub trait Key{
        pub fn init_key(&self) -> SecretKeys{
            let key = SecretKeys::new();
            key
        }

        pub fn signature(&self, data: String) -> String{
            let signature = SecretKeys::create_signature(&self key.secret_key, &self query);
            signature
        }

        pub fn timestamp(&self) -> String{
        let timestamp = chrono::Utc::now().timestamp_millis().to_string();
        let query = format!("timestamp={}",timestamp);
        query
        }
    }

    pub trait Querys{
        pub fn output(&self, url: &String) -> String{
            let output = Command::new("curl")
                .arg("-H")
                .arg(format!("X-MBX-APIKEY: {}", &self key.publick_key))
                .arg(&url)
                .output()?
        }
    }
}