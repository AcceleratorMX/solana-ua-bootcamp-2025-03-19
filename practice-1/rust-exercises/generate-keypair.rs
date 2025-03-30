use solana_sdk::signer::{keypair::Keypair, Signer};

fn main() {
    let keypair = Keypair::new();

    println!("Public key: {}", &keypair.pubkey().to_string());
    println!("Private key: {:?}", &keypair.to_bytes());
}