
pub mod encryption{

    use sha2::{Sha256, Digest};
    use hmac::{Hmac, Mac};

    type HmacSha256 = Hmac<Sha256>;
    
    pub fn create_signature(secret_api_key: &str, data: &str ) -> String{
        
        let mut mac = HmacSha256::new_from_slice(secret_api_key.as_bytes()).expect("Key maybe any size");
        mac.update(data.as_bytes());
                
        let result = mac.finalize();
        let code_bytes = result.into_bytes();


        
        code_bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
    }
}