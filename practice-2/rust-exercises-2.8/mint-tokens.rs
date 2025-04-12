use dotenvy::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use spl_token::instruction::mint_to;
use std::env;
use std::str::FromStr;

fn main() {
    dotenv().ok().expect("Failed to read .env file");

    let secret_key_str =
        env::var("SECRET_KEY").expect(".env file is not containing valid SECRET_KEY");

    let secret_key_bytes: Vec<u8> =
        serde_json::from_str(&secret_key_str).expect("Failed to decode secret key");

    let sender =
        Keypair::from_bytes(&secret_key_bytes).expect("Failed to create keypair from secret key");

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    println!("Our public key: {}", sender.pubkey());

    let token_mint_account = Pubkey::from_str("43WutfdugqytWt7MntmtWnYwHHZcjZ65rpZn269Jjnpu")
        .expect("Failed to parse token mint account public key");

    let recipient_associated_token_account = Pubkey::from_str(
        "5cChGbFE2q4KfhHMubeheh2WvUZaruRHLkbMBGqx9Cyw",
    )
    .expect("Failed to parse recipient associated token account public key");

    let minor_units_per_major_units = 10_u64.pow(2);

    let mint_to_instruction = mint_to(
        &spl_token::id(),
        &token_mint_account,
        &recipient_associated_token_account,
        &sender.pubkey(),
        &[],
        10 * minor_units_per_major_units,
    )
    .expect("Failed to create mint_to instruction");

    let recent_blockhash = client.get_latest_blockhash().expect("Failed to get recent blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[mint_to_instruction],
        Some(&sender.pubkey()),
        &[&sender],
        recent_blockhash,
    );

    let transaction_signature = client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    let link = format!(
        "https://explorer.solana.com/tx/{}?cluster=devnet",
        transaction_signature
    );

    println!("Success! Mint token transaction: {}", link);
}