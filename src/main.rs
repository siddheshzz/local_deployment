use std::io::Read;

use alloy::{
    hex,
    node_bindings::Anvil,
    primitives::{
        b256,
        bytes::{self, buf::Chain},
        keccak256, Bytes, FixedBytes, U256,
    },
    providers::{Provider, ProviderBuilder},
    rpc::types::{eth, Filter, TransactionRequest},
    signers::{
        k256::{ecdsa::SigningKey, sha2::digest::Update},
        local::PrivateKeySigner,
        Signer,
    },
    sol,
    sol_types::SolValue,
};
use eyre::Result;
use serde::{Deserialize, Serialize};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc,bytecode="6080604052348015600e575f80fd5b5060185f806101000a81548163ffffffff021916908363ffffffff16021790555061047d8061003c5f395ff3fe608060405234801561000f575f80fd5b506004361061004a575f3560e01c806358022de41461004e578063bbb1b6d81461007e578063eef98b6f146100ae578063f1d876b4146100cc575b5f80fd5b610068600480360381019061006391906102ad565b6100ea565b6040516100759190610350565b60405180910390f35b610098600480360381019061009391906102ad565b610141565b6040516100a59190610378565b60405180910390f35b6100b66101f3565b6040516100c39190610350565b60405180910390f35b6100d4610218565b6040516100e191906103af565b60405180910390f35b5f6001858386866040515f815260200160405260405161010d94939291906103d7565b6020604051602081039080840390855afa15801561012d573d5f803e3d5ffd5b505050602060405103519050949350505050565b5f8061014f868686866100ea565b9050805f60046101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555061019961022b565b865f602081106101ac576101ab61041a565b5b1a60f81b60f81c815f019060ff16908160ff1681525050600a5f806101000a81548163ffffffff021916908363ffffffff160217905550805f015192505050949350505050565b5f60049054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b5f8054906101000a900463ffffffff1681565b60405180602001604052805f60ff1681525090565b5f80fd5b5f819050919050565b61025681610244565b8114610260575f80fd5b50565b5f813590506102718161024d565b92915050565b5f60ff82169050919050565b61028c81610277565b8114610296575f80fd5b50565b5f813590506102a781610283565b92915050565b5f805f80608085870312156102c5576102c4610240565b5b5f6102d287828801610263565b94505060206102e387828801610263565b93505060406102f487828801610263565b925050606061030587828801610299565b91505092959194509250565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61033a82610311565b9050919050565b61034a81610330565b82525050565b5f6020820190506103635f830184610341565b92915050565b61037281610277565b82525050565b5f60208201905061038b5f830184610369565b92915050565b5f63ffffffff82169050919050565b6103a981610391565b82525050565b5f6020820190506103c25f8301846103a0565b92915050565b6103d181610244565b82525050565b5f6080820190506103ea5f8301876103c8565b6103f76020830186610369565b61040460408301856103c8565b61041160608301846103c8565b95945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffdfea264697066735822122082d8c0ffe7a213ce81d6fe380ebcdb2fc550bca8808e458d20f0343b88d5a0b864736f6c634300081a0033")]

    contract RecoverECDSA {
        using ECDSA for bytes32;

        uint32 public Value;
        address public generatedAddress;
        struct Proposal{
            uint8 value;
        }

        constructor(){
            Value = 24;

        }
        function recoverSigner(bytes32 message,bytes32 r,bytes32 s, uint8 v) public pure
            returns (address)
        {
            return ecrecover(message, v, r, s);
        }

        function check(bytes32 message,bytes32 r,bytes32 s, uint8 v) public returns(uint8){
            address signer = recoverSigner(message,r,s,v);
            generatedAddress = signer;
            Proposal memory proposal;
            proposal.value = uint8(message[0]);
            Value = 10;
            return proposal.value;
        }


    }
);

#[derive(Serialize, Deserialize)]
struct Proposal {
    value: u8,
}

impl Proposal {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}

fn convert_u8_to_u32(vec_u8: Vec<u8>) -> Vec<u32> {
    // Ensure the length of vec_u8 is a multiple of 4 (pad if necessary)
    let mut padded_vec = vec_u8;
    while padded_vec.len() % 4 != 0 {
        padded_vec.push(0);  // Add padding bytes (you could also handle this differently)
    }

    // Convert to Vec<u32>
    let mut vec_u32 = Vec::new();
    for chunk in padded_vec.chunks(4) {
        let u32_value = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        vec_u32.push(u32_value);
    }

    vec_u32
}

#[tokio::main]
async fn main() -> Result<()> {
    // let mut msg = b"ssssssssssssssssssssssssssssssss";
    // let mut msg2 = b"44444444444444444444444444444444";

    let p = Proposal { value: 12 };
    println!("{:?}", p.to_bytes());

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

    let signature = signer.sign_message(&p.to_bytes()).await.unwrap();
    let signature2 = signer2.sign_message(&p.to_bytes()).await.unwrap();
    let signature3 = signer3.sign_message(&p.to_bytes()).await.unwrap();

    println!("P TO BYTES{:?}", p.to_bytes());
    let builder = contract.check(
        p.to_bytes().eip712_data_word(),
        signature.r().into(),
        signature.s().into(),
        signature.v().into(),
    );
    let tx_res = builder.send().await;

    // println!("{:?}",proposalValue);

    println!("New value{:?}", contract.Value().call().await?.Value);

    let latest_block = provider.get_block_number().await?;
    
     // Create a filter to get all logs from the latest block.
    //  let filter = Filter::new().event("CheckResult");

    let filter = Filter::new();

     // Get all logs from the latest block that match the filter.
     let logs = provider.get_logs(&filter).await?;
 
     for log in logs {
         println!("{log:?}");
     }      
    // let tx =  provider.get_transaction_by_hash(proposalValue).await?;

    // let logs = provider.get_logs(&filter).await?;

    // println!("Transaction: {:?}", tx);
    // match tx {
    //     Some(transaction) => {
    //         println!("Transaction: {:?}", transaction);
    //         println!("To: {:?}", transaction.inner.to);
    //         println!("From: {:?}", transaction.from);
    //         println!("Value: {:?}", transaction.value);
    //         println!("Gas Price: {:?}", transaction.gas_price);
    //         println!("Data: {:?}", transaction.input);
    //     }
    //     None => {
    //         println!("Transaction not found.");
    //     }
    // }

    Ok(())
}
