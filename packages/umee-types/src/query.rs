use crate::query_leverage::{
  BorrowedParams, LeverageParametersParams, RegisteredTokensParams, UmeeQueryLeverage,
  ASSIGNED_QUERY_BORROWED, ASSIGNED_QUERY_BORROWED_VALUE, ASSIGNED_QUERY_LEVERAGE_PARAMS,
  ASSIGNED_QUERY_REGISTERED_TOKENS,
};
use crate::query_oracle::{ExchangeRatesParams, UmeeQueryOracle, ASSIGNED_QUERY_EXCHANGE_RATES};
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
  borrowed: Option<BorrowedParams>,
  exchange_rates: Option<ExchangeRatesParams>,
  registered_tokens: Option<RegisteredTokensParams>,
  leverage_parameters: Option<LeverageParametersParams>,
  borrowed_value: Option<BorrowedParams>,
  // loaned,
  // loaned_value,
  // avalable_borrow,
  // borrow_apy,
  // lend_apy,
  // market_size,1
  // token_market_size,
  // reserve_amount,
  // exchange_rate,
  // borrow_limit,
  // liquidation_threshold
  // liquidation_targets
  // market_summary
  // active_exchange_rates
  // feeder_delegation
  // miss_counter,
  // aggregate_prevote
  // aggregate_prevotes
  // aggregate_vote
  // aggregate_votes
  // oracle_params
}

// Defines all the implementation related to the StructUmeeQuery
// like creating new query structs, it is needed because
// the fields inside the struct are private, to avoid missmatching
// the query property with the assigned_query field
impl StructUmeeQuery {
  // creates a new borrowed query.
  pub fn borrowed(borrowed_params: BorrowedParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_BORROWED,
      borrowed: Some(borrowed_params),
      exchange_rates: None,
      registered_tokens: None,
      leverage_parameters: None,
      borrowed_value: None,
    }
  }
  // creates a new exchange_rates query.
  pub fn exchange_rates(exchange_rates_params: ExchangeRatesParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_EXCHANGE_RATES,
      borrowed: None,
      exchange_rates: Some(exchange_rates_params),
      registered_tokens: None,
      leverage_parameters: None,
      borrowed_value: None,
    }
  }
  // creates a new registered_tokens query.
  pub fn registered_tokens(registered_tokens_params: RegisteredTokensParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_REGISTERED_TOKENS,
      borrowed: None,
      exchange_rates: None,
      registered_tokens: Some(registered_tokens_params),
      leverage_parameters: None,
      borrowed_value: None,
    }
  }
  // creates a new leverage_parameters query.
  pub fn leverage_parameters(
    leverage_parameters_params: LeverageParametersParams,
  ) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_LEVERAGE_PARAMS,
      borrowed: None,
      exchange_rates: None,
      registered_tokens: None,
      leverage_parameters: Some(leverage_parameters_params),
      borrowed_value: None,
    }
  }
  // creates a new borrowed_value query.
  pub fn borrowed_value(borrowed_params: BorrowedParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_BORROWED_VALUE,
      borrowed: None,
      exchange_rates: None,
      registered_tokens: None,
      leverage_parameters: None,
      borrowed_value: Some(borrowed_params),
    }
  }
}
