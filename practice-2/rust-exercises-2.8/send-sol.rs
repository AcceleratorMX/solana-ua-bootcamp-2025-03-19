use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::env;
use std::str::FromStr;
use dotenvy::dotenv;

fn main() {
    dotenv().ok();

    let secret_key_str =
        env::var("SECRET_KEY").expect(".env file is not containing valid SECRET_KEY");

    let secret_key_bytes: Vec<u8> =
        serde_json::from_str(&secret_key_str).expect("Failed to decode secret key");

    let sender =
        Keypair::from_bytes(&secret_key_bytes).expect("Failed to create keypair from secret key");

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    println!("Our public key: {}", sender.pubkey());

    let recipient_pubkey = Pubkey::from_str("JvHstfJxybwQvk8xR6RRCmWuhBqY5kbWxXLiwR3G6PL").unwrap();
    println!("Attempting to send 0.01 SOL to {}", recipient_pubkey);

    let memo_program_id = Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr").unwrap();
    let memo_text = "Hello from Solana";

    let transfer_instruction = system_instruction::transfer(
        &sender.pubkey(),
        &recipient_pubkey,
        (0.01 * solana_sdk::native_token::LAMPORTS_PER_SOL as f64) as u64,
    );

    let memo_instruction = Instruction {
        program_id: memo_program_id,
        accounts: vec![],
        data: memo_text.as_bytes().to_vec(),
    };

    let message = Message::new(
        &[transfer_instruction, memo_instruction],
        Some(&sender.pubkey()),
    );

    let mut transaction = Transaction::new_unsigned(message);
    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get blockhash");
    transaction
        .try_sign(&[&sender], recent_blockhash)
        .expect("Failed to sign transaction");

    let signature = client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    println!("Transaction confirmed with signature: {}!", signature);
    println!("Memo is: {}", memo_text);
}
