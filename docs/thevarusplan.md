# thevarus Smart Contract & dApp

### Summary

thevarus is an experimental dApp & NFT smart contract designed to automatically spread itself to as many wallets as possible.

### Motivation

The purpose of this project is to gain a better understanding of the blockchain ecosystem on NEAR protocol.

The dApp is composed of 3 elements; the NFT smart contract, the near-api-js and an indexer.




### Internal functions of the Contract

- **Vaxxxinate**: Any AccountId that calls this function pays 1 NEAR and is added to the contract's vaxxxinated list. This prevents this account from being infected with any further viruses. This function does not automatically rid the account of any viruses it already holds and the account can still infect other accounts.


- **Cure**: Calling this method allows the caller to burn any virus NFTs it holds. 


- **Infect**: Creates two new copies from an existing virus and transfers them to two unique accounts.
  - the cost of using this function should be taken from the smart contract account

### External functions for the dApp

- **Check contract balance**: A view call that can be called through the API in order to establish how much Near is held by the contract account. This method will be periodically called and when there is enough Near on the account, the Infect function of the contract will be called.


- **Infect target accounts**: This method is composed of multiple JS subactions and triggers the contract Infect function
  - Pull the list of infected accounts from the smart contract.
  - Identify the latest infected account.
  - Check the selected infected account's transaction history and check whether it has transacted with two unique accounts.
  - If the selected account has not interacted with at least 2 unique accounts, select another infected account and repeat the above check.
  - If selected account history contains two unique accounts(target), call the contract Infect function on the two target accounts.


- **Visualisation**: Indexing infected accounts and vaxxxinated accounts as well as the source of the infection to create a graph diagram. The accounts will serve as vertices and the infect function history will form edges between vertices.
