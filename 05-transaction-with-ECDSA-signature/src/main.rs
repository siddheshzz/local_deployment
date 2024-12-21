use std::io::Read;

use alloy::{
    hex,
    node_bindings::Anvil,
    primitives::{
        b256,
        bytes::{self, buf::Chain},
        Bytes, FixedBytes, U256,
    },
    providers::{Provider, ProviderBuilder},
    rpc::types::{eth, TransactionRequest},
    signers::{
        k256::{ecdsa::SigningKey, sha2::digest::Update},
        local::PrivateKeySigner,
        Signer,
    },
    sol, sol_types::SolValue,
};
use eyre::Result;
use serde::{Deserialize, Serialize};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc,bytecode="6080604052348015600e575f80fd5b5060185f806101000a81548163ffffffff021916908363ffffffff1602179055506103d28061003c5f395ff3fe608060405234801561000f575f80fd5b506004361061004a575f3560e01c806358022de41461004e578063bbb1b6d81461007e578063eef98b6f1461009a578063f1d876b4146100b8575b5f80fd5b61006860048036038101906100639190610248565b6100d6565b60405161007591906102eb565b60405180910390f35b61009860048036038101906100939190610248565b61012d565b005b6100a26101a3565b6040516100af91906102eb565b60405180910390f35b6100c06101c8565b6040516100cd9190610322565b60405180910390f35b5f6001858386866040515f81526020016040526040516100f99493929190610359565b6020604051602081039080840390855afa158015610119573d5f803e3d5ffd5b505050602060405103519050949350505050565b5f61013a858585856100d6565b9050805f60046101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550600a5f806101000a81548163ffffffff021916908363ffffffff1602179055505050505050565b5f60049054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b5f8054906101000a900463ffffffff1681565b5f80fd5b5f819050919050565b6101f1816101df565b81146101fb575f80fd5b50565b5f8135905061020c816101e8565b92915050565b5f60ff82169050919050565b61022781610212565b8114610231575f80fd5b50565b5f813590506102428161021e565b92915050565b5f805f80608085870312156102605761025f6101db565b5b5f61026d878288016101fe565b945050602061027e878288016101fe565b935050604061028f878288016101fe565b92505060606102a087828801610234565b91505092959194509250565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6102d5826102ac565b9050919050565b6102e5816102cb565b82525050565b5f6020820190506102fe5f8301846102dc565b92915050565b5f63ffffffff82169050919050565b61031c81610304565b82525050565b5f6020820190506103355f830184610313565b92915050565b610344816101df565b82525050565b61035381610212565b82525050565b5f60808201905061036c5f83018761033b565b610379602083018661034a565b610386604083018561033b565b610393606083018461033b565b9594505050505056fea2646970667358221220061990e975611729e0f43c446bc6400d218f32fc88a0a46674462e8aa7f485e964736f6c634300081a0033")]

    contract RecoverECDSA {
        using ECDSA for bytes32;

        uint32 public Value;
        address public generatedAddress;

        constructor(){
            Value = 24;

        }
        function recoverSigner(bytes32 message,bytes32 r,bytes32 s, uint8 v) public pure
            returns (address)
        {
            return ecrecover(message, v, r, s);
        }

        function check(bytes32 message,bytes32 r,bytes32 s, uint8 v) public {
            address signer = recoverSigner(message,r,s,v);
            generatedAddress = signer;
            Value = 10;
        }


    }
);

#[tokio::main]
async fn main() -> Result<()> {
    let mut msg = b"ssssssssssssssssssssssssssssssss";
    let mut msg2 = b"44444444444444444444444444444444";

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .on_anvil_with_wallet();
    let a = provider.get_accounts().await.unwrap();

    let mut addresses = Vec::new();

    //Generates private key
    let signer = PrivateKeySigner::from_slice(&[1 as u8; 32]).unwrap();
    addresses.push(signer.address());
    let signer2 = PrivateKeySigner::from_slice(&[2 as u8; 32]).unwrap();
    addresses.push(signer2.address());
    let signer3 = PrivateKeySigner::from_slice(&[3 as u8; 32]).unwrap();
    addresses.push(signer3.address());

    let contract = RecoverECDSA::deploy(&provider).await.unwrap();

    println!(
        "Old value{:?}",
        contract.Value().call().await.unwrap().Value
    );

    // generates signature which is of 65 bytes = r,s,v
    // r = part coming from signing process
    // s = part coming from private key of signer(off couse not the key but derived from private key)
    // v = recovery pattern
    // It specifies which of the two possible elliptic curve points (r, s) corresponds to the correct signature.
    // The value of v is usually either 27 or 28 in Ethereum (for ECDSA), though other values might be used
    // in different contexts (e.g., 0 or 1 for other systems).
    // The v value is necessary to recover the public key from the signature.
    // In Ethereum, this value helps determine which of the two possible public keys produced by the
    // elliptic curve signature process should be used to verify the signature.

    let signature = signer.sign_message(msg).await.unwrap();
    let signature2 = signer2.sign_message(msg).await.unwrap();
    let signature3 = signer3.sign_message(msg2).await.unwrap();

    contract
        .check(msg.eip712_data_word(), signature.r().eip712_data_word(), signature.s().eip712_data_word(), signature.v().into())
        .send()
        .await?
        .watch()
        .await?;

    println!("New value{:?}", contract.Value().call().await?.Value);

    
    Ok(())
}
