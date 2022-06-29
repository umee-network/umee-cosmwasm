use crate::query_leverage::{
  BorrowParams, LeverageParametersParams, RegisteredTokensParams, UmeeQueryLeverage,
  ASSIGNED_QUERY_GET_BORROW, ASSIGNED_QUERY_LEVERAGE_PARAMS, ASSIGNED_QUERY_REGISTERED_TOKENS,
};
use crate::query_oracle::{
  ExchangeRateBaseParams, UmeeQueryOracle, ASSIGNED_QUERY_GET_EXCHANGE_RATE_BASE,
};
use cosmwasm_std::CustomQuery;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Define the implementation necessary for cosmwasm "custom" queries
impl CustomQuery for StructUmeeQuery {}
impl CustomQuery for UmeeQuery {}

// UmeeQuery combines all the native modules from umee as enum
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UmeeQuery {
  // Leverage wraps all the query enums from the leverage module
  Leverage(UmeeQueryLeverage),
  // Oracle wraps all the query enums from the oracle module
  Oracle(UmeeQueryOracle),
}

// StructUmeeQuery expected structure to query umee native modules
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StructUmeeQuery {
  assigned_query: u16,
  get_borrow: Option<BorrowParams>,
  get_exchange_rate_base: Option<ExchangeRateBaseParams>,
  registered_tokens: Option<RegisteredTokensParams>,
  leverage_parameters: Option<LeverageParametersParams>,
}

// Defines all the implementation related to the StructUmeeQuery
// like creating new query structs, it is needed because
// the fields inside the struct are private, to avoid missmatching
// the query property with the assigned_query field
impl StructUmeeQuery {
  // creates a new get_borrow query.
  pub fn get_borrow(borrow_params: BorrowParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_GET_BORROW,
      get_borrow: Some(borrow_params),
      get_exchange_rate_base: None,
      registered_tokens: None,
      leverage_parameters: None,
    }
  }
  // creates a new get_exchange_rate_Base query.
  pub fn get_exchange_rate_base(
    exchange_rate_base_params: ExchangeRateBaseParams,
  ) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_GET_EXCHANGE_RATE_BASE,
      get_borrow: None,
      get_exchange_rate_base: Some(exchange_rate_base_params),
      registered_tokens: None,
      leverage_parameters: None,
    }
  }
  // creates a new registered_tokens query.
  pub fn registered_tokens(registered_tokens_params: RegisteredTokensParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_REGISTERED_TOKENS,
      get_borrow: None,
      get_exchange_rate_base: None,
      registered_tokens: Some(registered_tokens_params),
      leverage_parameters: None,
    }
  }
  // creates a new leverage_parameters query.
  pub fn leverage_parameters(
    leverage_parameters_params: LeverageParametersParams,
  ) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_LEVERAGE_PARAMS,
      get_borrow: None,
      get_exchange_rate_base: None,
      registered_tokens: None,
      leverage_parameters: Some(leverage_parameters_params),
    }
  }
}
