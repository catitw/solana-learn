use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::pubkey;

async fn print_account_info(url: &str, program_id: &pubkey::Pubkey) -> anyhow::Result<()> {
    let connection = RpcClient::new_with_commitment(url.to_string(), CommitmentConfig::confirmed());

    let account_info = connection.get_account(program_id).await?;
    println!("{:#?}", account_info);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    print_account_info(
        "https://api.mainnet-beta.solana.com",
        &pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
    )
    .await?;

    Ok(())
}
