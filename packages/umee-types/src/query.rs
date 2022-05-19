use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{CustomQuery};
use crate::query_leverage::{UmeeQueryLeverage, BorrowParams, ASSIGNED_QUERY_GET_BORROW};

// Define the implementation necessary for cosmwasm "custom" queries
impl CustomQuery for StructUmeeQuery {}
impl CustomQuery for UmeeQuery {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UmeeQuery {
  // GetBorrow returns an sdk.Coin representing how much of a given denom a
  // borrower currently owes. Expect to returns BorrowResponse.
  Leverage(UmeeQueryLeverage),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StructUmeeQuery {
  assigned_query: u16,
  get_borrow: Option<BorrowParams>,
}

impl StructUmeeQuery {
  pub fn get_borrow(borrow_params: BorrowParams) -> StructUmeeQuery {
    StructUmeeQuery{
      assigned_query: ASSIGNED_QUERY_GET_BORROW,
      get_borrow: Some(borrow_params),
    }
  }
}


