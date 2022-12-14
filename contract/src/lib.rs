use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;

use near_contract_standards::fungible_token::core::FungibleTokenCore;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, PromiseOrValue};
use std::str;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    random_seed: LazyOption<String>,
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/png;base64,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";

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(owner_id: AccountId, total_supply: U128) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let mut this = Self {
            random_seed: LazyOption::new(b"s", None),
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(
                b"m".to_vec(),
                Some(&FungibleTokenMetadata {
                    spec: FT_METADATA_SPEC.to_string(),
                    name: "Santa Token".to_string(),
                    symbol: "STT".to_string(),
                    icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                    reference: None,
                    reference_hash: None,
                    decimals: 0,
                }),
            ),
        };
        this.token.internal_register_account(&owner_id);
        this.token.internal_deposit(&owner_id, total_supply.into());
        this
    }

    pub fn happy_new_year(&mut self) {
        assert!(env::block_timestamp() >= 1672560000, "Too early for a surprise");
        self.random_seed.set(&bs58::encode(env::random_seed()).into_string());
    }

    pub fn get_seed(&self) -> Option<String> {
        if self.random_seed.is_some() {
            return Some(self.random_seed.get().unwrap());
        }
        None
    }
}

#[near_bindgen]
impl FungibleTokenCore for Contract {
    #[payable]
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>) {
        assert!(self.random_seed.is_none(), "transfer frozen");
        if self.token.accounts.get(&receiver_id).is_none() {
            self.token.internal_register_account(&receiver_id);
        }
        self.token.ft_transfer(receiver_id, amount, memo);
    }

    #[payable]
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        assert!(self.random_seed.is_none(), "transfer frozen");
        self.token.ft_transfer_call(receiver_id, amount, memo, msg)
    }

    fn ft_total_supply(&self) -> U128 {
        self.token.ft_total_supply()
    }

    fn ft_balance_of(&self, account_id: AccountId) -> U128 {
        self.token.ft_balance_of(account_id)
    }
}

near_contract_standards::impl_fungible_token_storage!(Contract, token);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}
