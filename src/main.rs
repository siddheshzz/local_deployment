//! Example of deploying a contract from Solidity code using the `sol!` macro to Anvil and
//! interacting with it.

use alloy::{network::{EthereumWallet, NetworkWallet, TransactionBuilder}, node_bindings::Anvil, primitives::{address, bytes, U256, U8}, providers::{Provider, ProviderBuilder, WalletProvider}, rpc::types::{eth, TransactionRequest}, signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner, Signer}, sol};
use eyre::Result;
use dotenv::dotenv;
// use alloy::signers::local::{coins_bip39::English, MnemonicBuilder};

// Codegen from embedded Solidity code and precompiled bytecode.
sol! {
    #[allow(missing_docs)]
    // #[sol(rpc, bytecode="608060405234801561000f575f80fd5b50335f806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506001600281905550610a9e806100645f395ff3fe608060405260043610610085575f3560e01c8063b61f346d11610058578063b61f346d14610167578063c1cfb99a14610171578063ccbac9f51461019b578063dfbf53ae146101c5578063e7e4552c146101db57610085565b80630544ce5e146100895780630f31b7d2146100c55780638da5cb5b14610101578063a2fb11751461012b575b5f80fd5b348015610094575f80fd5b506100af60048036038101906100aa91906106f2565b610205565b6040516100bc919061075c565b60405180910390f35b3480156100d0575f80fd5b506100eb60048036038101906100e691906106f2565b610240565b6040516100f8919061075c565b60405180910390f35b34801561010c575f80fd5b50610115610279565b6040516101229190610795565b60405180910390f35b348015610136575f80fd5b50610151600480360381019061014c91906106f2565b61029c565b60405161015e919061075c565b60405180910390f35b61016f6102cc565b005b34801561017c575f80fd5b50610185610341565b60405161019291906107bd565b60405180910390f35b3480156101a6575f80fd5b506101af610348565b6040516101bc91906107bd565b60405180910390f35b3480156101d0575f80fd5b506101d9610399565b005b3480156101e6575f80fd5b506101ef61058e565b6040516101fc919061088d565b60405180910390f35b60018181548110610214575f80fd5b905f5260205f20015f915054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b5f60035f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff169050919050565b5f8054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b6003602052805f5260405f205f915054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b67016345785d8a000034116102df575f80fd5b600133908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550565b5f47905090565b5f805f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff164260405160200161037c929190610912565b604051602081830303815290604052805190602001205f1c905090565b5f8054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146103ef575f80fd5b5f6001805490506103fe610348565b610408919061096a565b90506001818154811061041e5761041d61099a565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166108fc4790811502906040515f60405180830381858888f19350505050158015610488573d5f803e3d5ffd5b5060025f81548092919061049b906109f4565b9190505550600181815481106104b4576104b361099a565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1660035f60025481526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f67ffffffffffffffff81111561054657610545610a3b565b5b6040519080825280602002602001820160405280156105745781602001602082028036833780820191505090505b506001908051906020019061058a929190610619565b5050565b6060600180548060200260200160405190810160405280929190818152602001828054801561060f57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116105c6575b5050505050905090565b828054828255905f5260205f2090810192821561068f579160200282015b8281111561068e578251825f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555091602001919060010190610637565b5b50905061069c91906106a0565b5090565b5b808211156106b7575f815f9055506001016106a1565b5090565b5f80fd5b5f819050919050565b6106d1816106bf565b81146106db575f80fd5b50565b5f813590506106ec816106c8565b92915050565b5f60208284031215610707576107066106bb565b5b5f610714848285016106de565b91505092915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6107468261071d565b9050919050565b6107568161073c565b82525050565b5f60208201905061076f5f83018461074d565b92915050565b5f61077f8261071d565b9050919050565b61078f81610775565b82525050565b5f6020820190506107a85f830184610786565b92915050565b6107b7816106bf565b82525050565b5f6020820190506107d05f8301846107ae565b92915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b6108088161073c565b82525050565b5f61081983836107ff565b60208301905092915050565b5f602082019050919050565b5f61083b826107d6565b61084581856107e0565b9350610850836107f0565b805f5b83811015610880578151610867888261080e565b975061087283610825565b925050600181019050610853565b5085935050505092915050565b5f6020820190508181035f8301526108a58184610831565b905092915050565b5f8160601b9050919050565b5f6108c3826108ad565b9050919050565b5f6108d4826108b9565b9050919050565b6108ec6108e782610775565b6108ca565b82525050565b5f819050919050565b61090c610907826106bf565b6108f2565b82525050565b5f61091d82856108db565b60148201915061092d82846108fb565b6020820191508190509392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f610974826106bf565b915061097f836106bf565b92508261098f5761098e61093d565b5b828206905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6109fe826106bf565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203610a3057610a2f6109c7565b5b600182019050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffdfea2646970667358221220535f3fc6a1d8176244a9eb6a6f9df35f5895b2af939995563b425120e028187a64736f6c63430008140033")]
    #[sol(rpc)]
    Lottery,
    "./contracts/Lottery.json"
}

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://1rpc.io/sepolia".parse()?;
    
    // let signingKey = SigningKey::from_slice("0xD2343D1EB4F9A170D4600F028402849A7FE3Acc5".as_bytes()).unwrap();
    // let signingKey = "0xD2343D1EB4F9A170D4600F028402849A7FE3Acc5".as_bytes();
    // let signer = PrivateKeySigner::from_signing_key(signingKey);
    // println!("Signer {:?}", signer.address());
    // let signer = signer.with_chain_id(Some(11155111));

    let signer: PrivateKeySigner = "fd8ceac1025b8f6a012894c86e0df8f4f3d939db7be46d5a74994e6c8a7421e7".parse().expect("should parse private key");
    let wallet = EthereumWallet::from(signer.clone());

    println!("wallet address: {:?}", wallet);
    let provider = ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http( rpc_url);
    let latest_block = provider.get_block_number().await?;

    println!("Latest block number: {latest_block}");
    


    let lottery = Lottery::new(address!("8ae59e4601842d14aea8c26221f80a263ce65573"),&provider);

    let call_builder = lottery.owner();

    let call_return = call_builder.call().await?;
    println!("Owner address {:?}",call_return._0);

    println!("Balance: {}", lottery.get_balance().call().await?._0);

    println!("Players: {:?}", lottery.get_players().call().await?._0);




    






    // send transaction using 1. contract instance
                            //   2. transaction builder/request
    // let lottery = Lottery::new(alice,&provider);

    // // u8 not accepting cant think of any reason yet

    // let tx = TransactionRequest::default()
    //                                                 .with_from(bob)
    //                                                 .with_nonce(0)
    //                                                 .with_chain_id(1)
    //                                                 .with_value(U256::from(0.000002))
    //                                                 .with_gas_limit(21_000)
    //                                                 .with_max_priority_fee_per_gas(1_000_000_000)
    //                                                 .with_max_fee_per_gas(20_000_000_000);

    
    // // // .with_to(bob).with_value(U256::from(100));

    // // // Send the transaction and wait for inclusion with 3 confirmations.
    // let tx_hash =
    //     provider.send_transaction(tx).await?.with_required_confirmations(3).watch().await?;

    // println!("Sent transaction: {tx_hash}");







    // let signingKey = SigningKey::from_slice(&[2u8;32]).unwrap();
    // let bob = PrivateKeySigner::from_signing_key(signingKey);
    // let signer = bob.clone().with_chain_id(Some(11155111));
    // let bobWallet = EthereumWallet::from(signer);

    // let signingKey = SigningKey::from_slice(&[3u8;32]).unwrap();
    // let charlie = PrivateKeySigner::from_signing_key(signingKey);
    // let signer = charlie.clone().with_chain_id(Some(11155111));
    // let charlieWallet = EthereumWallet::from(signer);

    // println!("Add Bob to lottery {}", lottery.addLottery().from(bob.address()).value(U256::from(0.000002)).send().await?.watch().await?);
    // println!("Add Charlie to lottery {}", lottery.addLottery().from(charlie.address()).value(U256::from(0.000002)).send().await?.watch().await?);

    
    
    
    // let tx = TransactionRequest::default()
    //     .with_nonce(0)
    //     .with_chain_id(provider.get_chain_id().await?)
    //     .with_value(U256::from(100))
    //     .with_gas_limit(21_000)
    //     .with_max_priority_fee_per_gas(1_000_000_000)
    //     .with_max_fee_per_gas(20_000_000_000);

    // // Send the transaction and wait for the broadcast.
    // let pending_tx = provider.send_transaction(tx).await?;

    // println!("Pending transaction... {}", pending_tx.tx_hash());

    
    





    
    
    
    
    
    // let gas = lottery.addLottery().estimate_gas().await?;
    let builder = lottery.addLottery().value(U256::from(20000));
    // println!("Estimated gas: {gas}");
    let tx_hash = builder.send().await?.watch().await?;


    println!("Players: {:?}", lottery.get_players().call().await?._0);
    // println!("Added me to lottery: {:?}", tx_hash);

    // Increment the number to 43.
    // let builder = lottery.addLottery().from(bob).value(U256::from(43));
    // let tx_hash = builder.send().await?.watch().await?;

    //according to forums this happend due to incorect way of deserialization

    // println!("Incremented number: {tx_hash}");
    // let players = lottery.get_players().call().await;
    // match players {
    //     Ok(players) => {
           
    //     },
    //     Err(e) => {
    //         println!("Error: {}", e);
    //     }
        
    // // }
    // let players = lottery.get_players().call().await?;

    // println!("Players: {}",players._0.len());
    // for i in players{
    //     println!("Player: {}", i);
    // }
    // println!("Contract balance: {:?}", lottery.get_players().gas( 25000).call().await?._0);


    // Retrieve the number, which should be 43.
    // let builder = lottery.winner().from(alice);
    // let number = builder.call().await?;
    // let win = lottery.get_winners(U256::from(1)).call().await?._0;
    // println!("Retrieved winner address: {:?}",win );
    

    Ok(())
}