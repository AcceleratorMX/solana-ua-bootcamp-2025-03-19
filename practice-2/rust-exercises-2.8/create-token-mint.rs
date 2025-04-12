use dotenvy::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
};
use spl_token::{
    instruction as spl_instruction,
    solana_program::program_pack::Pack,
    state as spl_state,
};
use std::env;

fn main() {
    dotenv().ok();

    let secret_key_str = env::var("SECRET_KEY").expect(".env file is not containing valid SECRET_KEY");
    let secret_key_bytes: Vec<u8> = serde_json::from_str(&secret_key_str).expect("Failed to decode secret key");
    let sender = Keypair::from_bytes(&secret_key_bytes).expect("Failed to create keypair from secret key");

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    println!("Our public key: {}", sender.pubkey());

    let mint_rent = client
        .get_minimum_balance_for_rent_exemption(spl_state::Mint::LEN)
        .expect("Failed to get minimum balance for rent exemption");

    let mint_keypair = Keypair::new();

    let mint_account_instruction = system_instruction::create_account(
        &sender.pubkey(),
        &mint_keypair.pubkey(),
        mint_rent,
        spl_state::Mint::LEN as u64,
        &spl_token::id(),
    );

    let token_mint = spl_instruction::initialize_mint(
        &spl_token::id(),
        &Pubkey::from(mint_keypair.pubkey()),
        &Pubkey::from(sender.pubkey()),
        None,
        2,
    ).unwrap();

    let recent_blockhash = client.get_latest_blockhash().expect("Failed to get recent blockhash");

    let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[mint_account_instruction, token_mint.into()],
        Some(&sender.pubkey()),
        &[&sender, &mint_keypair],
        recent_blockhash,
    );

    client.send_and_confirm_transaction(&transaction).expect("Failed to send and confirm transaction");

    let link = format!(
        "https://explorer.solana.com/address/{}?cluster=devnet",
        mint_keypair.pubkey()
    );

    println!("Token mint: {}", link);
}