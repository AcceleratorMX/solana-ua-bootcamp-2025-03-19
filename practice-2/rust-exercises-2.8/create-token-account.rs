use dotenvy::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    pubkey::Pubkey,
};
use spl_associated_token_account::instruction::create_associated_token_account;
use std::env;
use std::str::FromStr;

fn main() {
    dotenv().ok();

    let secret_key_str = env::var("SECRET_KEY").expect(".env file is not containing valid SECRET_KEY");
    let secret_key_bytes: Vec<u8> = serde_json::from_str(&secret_key_str).expect("Failed to decode secret key");
    let sender = Keypair::from_bytes(&secret_key_bytes).expect("Failed to create keypair from secret key");

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    println!("Our public key: {}", sender.pubkey());

    let token_mint_account = Pubkey::from_str("43WutfdugqytWt7MntmtWnYwHHZcjZ65rpZn269Jjnpu").expect("Invalid token mint account");
    let recipient = Pubkey::from_str("ALEx8xkoLJ2UMH1xo3UJcB5oB4QLRrGVVdKGmDm9j34G").expect("Invalid recipient public key");

    let token_account = spl_associated_token_account::get_associated_token_address(&recipient, &token_mint_account);

    if client.get_account(&token_account).is_ok() {
        println!("Token account already exists: {}", token_account);
        return;
    }

    let create_account_instruction = create_associated_token_account(
        &sender.pubkey(),
        &recipient,
        &token_mint_account,
        &spl_token::id(),
    );

    let recent_blockhash = client.get_latest_blockhash().expect("Failed to get recent blockhash");

    let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[create_account_instruction],
        Some(&sender.pubkey()),
        &[&sender],
        recent_blockhash,
    );

    client.send_and_confirm_transaction(&transaction).expect("Failed to send and confirm transaction");

    println!("Token account: {}", token_account);

    let link = format!(
        "https://explorer.solana.com/address/{}?cluster=devnet",
        token_account
    );

    println!("Created token account: {}", link);
}