//! Example of deploying a contract from Solidity code using the `sol!` macro to Anvil and
//! interacting with it.

use alloy::{network::{EthereumWallet, NetworkWallet, TransactionBuilder}, node_bindings::Anvil, primitives::{address, bytes, U256, U8}, providers::{Provider, ProviderBuilder, WalletProvider}, rpc::types::TransactionRequest, signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner, Signer}, sol};
use eyre::Result;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    Lottery,
    "./contract/Lottery.json"
}

#[tokio::main]
async fn main() -> Result<()> {

    
        // Signing: Using the private key to generate a signature for a message.
        // Verification: Using the public key to verify the signature of a message, proving that the signature was created by the corresponding private key.
    //Refer do for more methods apart from this like using Mnemonic, AWS,etc signers


    //The encrytion is done by ECDSA algorithm.While signing and recovering/verifying same is done by ECDSA.
    
    //[1u8;32] is a 32 byte array
    let signingKey = SigningKey::from_slice(&[1u8;32]).unwrap();
    //generating the private key
    let signer = PrivateKeySigner::from_signing_key(signingKey);
    //public address
    println!("Signer Address: {}", signer.address());

    //chain id for sepolia is 11155111 check out chainlink for more information
    let signer = signer.with_chain_id(Some(11155111));

    let message = b"hello";
    //message is signed
    let signature = signer.sign_message(message).await?;

    println!("Signature produced by {}: {:?}", signer.address(), signature);
    //recovered from signature
    println!("Signature recovered address: {}", signature.recover_address_from_msg(&message[..])?);

    Ok(())
}