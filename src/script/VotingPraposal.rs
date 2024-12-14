use std::io::Read;

use alloy::{
     hex, node_bindings::Anvil, primitives::{b256, bytes::{self, buf::Chain}, Bytes, FixedBytes, U256}, providers::{Provider, ProviderBuilder}, rpc::types::{eth, TransactionRequest}, signers::{k256::{ecdsa::SigningKey, sha2::digest::Update}, local::PrivateKeySigner, Signer}, sol
};
use eyre::{Result};
use serde::{Deserialize, Serialize};


sol!(
    #[allow(missing_docs)]
    #[sol(rpc,bytecode="608060405234801561000f575f80fd5b50604051610ab8380380610ab8833981810160405281019061003191906102a5565b60185f806101000a81548163ffffffff021916908363ffffffff1602179055505f5b81518110156100e3576001828281518110610071576100706102ec565b5b6020026020010151908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508080600101915050610053565b5050610319565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610145826100ff565b810181811067ffffffffffffffff821117156101645761016361010f565b5b80604052505050565b5f6101766100ea565b9050610182828261013c565b919050565b5f67ffffffffffffffff8211156101a1576101a061010f565b5b602082029050602081019050919050565b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6101df826101b6565b9050919050565b6101ef816101d5565b81146101f9575f80fd5b50565b5f8151905061020a816101e6565b92915050565b5f61022261021d84610187565b61016d565b90508083825260208201905060208402830185811115610245576102446101b2565b5b835b8181101561026e578061025a88826101fc565b845260208401935050602081019050610247565b5050509392505050565b5f82601f83011261028c5761028b6100fb565b5b815161029c848260208601610210565b91505092915050565b5f602082840312156102ba576102b96100f3565b5b5f82015167ffffffffffffffff8111156102d7576102d66100f7565b5b6102e384828501610278565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b610792806103265f395ff3fe608060405234801561000f575f80fd5b506004361061004a575f3560e01c80630bf4fe671461004e57806342f2bdbe1461006a578063da58c7d91461009a578063f1d876b4146100ca575b5f80fd5b610068600480360381019061006391906104e2565b6100e8565b005b610084600480360381019061007f919061053c565b610186565b6040516100919190610582565b60405180910390f35b6100b460048036038101906100af91906105ce565b6101a3565b6040516100c19190610638565b60405180910390f35b6100d26101de565b6040516100df919061066f565b60405180910390f35b5f6100f383836101f1565b90505f6100ff82610261565b9050801561014e5760025f8581526020019081526020015f205f81819054906101000a900460ff1680929190610134906106b5565b91906101000a81548160ff021916908360ff160217905550505b6002805f8681526020019081526020015f205f9054906101000a900460ff1660ff16106101805761017f6002610303565b5b50505050565b6002602052805f5260405f205f915054906101000a900460ff1681565b600181815481106101b2575f80fd5b905f5260205f20015f915054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b5f8054906101000a900463ffffffff1681565b5f805f806101fe85610325565b8093508194508295505050506001868484846040515f815260200160405260405161022c94939291906106ec565b6020604051602081039080840390855afa15801561024c573d5f803e3d5ffd5b50505060206040510351935050505092915050565b5f805f90505b6001805490508110156102f957600181815481106102885761028761072f565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff16036102ec5760019150506102fe565b8080600101915050610267565b505f90505b919050565b805f806101000a81548163ffffffff021916908363ffffffff16021790555050565b5f805f6041845114610335575f80fd5b5f805f602087015192506040870151915060608701515f1a90508083839550955095505050509193909250565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b61038581610373565b811461038f575f80fd5b50565b5f813590506103a08161037c565b92915050565b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6103f4826103ae565b810181811067ffffffffffffffff82111715610413576104126103be565b5b80604052505050565b5f610425610362565b905061043182826103eb565b919050565b5f67ffffffffffffffff8211156104505761044f6103be565b5b610459826103ae565b9050602081019050919050565b828183375f83830152505050565b5f61048661048184610436565b61041c565b9050828152602081018484840111156104a2576104a16103aa565b5b6104ad848285610466565b509392505050565b5f82601f8301126104c9576104c86103a6565b5b81356104d9848260208601610474565b91505092915050565b5f80604083850312156104f8576104f761036b565b5b5f61050585828601610392565b925050602083013567ffffffffffffffff8111156105265761052561036f565b5b610532858286016104b5565b9150509250929050565b5f602082840312156105515761055061036b565b5b5f61055e84828501610392565b91505092915050565b5f60ff82169050919050565b61057c81610567565b82525050565b5f6020820190506105955f830184610573565b92915050565b5f819050919050565b6105ad8161059b565b81146105b7575f80fd5b50565b5f813590506105c8816105a4565b92915050565b5f602082840312156105e3576105e261036b565b5b5f6105f0848285016105ba565b91505092915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610622826105f9565b9050919050565b61063281610618565b82525050565b5f60208201905061064b5f830184610629565b92915050565b5f63ffffffff82169050919050565b61066981610651565b82525050565b5f6020820190506106825f830184610660565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6106bf82610567565b915060ff82036106d2576106d1610688565b5b600182019050919050565b6106e681610373565b82525050565b5f6080820190506106ff5f8301876106dd565b61070c6020830186610573565b61071960408301856106dd565b61072660608301846106dd565b95945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffdfea264697066735822122026d569fe1a68e82b2750964d4c117ce9cd6e8e7e290835bb61ee781445fa867264736f6c634300081a0033")]
    
    contract Voting {
        using ECDSA for bytes32;
    
        uint32 public Value;
        address[] public voters;
        mapping(bytes32 => uint8) public Praposals;
        constructor(address[] memory v){
            Value = 24;
            for (uint i = 0; i < v.length; i++) {
                voters.push(v[i]);
            }
        }
        function _verify(address voter) internal view returns(bool){
            for(uint i=0;i<voters.length;i++){
                if (voter==voters[i]){
                    return true;
                }
            }
            return false;
        }
        function splitSignature(bytes memory sig)
            internal
            pure
            returns (uint8, bytes32, bytes32)
        {
            require(sig.length == 65);
    
            bytes32 r;
            bytes32 s;
            uint8 v;
    
            assembly {
                // first 32 bytes, after the length prefix
                r := mload(add(sig, 32))
                // second 32 bytes
                s := mload(add(sig, 64))
                // final byte (first byte of the next 32 bytes)
                v := byte(0, mload(add(sig, 96)))
            }
    
            return (v, r, s);
        }
    
        function recoverSigner(bytes32 message, bytes memory sig)
            internal
            pure
            returns (address)
        {
            uint8 v;
            bytes32 r;
            bytes32 s;
    
            (v, r, s) = splitSignature(sig);
    
            return ecrecover(message, v, r, s);
        }
    
        function check(bytes32 message, bytes memory sig) public {
            address signer = recoverSigner(message, sig);
            bool isVoter = _verify(signer);
            if(isVoter){
                Praposals[message]++; 
            }
            if(Praposals[message]>= 2){
                _updateValue(2);
            }
        }
        function _updateValue(uint32 num) internal{
            Value = num;
    
        }
    
    }
);


