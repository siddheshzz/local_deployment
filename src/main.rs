use alloy::{
      primitives::{  keccak256, Bytes}, providers::{Provider, ProviderBuilder},  signers::{k256::ecdsa::{signature::Verifier, Signature, SigningKey, VerifyingKey}, local::PrivateKeySigner, Signer}
};
use eyre::{Result};
use serde::{Deserialize, Serialize};


sol!(
    #[allow(missing_docs)]
    #[sol(rpc,bytecode="608060405234801561000f575f80fd5b50604051610e6f380380610e6f833981810160405281019061003191906102a6565b60185f60146101000a81548163ffffffff021916908363ffffffff1602179055505f5b81518110156100e4576001828281518110610072576100716102ed565b5b6020026020010151908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508080600101915050610054565b505061031a565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61014682610100565b810181811067ffffffffffffffff8211171561016557610164610110565b5b80604052505050565b5f6101776100eb565b9050610183828261013d565b919050565b5f67ffffffffffffffff8211156101a2576101a1610110565b5b602082029050602081019050919050565b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6101e0826101b7565b9050919050565b6101f0816101d6565b81146101fa575f80fd5b50565b5f8151905061020b816101e7565b92915050565b5f61022361021e84610188565b61016e565b90508083825260208201905060208402830185811115610246576102456101b3565b5b835b8181101561026f578061025b88826101fd565b845260208401935050602081019050610248565b5050509392505050565b5f82601f83011261028d5761028c6100fc565b5b815161029d848260208601610211565b91505092915050565b5f602082840312156102bb576102ba6100f4565b5b5f82015167ffffffffffffffff8111156102d8576102d76100f8565b5b6102e484828501610279565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b610b48806103275f395ff3fe608060405234801561000f575f80fd5b5060043610610060575f3560e01c8063238ac9331461006457806342f2bdbe1461008257806348808c10146100b2578063cdd72253146100ce578063da58c7d9146100ec578063f1d876b41461011c575b5f80fd5b61006c61013a565b604051610079919061053a565b60405180910390f35b61009c60048036038101906100979190610597565b61015d565b6040516100a991906105dd565b60405180910390f35b6100cc60048036038101906100c791906107d0565b61017a565b005b6100d66102ae565b6040516100e391906108fd565b60405180910390f35b61010660048036038101906101019190610950565b610339565b604051610113919061053a565b60405180910390f35b610124610374565b6040516101319190610999565b60405180910390f35b5f8054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b6002602052805f5260405f205f915054906101000a900460ff1681565b5f8260405160200161018c9190610a12565b6040516020818303038152906040528051906020012090506101ae8183610389565b5f806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f6102155f8054906101000a900473ffffffffffffffffffffffffffffffffffffffff166103f9565b9050600115158115150361027657600160025f8481526020019081526020015f205f9054906101000a900460ff1661024d9190610a5f565b60025f8481526020019081526020015f205f6101000a81548160ff021916908360ff1602179055505b6002805f8481526020019081526020015f205f9054906101000a900460ff1660ff16106102a8576102a7600261049b565b5b50505050565b6060600180548060200260200160405190810160405280929190818152602001828054801561032f57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116102e6575b5050505050905090565b60018181548110610348575f80fd5b905f5260205f20015f915054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b5f60149054906101000a900463ffffffff1681565b5f805f80610396856104be565b8093508194508295505050506001868484846040515f81526020016040526040516103c49493929190610aa2565b6020604051602081039080840390855afa1580156103e4573d5f803e3d5ffd5b50505060206040510351935050505092915050565b5f805f90505b60018054905081101561049157600181815481106104205761041f610ae5565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff1603610484576001915050610496565b80806001019150506103ff565b505f90505b919050565b805f60146101000a81548163ffffffff021916908363ffffffff16021790555050565b5f805f60418451146104ce575f80fd5b5f805f602087015192506040870151915060608701515f1a90508083839550955095505050509193909250565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610524826104fb565b9050919050565b6105348161051a565b82525050565b5f60208201905061054d5f83018461052b565b92915050565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b61057681610564565b8114610580575f80fd5b50565b5f813590506105918161056d565b92915050565b5f602082840312156105ac576105ab61055c565b5b5f6105b984828501610583565b91505092915050565b5f60ff82169050919050565b6105d7816105c2565b82525050565b5f6020820190506105f05f8301846105ce565b92915050565b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610644826105fe565b810181811067ffffffffffffffff821117156106635761066261060e565b5b80604052505050565b5f610675610553565b9050610681828261063b565b919050565b5f67ffffffffffffffff8211156106a05761069f61060e565b5b6106a9826105fe565b9050602081019050919050565b828183375f83830152505050565b5f6106d66106d184610686565b61066c565b9050828152602081018484840111156106f2576106f16105fa565b5b6106fd8482856106b6565b509392505050565b5f82601f830112610719576107186105f6565b5b81356107298482602086016106c4565b91505092915050565b5f67ffffffffffffffff82111561074c5761074b61060e565b5b610755826105fe565b9050602081019050919050565b5f61077461076f84610732565b61066c565b9050828152602081018484840111156107905761078f6105fa565b5b61079b8482856106b6565b509392505050565b5f82601f8301126107b7576107b66105f6565b5b81356107c7848260208601610762565b91505092915050565b5f80604083850312156107e6576107e561055c565b5b5f83013567ffffffffffffffff81111561080357610802610560565b5b61080f85828601610705565b925050602083013567ffffffffffffffff8111156108305761082f610560565b5b61083c858286016107a3565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b6108788161051a565b82525050565b5f610889838361086f565b60208301905092915050565b5f602082019050919050565b5f6108ab82610846565b6108b58185610850565b93506108c083610860565b805f5b838110156108f05781516108d7888261087e565b97506108e283610895565b9250506001810190506108c3565b5085935050505092915050565b5f6020820190508181035f83015261091581846108a1565b905092915050565b5f819050919050565b61092f8161091d565b8114610939575f80fd5b50565b5f8135905061094a81610926565b92915050565b5f602082840312156109655761096461055c565b5b5f6109728482850161093c565b91505092915050565b5f63ffffffff82169050919050565b6109938161097b565b82525050565b5f6020820190506109ac5f83018461098a565b92915050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f6109e4826109b2565b6109ee81856109bc565b93506109fe8185602086016109cc565b610a07816105fe565b840191505092915050565b5f6020820190508181035f830152610a2a81846109da565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610a69826105c2565b9150610a74836105c2565b9250828201905060ff811115610a8d57610a8c610a32565b5b92915050565b610a9c81610564565b82525050565b5f608082019050610ab55f830187610a93565b610ac260208301866105ce565b610acf6040830185610a93565b610adc6060830184610a93565b95945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffdfea2646970667358221220b62da59ca4bfea4050487b653c8b710caea38e45c3d912cf47b9f67f7842b50664736f6c634300081a0033")]
    

contract Voting {
    using ECDSA for bytes32;

    address public signer;
    uint32 public Value;
    address[] public voters;
    mapping(bytes32 => uint8) public Praposals;
    constructor(address[] memory v){
        Value = 24;
        for (uint i = 0; i < v.length; i++) {
            voters.push(v[i]);
        }
    }

    function getVoters() public view returns(address[] memory){
        return voters;
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

    function check(string memory message1, bytes memory sig) public {
        bytes32 message = keccak256(abi.encode(message1));
        signer = recoverSigner(message, sig);

        
        bool isVoter = _verify(signer);
        if(isVoter == true){
            Praposals[message] = Praposals[message]+1; 
        }
        
        if(Praposals[message]>=2){
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
    // let mut msg=b"ssssssssssssssssssssssssssssssss";
    // let mut msg2 = b"44444444444444444444444444444444";

    let msg="heelo please work";
    let msg2 = "msg2";

    let p = Proposal {
        value:12
    };
    println!("{:?}", p.to_bytes());


    let provider = ProviderBuilder::new().with_recommended_fillers().on_anvil_with_wallet();
    let a = provider.get_accounts().await.unwrap();
    
    let mut addresses = Vec::new();

    //Generates private key
    
    let signing_key = SigningKey::from_slice(&[1u8;32]).unwrap();
    addresses.push(signing_key);

    let signing_key1 = SigningKey::from_slice(&[2u8;32]).unwrap();
    addresses.push(signing_key1);

    let signing_key2 = SigningKey::from_slice(&[3u8;32]).unwrap();
    addresses.push(signing_key2);


    // let signer = PrivateKeySigner::from_slice(&[1 as u8;32]).unwrap();

    // let signer2 = PrivateKeySigner::from_slice(&[2 as u8;32]).unwrap();
    // addresses.push(signer2.address());
    // let signer3 = PrivateKeySigner::from_slice(&[3 as u8;32]).unwrap();
    // addresses.push(signer3.address());
    
    


    let contract = Voting::deploy(&provider,addresses).await.unwrap();
    // println!("Votersaddress: {:?}", contract.getVoters().call().await.unwrap()._0);
    // println!("Old value{:?}",contract.Value().call().await.unwrap().Value);

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


    
    let signature = signer.sign_hash(&keccak256(msg)).await.unwrap();
    let signature2 = signer2.sign_hash(&keccak256(msg)).await.unwrap();
    let signature3 = signer3.sign_hash(&keccak256(msg2)).await.unwrap();


    // let signing_key = SigningKey::(signer).unwrap();
    // let verifying_key = VerifyingKey::from(&signing_key);
    // verification 
    let verifying_key = VerifyingKey::from(signer.credential());
    
    println!("sIGNATURE: {:?} ",signature.with_parity(false));
    let signature = signature.with_parity(false);

    // println!("Verifying key: {:?}", verifying_key);

    //error is to do with impl trait -> verify needs 2# as something whichvis of type S i.e type which impl 
    // println!("Verifying :{:?}",verifying_key.verify(&msg.as_bytes(), &signature.to_k256().unwrap()));  


    match verifying_key.verify(&msg.as_bytes(), &signature.to_k256().unwrap()){
        Ok(_) => println!("Signature is valid"),
        Err(e) => println!("Signature is invalid: {}", e),
    }


    
    // contract.check(msg.to_string(),Bytes::from(signature.as_bytes())).call().await.unwrap();
    // println!("Signature: {:?}",signature);
    // println!("Signature as bytes: {:?}",Bytes::from(signature2.as_bytes()));
    // println!("Signer address decrypted is :{:?}",contract.signer().call().await.unwrap().signer);

    contract.check(msg.to_string(),Bytes::from(signature2.as_bytes())).call().await.unwrap();
    // println!("Signer address decrypted is :{:?}",contract.signer().call().await.unwrap().signer);

    // contract.check(msg2.to_string(),Bytes::from(signature3.as_bytes())).call().await.unwrap();
    // println!("Signer address decrypted is :{:?}",contract.signer().call().await.unwrap().signer);

    // println!("New value{:?}",contract.Value().call().await?.Value);

    //NOTES:- todo()!
    // //hard - deserialize the sent message like create struct in solidity contract
    
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


    Ok(())
}