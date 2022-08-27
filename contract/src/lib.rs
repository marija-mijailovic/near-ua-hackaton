/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::{env, log, near_bindgen, AccountId, Promise, PromiseError};

pub mod external;
pub use crate::external::*;

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub my_ft_account: AccountId,
    pub message: String,
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        //env::current_account_id()
        Self{
            my_ft_account: "dev-1661607508862-21282125247023".parse().unwrap(),
            message: DEFAULT_MESSAGE.to_string()
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_greeting(&self) -> String {
        return self.message.clone();
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, message: String) {
        // Use env::log to record logs permanently to the blockchain!
        log!("Saving greeting {}", message);
        self.message = message;
    }

    // Public method
    pub fn get_metadata(&self) -> Promise {
        let promise = my_ft::ext(self.my_ft_account.clone()).ft_metadata();
        return promise.then(
            Self::ext(env::current_account_id())
            .get_metadata_callback()
        )
    }

    #[private]
    pub fn get_metadata_callback(&self, #[callback_result] call_result: Result<FungibleTokenMetadata, PromiseError>) -> FungibleTokenMetadata {
        if call_result.is_err() {
            panic!("There was an error contacting My FT contract");
        }
    
        let metadata: FungibleTokenMetadata = call_result.unwrap();
        metadata
    }

    // Public method
    pub fn get_balance_of(&self, account_id: String) -> Promise {
        let promise = my_ft::ext(self.my_ft_account.clone()).ft_balance_of(account_id);
        return promise.then(
            Self::ext(env::current_account_id())
            .get_balance_of_callback()
        )
    }

    #[private]
    pub fn get_balance_of_callback(&self, #[callback_result] call_result: Result<String, PromiseError>) -> String {
        if call_result.is_err() {
            panic!("There was an error contacting My FT contract");
        }
    
        let balance: String = call_result.unwrap();
        balance
    }

    // Public method
    #[payable]
    pub fn transfer(&mut self, receiver_id: String, amount: String) -> Promise {
        let promise = my_ft::ext(self.my_ft_account.clone())
            .with_attached_deposit(1)
            .ft_transfer(receiver_id, amount);
        return promise.then(
            Self::ext(env::current_account_id())
            .transfer_callback()
        )
    }

    #[private]
    pub fn transfer_callback(&mut self, #[callback_result] call_result: Result<(), PromiseError>) -> bool {
        if call_result.is_err() {
            panic!("There was an error contacting My FT contract");
        }
    
        env::log_str("Transfering token success");
        return true
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            contract.get_greeting(),
            "Hello".to_string()
        );
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            contract.get_greeting(),
            "howdy".to_string()
        );
    }
}
