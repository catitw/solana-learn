use solana_sdk::{
    pubkey::Pubkey,
    signer::{Signer, keypair::Keypair},
};

use solana_sdk::pubkey;

fn generate_keypair_by_pubkey() {
    let keypair = Keypair::new();
    println!("Keypair by public key");
    println!("Public Key: {}", keypair.pubkey());
    println!("Secret bytes: {:?}", keypair.secret_bytes());
    println!("Keypair     : {:?}", keypair.to_bytes());
}

fn generate_keypair_by_program_derived_address() {
    let program_address = pubkey!("11111111111111111111111111111111");
    let seeds = [b"helloWorld".as_ref()];
    let (pda, bump) = Pubkey::find_program_address(&seeds, &program_address);

    println!("Keypair by program derived address");
    println!("PDA: {}", pda);
    println!("Bump: {}", bump);
}

#[tokio::main]
async fn main() {
    generate_keypair_by_pubkey();
    println!();
    generate_keypair_by_program_derived_address();
}
