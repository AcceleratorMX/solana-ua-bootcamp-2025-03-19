use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

#[tokio::main]
async fn main() {
    let connection = RpcClient::new("https://api.devnet.solana.com".to_string());
    println!("Connection successful!");

    let public_key = Pubkey::from_str("ALEx8xkoLJ2UMH1xo3UJcB5oB4QLRrGVVdKGmDm9j34G").unwrap();

    let balance_in_lamports = connection.get_balance(&public_key).await.unwrap();
    let balance_in_sol = balance_in_lamports / LAMPORTS_PER_SOL;

    println!("Your wallet: {}", public_key);
    println!("Your balance: {} SOL", balance_in_sol);
}