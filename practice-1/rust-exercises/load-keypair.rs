use dotenv::dotenv;
use std::env;
use solana_sdk::signer::{keypair::Keypair, Signer};
use serde_json::from_str;
use std::fs;

fn main() {
    match dotenv() {
        Ok(_) => println!(".env successfully loaded"),
        Err(e) => println!("Failed to read .env file .env: {:?}", e),
    }
    
    let secret_key_str = env::var("SECRET_KEY").expect("SECRET_KEY not found in .env");
    let cleaned_key_str = secret_key_str.trim_matches('"');
    let secret_key_bytes: Vec<u8> = from_str(cleaned_key_str).expect("Failed to decode secret key");
    let keypair = Keypair::from_bytes(&secret_key_bytes).expect("Failed to create keypair from secret key");

    println!("Public key: {}", keypair.pubkey());
}