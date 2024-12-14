use alloy::{
    node_bindings::Anvil,
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::types::{eth, TransactionRequest},
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner, Signer},
    sol,
};
use eyre::Result;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20PresetMinterPauser,
    "./contracts/ERC20PresetMinterPauser.json"
);

#[tokio::main]
async fn main() -> Result<()> {

    //Owner of the contract i.e alice can mint
    
    //cannot use burnFrom on alice
    //burnForm can be only used in case of the sender is spender and is approved some tokens
    
    //cannot burnFrom on carlie

    //can only burn own token

    // allowance and approve to work-> the spender shoud have amount equal or less than amount he
    // is trying to transferFrom

    //spender can transferFrom approver to himself as well as long as above condition is satisfied


    
    //Burn & BurnFrom
    //burn can be used by any individual account to burn the token which they own
    //burnFrom can be used by any individual account to burn the token which they have been given allowance to use and approved

    
    
    Ok(())
}

#[cfg(test)]
mod tests {

    use std::f32::consts::E;

    use alloy::contract::Error;
    // use alloy::{contract::Error, json_abi::Error};
    use eyre::Ok;

    use super::*;

    /// Tests that the ERC20PresetMinterPauser contract can be deployed successfully by the default owner.
    /// Verifies that the contract address is generated and is not empty upon deployment.
    #[tokio::test]
    async fn only_default_owner_can_mint_initially() -> Result<()> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_anvil_with_wallet();
        let contract =
            ERC20PresetMinterPauser::deploy(&provider, "Alchemy".to_string(), "ALC".to_string())
                .await?;

        let accounts = provider.get_accounts().await?;
        let alice = accounts[0];
        let bob = accounts[1];
        let charlie = accounts[2];

        println!("Alice{:?}", contract.balanceOf(alice).call().await?._0);
        println!("Bob{:?}", contract.balanceOf(bob).call().await?._0);
        println!("Charlie{:?}", contract.balanceOf(charlie).call().await?._0);

