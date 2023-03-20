use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault, StorageUsage,
};
mod external;
mod internal;
mod storage;
mod test;
mod utils;
use external::*;
use storage::StorageManagement;
use utils::*;

pub const FT_TRANSFER_GAS: Gas = Gas(10_000_000_000_000);

#[derive(BorshStorageKey, BorshSerialize, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum CategoryToken {
    Near,
    Vin,
}
#[derive(BorshSerialize, BorshStorageKey)]

pub enum Prefix {
    Accounts,
    Tokens,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub token_address: AccountId,
    pub accounts: LookupMap<AccountId, LookupMap<CategoryToken, Balance>>,
    /// The bytes for the largest possible account ID that can be registered on the contract
    pub bytes_for_longest_account_id: StorageUsage,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(token_address: AccountId) -> Self {
        let mut this = Self {
            token_address,
            accounts: LookupMap::new(Prefix::Accounts),
            bytes_for_longest_account_id: 0,
        };

        // todo multiple token
        this.measure_bytes_for_longest_account_id(&CategoryToken::Vin);

        this
    }

    #[payable]
    pub fn deposit(&mut self, amount: U128, category: CategoryToken) {
        assert_at_least_one_yocto();
        let init_storage = env::storage_usage();

        // check if sender haven't registered yet => SC register for sender.
        if category != CategoryToken::Near {
            if !self.is_registered_account(&env::predecessor_account_id(), &category) {
                self.storage_deposit(None, None);
            }
        }
        let account_id = env::predecessor_account_id();

        if category == CategoryToken::Near {
            let attached_deposit = env::attached_deposit();
            if attached_deposit != amount.into() {
                env::panic_str("The attached_deposit must equal to the amount");
            }
        } else {
            ext_ft_fungible_token::ext(self.token_address.clone())
                .with_attached_deposit(1)
                .with_static_gas(FT_TRANSFER_GAS)
                .ft_transfer(
                    env::current_account_id(),
                    amount.into(),
                    Some(String::from("hehe")),
                );
        }
        if category == CategoryToken::Vin {
            refund_deposit(init_storage);
        }

        self.internal_deposit(&account_id, &category, amount.into());
    }

    // View method
    pub fn get_balance(&self, account_id: &AccountId, token: &CategoryToken) -> Balance {
        let balance = self.internal_unwrap_balance(account_id, token);

        balance
    }

    pub fn register_account(&mut self, account_id: &AccountId, token: &CategoryToken) {
        self.internal_register_account(&account_id, &token);
        // will return 0 if the account_id has deposited Near to the smartcontract;
    }
}
