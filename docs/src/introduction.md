# Polygon Miden Intro

> *This documentation is still Work In Progress. Some topics have been discussed in greater depth, while others require additional clarification. Sections of this documentation might later be reorganized in order to achieve a better flow.*

## Welcome to the Polygon Miden Documentation
Polygon Miden is a zk-optimized rollup with client-side proving. It is expected to launch a public testnet in Q3. 

Unlike most other rollups, Polygon Miden prioritizes zk-friendliness over EVM compatibility. It also uses a novel, actor-based state model to exploit the full power of a zk-centric design. These design choices allow Polygon Miden to extend Ethereum’s feature set. These features allow developers to create applications currently difficult and impractical on EVM-like systems. 

Polygon Miden will be the first decentralised rollup that leverages execution proofs of concurrent, local transactions. Anyone can execute a transaction and create a STARK-proof for the network. Executing a transaction in Miden means changing the state of a single account. Execution and proving can happen concurrently and privately (locally) in Polygon Miden.

## Polygon Miden creates a new design space secured by Ethereum
Our goal is to not only scale Ethereum but to extend it. Rollups - secured by Ethereum - can be new design spaces and even experimental. This is the place to innovate. The base layer, however, should stay conservative and only slowly evolve to ensure the required safety and stability. 

Like other rollups, we want to scale Ethereum and inherit its security. We want to provide a safe and decentralized environment for composable smart contracts. 

But there is more. Rollups allow the creation of new design spaces while retaining the collateral security of Ethereum. We aim to create a design space for new applications secured by Ethereum.

## Benefits of Polygon Miden

* Ethereum security 
* Developers can build applications not possible on other systems, e.g. 
  * **on-chain order book exchange** due to parallel tx exectuion and updateable transactions
  * **complex, incomplete information games** due to client-side proving and cheap complex computations
  * **safe wallets** due to assets being stored in the accounts and account state can be (partially) hidden
* Lower fees than on other systems due to client-side proving
* Better privacy properties than on Ethereum - first web2 privacy, later even stronger privacy guarantees
* Transactions can be recalled and updated 
* dApps on Miden are safe to use due to account abstraction and modern smart contract languages like Move 