        //Owner of the contract i.e alice can mint
        contract
            .mint(alice, U256::from(10000000))
            .send()
            .await?
            .watch()
            .await?;
        println!("{:?}", contract.balanceOf(alice).call().await?._0);
        assert_eq!(
            contract.totalSupply().call().await?._0,
            U256::from(10000000)
        );
        Ok(())
    }
    /// Tests that tokens can be transferred successfully.
    /// Verifies that the recipient's balance is increased by the amount transferred.
    ///
    #[tokio::test]
    async fn can_transfer_token() -> Result<()> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_anvil_with_wallet();
        let contract =
            ERC20PresetMinterPauser::deploy(&provider, "Alchemy".to_string(), "ALC".to_string())
                .await?;

        let accounts = provider.get_accounts().await?;
        let alice = accounts[0];
        let bob = accounts[1];
        let charlie = accounts[2];

        println!("Alice{:?}", contract.balanceOf(alice).call().await?._0);
        println!("Bob{:?}", contract.balanceOf(bob).call().await?._0);
        println!("Charlie{:?}", contract.balanceOf(charlie).call().await?._0);

        //Owner of the contract i.e alice can mint
        contract
            .mint(alice, U256::from(10000000))
            .send()
            .await?
            .watch()
            .await?;
        println!("{:?}", contract.balanceOf(alice).call().await?._0);

        println!("boB BEFORE{:?}", contract.balanceOf(bob).call().await?._0);
        contract
            .transfer(bob, U256::from(100000))
            .from(alice)
            .send()
            .await?
            .watch()
            .await?;
        println!("Bob after{:?}", contract.balanceOf(bob).call().await?._0);

        contract
            .transfer(charlie, U256::from(1000))
            .from(bob)
            .send()
            .await?
            .watch()
            .await?;
        println!(
            "Charlie after{:?}",
            contract.balanceOf(charlie).call().await?._0
        );
        assert_eq!(
            contract.balanceOf(charlie).call().await?._0,
            U256::from(1000)
        );
        Ok(())
    }
    /// Tests that a user can transfer tokens using allowance.
    ///
    /// This test will first mint tokens to Alice and then transfer them to Bob.
    /// It will then give Charlie an allowance of 1000 tokens to use from Bob.
    /// Charlie will then transfer 1 token from Bob to himself using the allowance given.
    /// It will then check that the allowance of Charlie to use Bob's fund is reduced by 1 and that the balance of Bob is reduced by 1.
    #[tokio::test]
    async fn can_transfer_using_allowance() -> Result<()> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_anvil_with_wallet();
        let contract =
            ERC20PresetMinterPauser::deploy(&provider, "Alchemy".to_string(), "ALC".to_string())
                .await?;

        let accounts = provider.get_accounts().await?;
        let alice = accounts[0];
        let bob = accounts[1];
        let charlie = accounts[2];

        //Owner of the contract i.e alice can mint
        contract
            .mint(alice, U256::from(10000000))
            .send()
            .await?
            .watch()
            .await?;
        contract
            .transfer(bob, U256::from(100000))
            .from(alice)
            .send()
            .await?
            .watch()
            .await?;
        contract
            .transfer(charlie, U256::from(100))
            .from(alice)
            .send()
            .await?
            .watch()
            .await?;
        println!(
            "Allowance of charlie to use bob's fund{:?}",
            contract.allowance(bob, charlie).call().await?._0
        );
        //APPROVAL OF 1000 TOKENS
        contract
            .approve(charlie, U256::from(1000))
            .from(bob)
            .send()
            .await?
            .watch()
            .await?;

        println!("check now can charlie transfer from allowand fun");
        //charlie transfers allowance token from bob to himself provided he has enough in his own balace
        contract
            .transferFrom(bob, charlie, U256::from(1))
            .from(charlie)
            .send()
            .await?
            .watch()
            .await?;
        println!(
            "Charlie balance  {:?}",
            contract.balanceOf(charlie).call().await?._0
        );
        println!("Bob balance {:?}", contract.balanceOf(bob).call().await?._0);
        assert_eq!(
            contract.balanceOf(charlie).call().await?._0,
            U256::from(101)
        );
        Ok(())
    }
    /// Tests that a user can burn a token.
    ///
    /// Verifies that the token's balance is decreased by the amount burned.
    #[tokio::test]
    async fn can_burn_token() -> Result<()> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_anvil_with_wallet();
        let contract =
            ERC20PresetMinterPauser::deploy(&provider, "Alchemy".to_string(), "ALC".to_string())
                .await?;

        let accounts = provider.get_accounts().await?;
        let alice = accounts[0];
        let bob = accounts[1];
        let charlie = accounts[2];

        //Owner of the contract i.e alice can mint
        contract
            .mint(alice, U256::from(10000000))
            .send()
            .await?
            .watch()
            .await?;
        contract
            .transfer(bob, U256::from(100000))
            .from(alice)
            .send()
            .await?
            .watch()
            .await?;
        contract
            .transfer(charlie, U256::from(100))
            .from(alice)
            .send()
            .await?
            .watch()
            .await?;
        contract
            .burn(U256::from(1))
            .from(bob)
            .send()
            .await?
            .watch()
            .await?;
        assert_eq!(contract.balanceOf(bob).call().await?._0, U256::from(99999));
        Ok(())
    }


    /// Tests that a user can burn allowance tokens from another user, given that user has enough balance of the tokens.
    ///
    /// This test will first mint tokens to Alice and then transfer them to Bob.
    /// It will then give Charlie an allowance of 1000 tokens to use from Bob.
    /// Charlie will then transfer 1 token from Bob to himself using the allowance given.
    /// It will then check that the allowance of Charlie to use Bob's fund is reduced by 1 and that the balance of Bob is reduced by 1.
    #[tokio::test]
    async fn spender_can_burn_allowance_token() -> Result<()> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_anvil_with_wallet();
        let contract =
            ERC20PresetMinterPauser::deploy(&provider, "Alchemy".to_string(), "ALC".to_string())
                .await?;

        let accounts = provider.get_accounts().await?;
        let alice = accounts[0];
        let bob = accounts[1];
        let charlie = accounts[2];

        //Owner of the contract i.e alice can mint
        contract
            .mint(alice, U256::from(10000000))
            .send()
            .await?
            .watch()
            .await?;
        contract
            .transfer(bob, U256::from(100000))
            .from(alice)
            .send()
            .await?
            .watch()
            .await?;
        contract
            .transfer(charlie, U256::from(100))
            .from(alice)
            .send()
            .await?
            .watch()
            .await?;
        println!(
            "Allowance of charlie to use bob's fund{:?}",
            contract.allowance(bob, charlie).call().await?._0
        );
        //APPROVAL OF 1000 TOKENS
        contract
            .approve(charlie, U256::from(1000))
            .from(bob)
            .send()
            .await?
            .watch()
            .await?;

        println!("check now can charlie transfer from allowand fun");
        //charlie transfers allowance token from bob to himself provided he has enough in his own balace
        contract
            .burnFrom(bob, U256::from(1))
            .from(charlie)
            .send()
            .await?
            .watch()
            .await?;
        println!("Bob balance {:?}", contract.balanceOf(bob).call().await?._0);
        assert_eq!(contract.balanceOf(bob).call().await?._0, U256::from(99999));
        Ok(())
    }
    /// Tests that the allowance of a user can be decreased by the user who approved the allowance.
    /// Verifies that the allowance is decreased by the correct amount.
    /// Verifies that the allowance of the user is not decreased below 0.
    #[tokio::test]
    async fn approver_can_decerease_allowance() -> Result<()> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_anvil_with_wallet();
        let contract =
            ERC20PresetMinterPauser::deploy(&provider, "Alchemy".to_string(), "ALC".to_string())
                .await?;

        let accounts = provider.get_accounts().await?;
        let alice = accounts[0];
        let bob = accounts[1];
        let charlie = accounts[2];

        //Owner of the contract i.e alice can mint
        contract
            .mint(alice, U256::from(10000000))
            .send()
            .await?
            .watch()
            .await?;
        contract
            .transfer(bob, U256::from(100000))
            .from(alice)
            .send()
            .await?
            .watch()
            .await?;
        contract
            .transfer(charlie, U256::from(100))
            .from(alice)
            .send()
            .await?
            .watch()
            .await?;
        println!(
            "Allowance of charlie to use bob's fund{:?}",
            contract.allowance(bob, charlie).call().await?._0
        );
        //APPROVAL OF 1000 TOKENS
        contract
            .approve(charlie, U256::from(10))
            .from(bob)
            .send()
            .await?
            .watch()
            .await?;

        contract
            .decreaseAllowance(charlie, U256::from(5))
            .from(bob)
            .send()
            .await?
            .watch()
            .await?;

        println!("check now can charlie transfer 8 from allowand fun");
        //charlie transfers allowance token from bob to himself provided he has enough in his own balace
        let res = contract
            .transferFrom(bob, charlie, U256::from(8))
            .from(charlie)
            .send()
            .await;
        //the transferFrom will return error as the approved token < requested token
        assert!(res.is_err());

        Ok(())
    }
    /// Tests that the approver can increase the allowance of a user to use his/her tokens.
    ///
    /// Verifies that the allowance is increased by the amount specified.
    /// Also verifies that the user can then transfer the increased allowance to another user.
    #[tokio::test]
    async fn approver_can_increase_allowance() -> Result<()> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_anvil_with_wallet();
        let contract =
            ERC20PresetMinterPauser::deploy(&provider, "Alchemy".to_string(), "ALC".to_string())
                .await?;

        let accounts = provider.get_accounts().await?;
        let alice = accounts[0];
        let bob = accounts[1];
        let charlie = accounts[2];

        //Owner of the contract i.e alice can mint
        contract
            .mint(alice, U256::from(10000000))
            .send()
            .await?
            .watch()
            .await?;
        contract
            .transfer(bob, U256::from(100000))
            .from(alice)
            .send()
            .await?
            .watch()
            .await?;
        println!(
            "Allowance of charlie to use bob's fund{:?}",
            contract.allowance(bob, charlie).call().await?._0
        );
        //APPROVAL OF 1000 TOKENS
        contract
            .approve(charlie, U256::from(10))
            .from(bob)
            .send()
            .await?
            .watch()
            .await?;

        contract
            .increaseAllowance(charlie, U256::from(5))
            .from(bob)
            .send()
            .await?
            .watch()
            .await?;

        println!("check now can charlie transfer 8 from allowand fun");
        //charlie transfers allowance token from bob to himself provided he has enough in his own balace
        contract
            .transferFrom(bob, charlie, U256::from(13))
            .from(charlie)
            .send()
            .await?
            .watch()
            .await?;

        assert_ne!(
            contract.balanceOf(charlie).call().await?._0,
            U256::from(113)
        );
        Ok(())
    }

    /// Tests that the owner of the contract can grant MINTER_ROLE to another user.
    /// Verifies that the user who was granted the role can mint tokens.
    /// Verifies that the user who was granted the role can grant MINTER_ROLE to another user.
    #[tokio::test]
    async fn owner_can_grant_role() -> Result<()> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_anvil_with_wallet();
        let contract =
            ERC20PresetMinterPauser::deploy(&provider, "Alchemy".to_string(), "ALC".to_string())
                .await?;

        let accounts = provider.get_accounts().await?;
        let alice = accounts[0];
        let bob = accounts[1];
        let charlie = accounts[2];

        //Owner of the contract i.e alice can mint
        contract
            .mint(alice, U256::from(10000000))
            .send()
            .await?
            .watch()
            .await?;
        contract
            .transfer(bob, U256::from(100000))
            .from(alice)
            .send()
            .await?
            .watch()
            .await?;
        contract
            .transfer(bob, U256::from(100))
            .from(alice)
            .send()
            .await?
            .watch()
            .await?;

        contract
            .grantRole(contract.MINTER_ROLE().call().await?._0, bob)
            .from(alice)
            .send()
            .await?
            .watch()
            .await?;

        // contract
        //     .grantRole(contract.MINTER_ROLE().call().await?._0, charlie)
        //     .from(bob)
        //     .send()
        //     .await?
        //     .watch()
        //     .await?;
       

        println!(
            "Total supply before bob became admin{:?}",
            contract.totalSupply().call().await?._0
        );

        let numberOfMember = contract
            .getRoleMemberCount(contract.MINTER_ROLE().call().await?._0)
            .call()
            .await?
            ._0;

        assert_ne!(numberOfMember, U256::from(3));

        Ok(())
    }

