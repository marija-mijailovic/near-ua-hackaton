use near_sdk::{ext_contract};
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;

#[ext_contract(this_contract)]
trait Callbacks {
  fn get_metadata_callback(&self) -> FungibleTokenMetadata;
  fn get_balance_of_callback(&self) -> String;
  fn transfer_callback(&mut self) -> bool;
}

#[ext_contract(my_ft)]
trait MyFT {
  fn ft_metadata(&self) -> FungibleTokenMetadata;
  fn ft_balance_of(&self, account_id: String) -> String;
  fn ft_transfer(&self, receiver_id: String, amount: String) -> bool;
}