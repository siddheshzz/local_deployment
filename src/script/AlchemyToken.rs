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

    use alloy::providers::WalletProvider;
    use eyre::Ok;

    use super::*;

    #[tokio::test]
    async fn can_only_owner_mint()-> Result<()> {
        let provider = ProviderBuilder::new().with_recommended_fillers().on_anvil_with_wallet(); 
        
        let contract = ERC20Example::deploy(&provider).await?;

        // require!(contract.owner().call().await? == alice);
        let accounts = provider.get_accounts().await?;
        //the first account is the one which deploys the contract when we call ::deploy()- alice
        let alice = accounts[0];
        
        let bob = accounts[1];
        println!("{:?}",contract.balanceOf(bob).call().await?._0);
        let charlie = accounts[2];
        contract.transfer(bob, U256::from(1000000)).send().await?.watch().await?;
        println!("{:?}",contract.balanceOf(bob).call().await?._0);
        contract.allowance(alice, bob).call().await?._0;
        println!("{:?}",contract.balanceOf(bob).call().await?._0);
        contract.increaseAllowance(bob,  U256::from(100)).send().await?.watch().await?;
        println!("{:?}",contract.balanceOf(alice).call().await?._0);
        println!("{:?}",contract.balanceOf(bob).call().await?._0);

        assert_eq!(contract.balanceOf(bob).call().await?._0, U256::from(1000000));
        Ok(())
    }
    #[tokio::test]
    async fn can_transfer()-> Result<()> {
        let provider = ProviderBuilder::new().with_recommended_fillers().on_anvil_with_wallet(); 
        
        let contract = ERC20Example::deploy(&provider).await?;

        // require!(contract.owner().call().await? == alice);
        let accounts = provider.get_accounts().await?;
        //the first account is the one which deploys the contract when we call ::deploy()- alice
        let alice = accounts[0];
        
        let bob = accounts[1];

        let charlie = accounts[2];
       
        contract.transfer(bob, U256::from(100000)).from(alice).send().await?.watch().await?;

        println!("{:?}",contract.balanceOf(bob).call().await?._0);
        assert_eq!(contract.balanceOf(bob).call().await?._0, U256::from(100000));
        Ok(())
    }
    #[tokio::test]
    async fn can_transfer_only_owner()-> Result<()> {
        let provider = ProviderBuilder::new().with_recommended_fillers().on_anvil_with_wallet(); 
        
        let contract = ERC20Example::deploy(&provider).await?;

        // require!(contract.owner().call().await? == alice);
        let accounts = provider.get_accounts().await?;
        //the first account is the one which deploys the contract when we call ::deploy()- alice
        let alice = accounts[0];
        
        let bob = accounts[1];

        let charlie = accounts[2];
        //.from lets you send the transaction from a specific account
        contract.transfer(charlie, U256::from(1000)).from(alice).send().await?.watch().await?;
        
        // println!("{:?}",);
        contract.transfer(bob, U256::from(10)).from(charlie).send().await?.watch().await?;
        println!("{:?}",contract.balanceOf(bob).call().await?._0);
        assert_ne!(contract.balanceOf(bob).call().await?._0, U256::from(0));
        Ok(())
    }

    #[tokio::test]
    async fn can_transfer_from_non_owner_with_approve()-> Result<()> {
        let provider = ProviderBuilder::new().with_recommended_fillers().on_anvil_with_wallet(); 
        
        let contract = ERC20Example::deploy(&provider).await?;

        // require!(contract.owner().call().await? == alice);
        let accounts = provider.get_accounts().await?;
        //the first account is the one which deploys the contract when we call ::deploy()- alice
        let alice = accounts[0];
        
        let bob = accounts[1];

        let charlie = accounts[2];
        println!("{:?}",charlie);

        // println!("totalSupply: {:?}", contract.approve(alice,  U256::from(100000)).call().await?._0);
        // contract.approve(bob,  U256::from(100000)).send().await?.watch().await?;
        // contract.allowance(alice, bob).call().await?._0;
        // contract.approve(bob,  U256::from(100000)).call().await?._0;
        // contract.increaseAllowance(bob,  U256::from(100)).send().await?.watch().await?;
        contract.transfer(bob, U256::from(100)).call().await?._0;
        // contract.transferFrom(bob, charlie, U256::from(10)).send().await?.watch().await?;s

        println!("{:?}",contract.balanceOf(charlie).call().await?._0);


        assert_eq!(contract.balanceOf(charlie).call().await?._0, U256::from(0));
        Ok(())
    }

}