#[derive(Serialize, Deserialize)]
struct Proposal{
    value:u8,
}


impl Proposal { 

    pub fn to_bytes (&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }

}


#[tokio::main]
async fn main() -> Result<()> {
    let mut msg=b"ssssssssssssssssssssssssssssssss";
    let mut msg2 = b"44444444444444444444444444444444";


    let p = Proposal {
        value:12
    };
    println!("{:?}", p.to_bytes());


    let provider = ProviderBuilder::new().with_recommended_fillers().on_anvil_with_wallet();
    let a = provider.get_accounts().await.unwrap();
    
    let mut addresses = Vec::new();

    //Generates private key
    let signer = PrivateKeySigner::from_slice(&[1 as u8;32]).unwrap();
    addresses.push(signer.address());
    let signer2 = PrivateKeySigner::from_slice(&[2 as u8;32]).unwrap();
    addresses.push(signer2.address());
    let signer3 = PrivateKeySigner::from_slice(&[3 as u8;32]).unwrap();
    addresses.push(signer3.address());
    


    let contract = Voting::deploy(&provider,addresses).await.unwrap();

    println!("Old value{:?}",contract.Value().call().await.unwrap().Value);
    
    //generates signature which is of 65 bytes = r,s,v
    //r = part coming from signing process
    //s = part coming from private key of signer(off couse not the key but derived from private key)
    //v = recovery pattern
    //It specifies which of the two possible elliptic curve points (r, s) corresponds to the correct signature.
    //The value of v is usually either 27 or 28 in Ethereum (for ECDSA), though other values might be used
    // in different contexts (e.g., 0 or 1 for other systems).
    // The v value is necessary to recover the public key from the signature.
    // In Ethereum, this value helps determine which of the two possible public keys produced by the
    // elliptic curve signature process should be used to verify the signature.


    let signature = signer.sign_message(msg).await.unwrap();
    let signature2 = signer2.sign_message(msg).await.unwrap();
    let signature3 = signer3.sign_message(msg2).await.unwrap();
    
    contract.check(FixedBytes::from(msg),Bytes::from(signature.as_bytes())).call().await.unwrap();

    contract.check(FixedBytes::from(msg),Bytes::from(signature2.as_bytes())).call().await.unwrap();

    contract.check(FixedBytes::from(msg2),Bytes::from(signature3.as_bytes())).call().await.unwrap();

    println!("New value{:?}",contract.Value().call().await?.Value);

    
    // let accounts = provider.get_accounts().await?;
    // let voters = accounts[..10].to_vec();
    // let mut addresses = Vec::new();
    // // //for loop with range from 0 to 10
    // // for i in 0..10 {
    // //     // let signingKey = SigningKey::from_slice(&[i as u8;32])?;
    //     let signer = PrivateKeySigner::from_slice(&[1 as u8;32]).unwrap();
    //     addresses.push(signer);

    //     let signer = PrivateKeySigner::from_slice(&[2 as u8;32]).unwrap();
    //     addresses.push(signer);

    //     let signer = PrivateKeySigner::from_slice(&[3 as u8;32]).unwrap();
    //     addresses.push(signer);
    // // // }

    // println!("{:?}",addresses);


    // // let signingKey = SigningKey::from_slice(&[3u8;32]).unwrap();
    // // let signer = PrivateKeySigner::from_signing_key(signingKey);


    
    // let contract = Voting::deploy(&provider,voters).await.unwrap();

    // println!("{:?}",contract.Value().call().await?.Value);

    // // contract.check(praposal, voter);
    // // message and signature
    // // send signature as well as message to contract
    // // let signer1 = PrivateKeySigner::address(accounts[3].);

    // let signingKey = SigningKey::from_slice(&[1u8;32]).unwrap();
    // //generating the private key
    // let signer = PrivateKeySigner::from_signing_key(signingKey);
    // //public address
    // println!("Signer Address: {}", signer.address());

    // let message = b"24";
    // //message is signed
    // let signature = addresses[0].sign_message(message).await?;

    // println!("Signature produced by {}: {:?}\n", signer.address(), signature);

    // contract.check(msg.,Bytes::from(signature.as_bytes()) ).send().await?.watch().await?;

    // let signingKey = SigningKey::from_slice(&[2u8;32]).unwrap();
    // //generating the private key
    // let signer = PrivateKeySigner::from_signing_key(signingKey);
    // //public address
    // println!("Signer Address: {}", addresses[1].address());

    // let message = b"24";
    // //message is signed
    // let signature = addresses[1].sign_message(message).await?;

    // println!("Signature produced by {}: {:?}\n", signer.address(), signature);

    // contract.check(,Bytes::from(signature.as_bytes()) ).send().await?.watch().await?;

    

    // let signingKey = SigningKey::from_slice(&[3u8;32]).unwrap();
    // //generating the private key
    // let signer = PrivateKeySigner::from_signing_key(signingKey);
    // //public address
    // println!("Signer Address: {}", addresses[2].address());

    // let message = b"4";
    // //message is signed
    // let signature = addresses[2].sign_message(message).await?;

    // println!("Signature produced by {}: {:?}\n", signer.address(), signature);

    // contract.check(FixedBytes::from(msg),Bytes::from(signature.as_bytes()) ).send().await?.watch().await?;

    // // println!("{:?}",contract.Praposals(0))

    // println!("{:?}",contract.Value().call().await?.Value);

    // //hard - deserialize the sent message like create struct in solidity contract






















    // println!("{:?}", p.serialize(serializers::bincode::Serializer::new(Vec::new())));
    
    // encode -bytes
    // decode-Struct 


    // contract -multiple people can vote

    // struct serialize and deserialize



    // participants - 10

    // struct feeeup{
    // u32
    // }

    // 10 wallet 1 praposal sign and submit
    // praposal - 
    // fee 
    // praposalfor fee Update
    // who and howmuch


    // Doubts:-
    // wallet is aacout store on Chain
    // poiner types

    Ok(())
}