use solana_sdk::{
    pubkey::Pubkey,
    signer::{Signer, keypair::Keypair},
};

use solana_sdk::pubkey;

fn generate_address_by_pubkey() {
    let keypair = Keypair::new();
    println!("Generate address by public key");
    println!(
        "Public Key which serves as the account address: {}",
        keypair.pubkey()
    );
    println!(
        "Secret bytes which is used to sign transactions : {:?}",
        keypair.secret_bytes()
    );
    println!("Keypair     : {:?}", keypair.to_bytes());
}

fn generate_keypair_by_program_derived_address() {
    let program_address = pubkey!("11111111111111111111111111111111"); // the program id
    let seeds = [b"helloWorld".as_ref()];
    // PDA is an address that is deterministically derived using a program ID and one or more optional inputs (seeds)
    //
    // - see also [questions#2271](https://solana.stackexchange.com/questions/2271/what-is-the-bump-in-a-program-derived-address)
    let (pda, bump) = Pubkey::find_program_address(&seeds, &program_address);

    println!("Keypair by program derived address");
    println!("Program derived address(PDA): {}", pda);
    println!("Bump: {}", bump);
}

#[tokio::main]
async fn main() {
    generate_address_by_pubkey();
    println!();
    generate_keypair_by_program_derived_address();
}
