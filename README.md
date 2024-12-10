
# Simple contact deployment using alloy

Here we deploy a simple evm smart contract using alloy and anvil.

Steps:-

Create a local blockchain node running on anvil.

Create a provider using ProviderBuilder - on_anvil_with_wallet() helps in running local chain without any specific declaration.

Deploy the contract using ABI or bytecode

Build a tracsaction and send()