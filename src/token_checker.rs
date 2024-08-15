use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use std::convert::TryFrom;
use std::error::Error;
use {dotenv::dotenv, std::env};

pub async fn get_token_balance(
    token_address: &str,
    user_address: &str,
) -> Result<String, Box<dyn Error>> {
    dotenv().ok();

    let project_id = env::var("INFURA_TOKEN")?;
    let url = format!("https://mainnet.infura.io/v3/{}", project_id);
    let provider = Provider::<Http>::try_from(url)?;

    let token_address = token_address.parse::<Address>()?;
    let user_address = user_address.parse::<Address>()?;

    let abi = include_bytes!("erc20_abi.json");
    let contract = Contract::new(token_address, abi, provider);
    let balance: U256 = contract.method::<U256>("balanceOf", user_address)?.call().await?;

    Ok(balance.to_string())
}