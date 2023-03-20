use crate::*;
use near_sdk::collections::LookupMap;

impl Contract {
    pub(crate) fn internal_unwrap_balance(
        &self,
        account_id: &AccountId,
        token: &CategoryToken,
    ) -> Balance {
        match token {
            CategoryToken::Near => match self.accounts.get(account_id) {
                Some(token_accounts) => match token_accounts.get(&token) {
                    Some(balance) => balance,
                    None => 0,
                },
                None => 0,
            },
            CategoryToken::Vin => match self.accounts.get(account_id) {
                Some(token_accounts) => match token_accounts.get(&token) {
                    Some(balance) => balance,
                    None => env::panic_str(
                        format!(
                            "The account {} is not registered fungible tokens",
                            &account_id
                        )
                        .as_str(),
                    ),
                },
                None => env::panic_str(
                    format!(
                        "The account {} is not registered fungible token",
                        &account_id
                    )
                    .as_str(),
                ),
            },
        }
    }

    /// Internal method for depositing some amount of native/FTs into an account.
    pub(crate) fn internal_deposit(
        &mut self,
        account_id: &AccountId,
        token: &CategoryToken,
        amount: Balance,
    ) {
        // Get the current balance of the account. If they're not registered, panic.
        let balance = self.internal_unwrap_balance(account_id, token);

        // Add the amount to the balance and insert the new balance into the accounts map
        if let Some(new_balance) = balance.checked_add(amount) {
            let mut token_account: LookupMap<CategoryToken, Balance> =
                LookupMap::new(Prefix::Tokens);
            token_account.insert(token, &new_balance);

            self.accounts.insert(account_id, &token_account);
        } else {
            env::panic_str("Balance overflow");
        }
    }

    pub(crate) fn internal_register_account(
        &mut self,
        account_id: &AccountId,
        token: &CategoryToken,
    ) {
        let mut token_accounts: LookupMap<CategoryToken, Balance> = LookupMap::new(Prefix::Tokens);
        token_accounts.insert(token, &0);
        match self.accounts.insert(account_id, &token_accounts) {
            Some(tokens) => match token {
                CategoryToken::Near => (),
                CategoryToken::Vin => match tokens.get(&token) {
                    Some(_) => env::panic_str("The account is already registered"),
                    None => (),
                },
            },
            None => (),
        };
    }

    /// Internal method for measuring how many bytes it takes to insert the longest possible account ID into our map
    /// This will insert the account, measure the storage, and remove the account. It is called in the initialization function.
    pub(crate) fn measure_bytes_for_longest_account_id(&mut self, token: &CategoryToken) {
        let initial_storage_usage = env::storage_usage();
        let tmp_account_id = AccountId::new_unchecked("a".repeat(64));
        let mut token_accounts: LookupMap<CategoryToken, Balance> = LookupMap::new(Prefix::Tokens);
        token_accounts.insert(token, &0);

        self.accounts.insert(&tmp_account_id, &token_accounts);
        self.bytes_for_longest_account_id = env::storage_usage() - initial_storage_usage;
        self.accounts.remove(&tmp_account_id);
    }

    pub(crate) fn is_registered_account(
        &self,
        account_id: &AccountId,
        token: &CategoryToken,
    ) -> bool {
        // current implementation: 1 fungible token
        match self.accounts.get(account_id) {
            Some(token_account) => match token_account.get(&token) {
                Some(_) => return true,
                None => return false,
            },
            None => return false,
        }
    }
}
