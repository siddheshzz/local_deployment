    
# Simple contact deployment using alloy

Here we deploy a simple evm smart contract using alloy and anvil.

Steps:-

Create a local blockchain node running on anvil.

Create a provider using ProviderBuilder - on_anvil_with_wallet() helps in running local chain without any specific declaration.

Deploy the contract using ABI or bytecode

Build a tracsaction and send()




# Signing a message

Using SigningKey generate signature
Create a signer using PrivateKeySigner
This can now create a signature

Generates signature which is of 65 bytes = r,s,v
    r = part coming from signing process
    s = part coming from private key of signer(off couse not the key but derived from private key)
    v = recovery pattern
It specifies which of the two possible elliptic curve points (r, s) corresponds to the correct signature. The value of v is usually either 27 or 28 in Ethereum (for ECDSA), though other values might be used in different contexts (e.g., 0 or 1 for other systems). The v value is necessary to recover the public key from the signature. In Ethereum, this value helps determine which of the two possible public keys produced by the elliptic curve signature process should be used to verify the signature.
