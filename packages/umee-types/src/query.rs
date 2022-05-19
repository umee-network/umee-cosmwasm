use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{CustomQuery};
use crate::query_leverage::{UmeeQueryLeverage, BorrowParams};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UmeeQuery {
  // GetBorrow returns an sdk.Coin representing how much of a given denom a
  // borrower currently owes. Expect to returns BorrowResponse.
  // GetBorrow {  borrower_addr: Addr, denom: String },
  Leverage(UmeeQueryLeverage),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StructUmeeQuery {
  assigned_query: u16,
  get_borrow: Option<BorrowParams>,
}

impl CustomQuery for StructUmeeQuery {}
impl CustomQuery for UmeeQuery {}
