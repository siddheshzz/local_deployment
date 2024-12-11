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



#[cfg(test)]
mod tests {

    use eyre::Ok;

    use super::*;

    #[tokio::test]
    async fn can_launch_deploy_contract()-> Result<()> {
        let provider = ProviderBuilder::new().with_recommended_fillers().on_anvil_with_wallet();    
        let contract = Counter::deploy(&provider).await?;
        //verify if the contact address is generated and not empty
        assert_eq!(contract.address().is_empty(),false );
        Ok(())
    }

    #[tokio::test]
    async fn can_set_counter()-> Result<()> {
        let provider = ProviderBuilder::new().with_recommended_fillers().on_anvil_with_wallet();    
        let contract = Counter::deploy(&provider).await?;
        // Lets increment number by 1
        contract.increment().send().await?.watch().await?;
        let num = contract.number().call().await?;
        // Verify if number is incred by 1
        assert_eq!(1,num.number);
        // Lets increment number one more time
        contract.increment().send().await?.watch().await?;

        // Verify if value is  incremented and number is set to 2
        let num = contract.number().call().await?;
        assert_eq!(2,num.number);
        Ok(())
        
    }

    #[tokio::test]
    async fn can_set_number()-> Result<()> {
        let provider = ProviderBuilder::new().with_recommended_fillers().on_anvil_with_wallet();    
        let contract = Counter::deploy(&provider).await?;
        //verify if the value is set to new value on call of setNumber method.
        contract.setNumber(U256::from(23)).send().await?.watch().await?;
        assert_eq!(contract.number().call().await?.number,23);
        
        Ok(())
    }
}
