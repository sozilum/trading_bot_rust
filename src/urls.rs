pub mod api_methods{
    use std::{
        process::Command, 
        str::from_utf8,
        result::Result,
        error::Error,
        };
    use chrono;
    use crate::utils::{
        SecretKeys,
        NewOrder,
    };
    use crate::typical_tasks::{
        TypicalErrors::{
            api_error,
        }, 
        Key::{
            init_key,
            signature,
            timestamp,
        },
        Querys::{
            output,
        },
    };

    pub fn about_wallet() -> Result<(), Box<dyn Error>>{
        init_key();
        let url = format!("https://api.binance.com/api/v3/account?timestamp={}&signature={}", timestamp(), signature());

        let output = output(&url);

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
        }else{api_error(&output)};
            
    }

    pub fn check_orders() -> Result<(), Box<dyn Error>>{
        init_key();
        let url = format!("https://api.binance.com/api/v3/order?timestamp={}&signature={}", timestamp(), signature());

        let output = output(&url);

        if output.status.success(){
            match from_utf8(&output.stdout){
                Ok(response) => {
                    println!("Orders is {}", response);
                    Ok(())
                },
                Err(e) => {
                    println!("Failed to parse response at utf8 {}", e);
                    Err(Box::new(e))
                }
            }
        }else{api_error(&output)};
    }

    pub fn create_order() -> Result<(), Box<dyn error>>{
        init_key();
        let url = format!("https://api.binance.com/api/v3/order?timestamp={}&signature={}", timestamp(), signature());

        let output = output(&url);
        
        Ok(())
    }
}