/// Tests that a user with the MINTER_ROLE can mint new tokens.
/// 
/// This test will first deploy the contract and mint tokens to Alice.
/// It will then transfer some tokens to Bob and grant Bob the MINTER_ROLE.
/// Finally, Bob will mint an additional token to his account, and the test
/// will verify that the total supply of tokens has increased accordingly.
    #[tokio::test]
    async fn can_mint_if_MINTER_role() -> Result<()> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_anvil_with_wallet();
        let contract =
            ERC20PresetMinterPauser::deploy(&provider, "Alchemy".to_string(), "ALC".to_string())
                .await?;

        let accounts = provider.get_accounts().await?;
        let alice = accounts[0];
        let bob = accounts[1];
        let charlie = accounts[2];

        //Owner of the contract i.e alice can mint
        contract
            .mint(alice, U256::from(10000000))
            .send()
            .await?
            .watch()
            .await?;
        contract
            .transfer(bob, U256::from(100000))
            .from(alice)
            .send()
            .await?
            .watch()
            .await?;
        contract
            .transfer(bob, U256::from(100))
            .from(alice)
            .send()
            .await?
            .watch()
            .await?;

        contract
            .grantRole(contract.MINTER_ROLE().call().await?._0, bob)
            .from(alice)
            .send()
            .await?
            .watch()
            .await?;

        contract
            .mint(bob, U256::from(1))
            .from(bob)
            .send()
            .await?
            .watch()
            .await?;
        assert_eq!(
            contract.totalSupply().call().await?._0,
            U256::from(10000001)
        );
        Ok(())
    }
    /// Tests that the PAUSER_ROLE can pause the contract.
    ///
    /// Verifies that the contract is paused after the PAUSER_ROLE calls the pause function.
    /// Verifies that the paused() function returns true after the contract is paused.
    #[tokio::test]
    async fn can_pause_if_PAUSER_role() -> Result<()> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_anvil_with_wallet();
        let contract =
            ERC20PresetMinterPauser::deploy(&provider, "Alchemy".to_string(), "ALC".to_string())
                .await?;

        let accounts = provider.get_accounts().await?;
        let alice = accounts[0];
        let bob = accounts[1];
        let charlie = accounts[2];

        //Owner of the contract i.e alice can mint
        contract
            .mint(alice, U256::from(10000000))
            .send()
            .await?
            .watch()
            .await?;
        contract
            .transfer(bob, U256::from(100000))
            .from(alice)
            .send()
            .await?
            .watch()
            .await?;
        contract
            .transfer(bob, U256::from(100))
            .from(alice)
            .send()
            .await?
            .watch()
            .await?;

        contract
            .grantRole(contract.PAUSER_ROLE().call().await?._0, bob)
            .from(alice)
            .send()
            .await?
            .watch()
            .await?;

        contract.pause().from(bob).send().await?.watch().await?;
        assert_eq!(contract.paused().call().await?._0, true);
        Ok(())
    }
}
