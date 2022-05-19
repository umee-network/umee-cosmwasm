use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Coin, Addr};

// All the queries must have an assigned query
pub const ASSIGNED_QUERY_GET_BORROW: u16 = 1;

// UmeeQueryLeverage defines all the available queries
// for the umee leverage native module
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UmeeQueryLeverage {
  // GetBorrow returns an sdk.Coin representing how much of a given denom a
  // borrower currently owes. Expect to returns BorrowResponse.
  GetBorrow(BorrowParams)
}

// BorrowParams params to query GetBorrow
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BorrowParams {
  pub borrower_addr: Addr,
  pub denom: String,
}

// BorrowResponse response struct of GetBorrow query
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BorrowResponse {
  pub borrowed_amount: Coin,
}
