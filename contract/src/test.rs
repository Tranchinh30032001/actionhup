#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::storage::StorageManagement;
    use crate::CategoryToken;
    use crate::Contract;
    use near_sdk::json_types::U128;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, AccountId, VMContext, ONE_NEAR};

    fn get_context() -> VMContext {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(accounts(1))
            .predecessor_account_id(accounts(1))
            .is_view(false)
            .storage_usage(100000);
        builder.build()
    }

    #[test]
    #[should_panic(expected = "Required attached deposit of at least 1 yoctoNEAR")]
    fn deposit() {
        let context = get_context();
        testing_env!(context);
        let ft_contract = AccountId::new_unchecked(String::from("ft.testnet"));
        let mut contract = Contract::new(ft_contract);
        contract.deposit(U128(100), CategoryToken::Near)
    }

    #[test]
    #[should_panic(expected = "The attached_deposit must equal to the amount")]
    fn deposit_native_token_attached() {
        let mut context = get_context();
        context.attached_deposit = 101;
        testing_env!(context);
        let ft_contract = AccountId::new_unchecked(String::from("ft.testnet"));
        let mut contract = Contract::new(ft_contract);
        contract.deposit(U128(100), CategoryToken::Near)
    }

    #[test]
    fn deposit_native_token() {
        let mut context = get_context();
        context.attached_deposit = 100;
        testing_env!(context);
        let ft_contract = AccountId::new_unchecked(String::from("ft.testnet"));
        let mut contract = Contract::new(ft_contract);
        contract.deposit(U128(100), CategoryToken::Near);
        assert_eq!(
            contract
                .accounts
                .get(&accounts(1))
                .unwrap()
                .get(&CategoryToken::Near)
                .unwrap(),
            100
        );
        contract.deposit(U128(100), CategoryToken::Near);
        assert_eq!(
            contract
                .accounts
                .get(&accounts(1))
                .unwrap()
                .get(&CategoryToken::Near)
                .unwrap(),
            200
        );
    }

    #[test]
    fn deposit_fungible_token() {
        let mut context = get_context();
        // for testing purpose
        context.attached_deposit = ONE_NEAR;
        testing_env!(context);
        let ft_contract = AccountId::new_unchecked(String::from("ft.testnet"));
        let mut contract = Contract::new(ft_contract);
        contract.storage_deposit(Some(accounts(1)), None);

        contract.deposit(U128(100), CategoryToken::Vin);
        assert_eq!(
            contract
                .accounts
                .get(&accounts(1))
                .unwrap()
                .get(&CategoryToken::Vin)
                .unwrap(),
            100
        );
        contract.deposit(U128(100), CategoryToken::Vin);
        assert_eq!(
            contract
                .accounts
                .get(&accounts(1))
                .unwrap()
                .get(&CategoryToken::Vin)
                .unwrap(),
            200
        );
    }
}
