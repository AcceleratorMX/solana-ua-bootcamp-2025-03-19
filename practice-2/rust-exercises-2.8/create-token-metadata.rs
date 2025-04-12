use dotenvy::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use mpl_token_metadata::instruction::create_metadata_accounts_v3;
use std::env;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let secret_key_str = env::var("SECRET_KEY").expect("SECRET_KEY not found in .env");
    let secret_key_bytes: Vec<u8> = serde_json::from_str(&secret_key_str)?;
    let user = Keypair::from_bytes(&secret_key_bytes)?;

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let token_metadata_program_id = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")?;
    let token_mint_account = Pubkey::from_str("43WutfdugqytWt7MntmtWnYwHHZcjZ65rpZn269Jjnpu")?;

    let (metadata_pda, _bump) = Pubkey::find_program_address(
        &[
            b"metadata",
            token_metadata_program_id.as_ref(),
            token_mint_account.as_ref(),
        ],
        &token_metadata_program_id,
    );

    let create_metadata_instruction = create_metadata_accounts_v3(
        token_metadata_program_id,
        metadata_pda,
        token_mint_account,
        user.pubkey(),
        user.pubkey(),
        user.pubkey(),
        "Solana UA Bootcamp 2025-03-19".to_string(),
        "UAB-3".to_string(),
        "https://arweave.net/1234".to_string(),
        None,
        0,
        true,
        false,
        None,
        None,
        None,
    );

    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[create_metadata_instruction],
        Some(&user.pubkey()),
        &[&user],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;

    let token_mint_link = format!(
        "https://explorer.solana.com/address/{}?cluster=devnet",
        token_mint_account
    );
    println!("✅ Look at the token mint again: {}", token_mint_link);

    Ok(())
}