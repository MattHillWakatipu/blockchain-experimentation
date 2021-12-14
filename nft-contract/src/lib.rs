use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, ValidAccountId, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise,
};

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::approval::*;
pub use crate::royalty::*;

mod internal;
mod approval; 
mod enumeration; 
mod metadata; 
mod mint; 
mod nft_core; 
mod royalty;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,
    
    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTMetadata>,

    pub vaxxxed: UnorderedSet<AccountId>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NftMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
    VAXXXED,
}

#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]
    pub fn new_default_meta(owner_id: ValidAccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in 
        Self::new(
            owner_id,
            NFTMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "thevarus".to_string(),
                symbol: "VARUS".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id. 
    */
    #[init]
    pub fn new(owner_id: ValidAccountId, metadata: NFTMetadata) -> Self {
        //create a variable of type Self with all the fields initialized. 
        let this = Self {
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            //set the owner_id field equal to the passed in owner_id. 
            owner_id: owner_id.into(), //.into() converts from a ValidAccountId to an AccountId
            metadata: LazyOption::new(
                StorageKey::NftMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            vaxxxed: UnorderedSet::new(StorageKey::VAXXXED.try_to_vec().unwrap(),)
        };

        //return the Contract object
        this
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    use near_sdk::test_utils::test_env::{alice, bob};

    /// Create a virtual blockchain from input parameters
    fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: "contract.testnet".to_string(),
            signer_account_id: alice(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: 10u128.pow(24) as Balance,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    /// Helper function to construct a valid account from input string
    fn valid_account(input : &str) -> ValidAccountId {
        ValidAccountId::try_from(input).expect("not a valid account id")
    }

    #[test]
    fn check_mint() {
        let context = get_context(alice(), 0);
        testing_env!(context);
        let mut contract = Contract::new_default_meta(valid_account("contract.near"));


    }


    // Vaxxx Tests
    #[test]
    /// Ensure initialisation of metadata works and that the vaxxx list begins empty
    fn check_initialisation() {
        let context = get_context(alice(), 0);
        testing_env!(context);
        let mut contract = Contract::new_default_meta(valid_account("contract.near"));
        assert_eq!(0, contract.vaxxxed.len(), "Expected vaxxxed to be an empty vector.");

        let option = contract.metadata.take().unwrap();
        assert_eq!("nft-1.0.0", option.spec, "Expected different spec.");
        assert_eq!("thevarus", option.name, "Expected different name.");
        assert_eq!("VARUS",option.symbol,"Expected different symbol.");
    }

    #[test]
    /// Check that vaxxx function adds to the vaxxxed list
    fn vaxxx_adds_to_vaxxxed() {
        let context = get_context(bob(), 0);
        testing_env!(context);
        let mut contract = Contract::new_default_meta(valid_account("contract.near"));
        assert_eq!(0, contract.vaxxxed.len(), "Expected empty vaxxx list."); // Sanity check

        // vaxxx alice and bob
        contract.vaxxx(valid_account("alice.near"));
        contract.vaxxx(valid_account("bob.near"));

        // ensure vaxxx list now contains both alice and bob
        assert_eq!(2, contract.vaxxxed.len(), "Expected single addition to vaxxx list.");
        contract.vaxxxed.contains(&alice());
        contract.vaxxxed.contains(&bob());
    }

    #[test]
    fn check_vaxxx_pass() {
        let context = get_context(bob(), 0);
        testing_env!(context);
        let mut contract = Contract::new_default_meta(valid_account("contract.near"));
        assert_eq!(0, contract.vaxxxed.len(), "Expected empty vaxxx list."); // Sanity check

        contract.vaxxx(valid_account("alice.near")); // Vaxxx alice
        assert!(contract.vaxxx_pass(valid_account("alice.near")), "Expected alice to be vaxxxed");
    }

    #[test]
    /// Check that the vaxxx_list contains all of the added addresses
    fn check_vaxxx_list() {
        let context = get_context(bob(), 0);
        testing_env!(context);
        let mut contract = Contract::new_default_meta(valid_account("contract.near"));
        assert_eq!(0, contract.vaxxxed.len(), "Expected empty vaxxx list."); // Sanity check

        // Vaxxx alice and bob
        contract.vaxxx(valid_account("alice.near"));
        contract.vaxxx(valid_account("bob.near"));

        // Check vaxxx_list
        let vaxxxed_vector = contract.vaxxx_list();
        assert_eq!("alice.near", vaxxxed_vector.get(0).unwrap(), "");
        assert_eq!("bob.near", vaxxxed_vector.get(1).unwrap(), "");
    }
}