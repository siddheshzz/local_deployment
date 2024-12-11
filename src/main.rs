use alloy::{ primitives:: U256, providers::ProviderBuilder, sol
};
use eyre::Result;

sol! {
    #[allow(missing_docs)]
    // solc v0.8.26; solc Counter.sol --via-ir --optimize --bin
    #[sol(rpc, bytecode="6080806040523460135760df908160198239f35b600080fdfe6080806040526004361015601257600080fd5b60003560e01c9081633fb5c1cb1460925781638381f58a146079575063d09de08a14603c57600080fd5b3460745760003660031901126074576000546000198114605e57600101600055005b634e487b7160e01b600052601160045260246000fd5b600080fd5b3460745760003660031901126074576020906000548152f35b34607457602036600319011260745760043560005500fea2646970667358221220e978270883b7baed10810c4079c941512e93a7ba1cd1108c781d4bc738d9090564736f6c634300081a0033")]
    contract Counter {
        uint32 public number;

        function setNumber(uint256 newNumber) public {
            number = newNumber;
        }

        function increment() public {
            number++;
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Step1 Create Anvil node and get rpc url
        //on_anvil_with_wallet() takes care of this for us.

    // Step2 Create provider.
     let provider = ProviderBuilder::new().with_recommended_fillers().on_anvil_with_wallet();    

    // Step3 Deploy contract
    let contract = Counter::deploy(&provider).await?;
    
    println!("Deployed contract at address: {}", contract.address());

    let builder = contract.setNumber(U256::from(12));
    let tx_hash = builder.send().await?.watch().await?;

    println!("Set number to 42: {tx_hash}");

    let builder = contract.increment();
    let tx_hash = builder.send().await?.watch().await?;

    println!("Incremented number: {tx_hash}");

    

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
