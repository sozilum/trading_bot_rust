use sha2::{Sha256, Digest};

pub fn create_signature(secret_api_key: &str, data: &str ) -> String{
    let key = secret_api_key.as_bytes();
    let message = data.as_bytes();

    let mut hasher = Sha256::new();
    hasher.update(key);
    hasher.update(message);

    let result = hasher.finalize();

    result.iter().map(|byte| format!("{:02x}", byte)).collect()
}