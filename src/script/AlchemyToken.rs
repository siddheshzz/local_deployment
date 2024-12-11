use alloy::{
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    sol,
};
use eyre::Result;
//Artifact of the smart contract
sol! {
    #[allow(missing_docs)]
    // solc v0.8.26; solc Counter.sol --via-ir --optimize --bin
    #[sol(rpc)]
    ERC20Example,
    "./contracts/NewAlchemy.json"

}

//Smart contract using ERC20-

#[tokio::main]
async fn main() -> Result<()> {
    //rpc url
    let rpc_url = "https://eth.merkle.io";

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .on_anvil_with_wallet_and_config(|anvil| anvil.fork(rpc_url));
    //CREATE account to check
    let accounts = provider.get_accounts().await?;
    let alice = accounts[0];

    let bob = accounts[1];

    let charlie = accounts[2];
    //Deploy:-
    //  we can deploy the contract using ::deploy present in the artifact of the smart contract json file
    //  we can invoke (not technically deploy) by using ::new which takes in the addresss of the deployed contract
    let contract = ERC20Example::deploy(provider).await?;

    println!("Deployed contract at address: {}", contract.address());

    let supply = contract.name().call().await?;
    println!("Token name: {:?}", supply._0);

    println!("Name: {}", contract.name().call().await?._0);

    println!("Symbol: {}", contract.balanceOf(alice).call().await?._0);

    Ok(())
}
