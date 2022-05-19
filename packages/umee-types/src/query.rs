use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{CustomQuery};
use crate::query_leverage::{UmeeQueryLeverage, BorrowParams, ASSIGNED_QUERY_GET_BORROW};

// Define the implementation necessary for cosmwasm "custom" queries
impl CustomQuery for StructUmeeQuery {}
impl CustomQuery for UmeeQuery {}

// UmeeQuery combines all the native modules from umee as enum
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UmeeQuery {
  // GetBorrow returns an sdk.Coin representing how much of a given denom a
  // borrower currently owes. Expect to returns BorrowResponse.
  Leverage(UmeeQueryLeverage),
}

// StructUmeeQuery expected structure to query umee native modules
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StructUmeeQuery {
  assigned_query: u16,
  get_borrow: Option<BorrowParams>,
}

// Defines all the implementation related to the StructUmeeQuery
// like creating new query structs, it is needed because
// the fields inside the struct are private, to avoid missmatching
// the query property with the assigned_query field
impl StructUmeeQuery {
  // creates a new get_borrow query
  pub fn get_borrow(borrow_params: BorrowParams) -> StructUmeeQuery {
    StructUmeeQuery{
      assigned_query: ASSIGNED_QUERY_GET_BORROW,
      get_borrow: Some(borrow_params),
    }
  }
}


