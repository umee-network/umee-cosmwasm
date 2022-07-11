use crate::query_leverage::{
  AvailableBorrowParams, BorrowAPYParams, BorrowedParams, BorrowedValueParams,
  LeverageParametersParams, RegisteredTokensParams, SuppliedParams, SuppliedValueParams,
  SupplyAPYParams, UmeeQueryLeverage, ASSIGNED_QUERY_AVAILABLE_BORROW, ASSIGNED_QUERY_BORROWED,
  ASSIGNED_QUERY_BORROWED_VALUE, ASSIGNED_QUERY_BORROW_APY, ASSIGNED_QUERY_LEVERAGE_PARAMS,
  ASSIGNED_QUERY_REGISTERED_TOKENS, ASSIGNED_QUERY_SUPPLIED, ASSIGNED_QUERY_SUPPLIED_VALUE,
  ASSIGNED_QUERY_SUPPLY_APY,
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
  borrowed_value: Option<BorrowedValueParams>,
  supplied: Option<SuppliedParams>,
  supplied_value: Option<SuppliedValueParams>,
  available_borrow: Option<AvailableBorrowParams>,
  borrow_apy: Option<BorrowAPYParams>,
  supply_apy: Option<SupplyAPYParams>,
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
      supplied: None,
      supplied_value: None,
      available_borrow: None,
      borrow_apy: None,
      supply_apy: None,
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
      supplied: None,
      supplied_value: None,
      available_borrow: None,
      borrow_apy: None,
      supply_apy: None,
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
      supplied: None,
      supplied_value: None,
      available_borrow: None,
      borrow_apy: None,
      supply_apy: None,
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
      supplied: None,
      supplied_value: None,
      available_borrow: None,
      borrow_apy: None,
      supply_apy: None,
    }
  }
  // creates a new borrowed_value query.
  pub fn borrowed_value(borrowed_value_params: BorrowedValueParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_BORROWED_VALUE,
      borrowed: None,
      exchange_rates: None,
      registered_tokens: None,
      leverage_parameters: None,
      borrowed_value: Some(borrowed_value_params),
      supplied: None,
      supplied_value: None,
      available_borrow: None,
      borrow_apy: None,
      supply_apy: None,
    }
  }
  // creates a new supplied query.
  pub fn supplied(supplied_params: SuppliedParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_SUPPLIED,
      borrowed: None,
      exchange_rates: None,
      registered_tokens: None,
      leverage_parameters: None,
      borrowed_value: None,
      supplied: Some(supplied_params),
      available_borrow: None,
      supplied_value: None,
      borrow_apy: None,
      supply_apy: None,
    }
  }
  // creates a new supplied value query.
  pub fn supplied_value(supplied_value_params: SuppliedValueParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_SUPPLIED_VALUE,
      borrowed: None,
      exchange_rates: None,
      registered_tokens: None,
      leverage_parameters: None,
      borrowed_value: None,
      supplied: None,
      supplied_value: Some(supplied_value_params),
      available_borrow: None,
      borrow_apy: None,
      supply_apy: None,
    }
  }
  // creates a new available borrow query.
  pub fn available_borrow(available_borrow_params: AvailableBorrowParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_AVAILABLE_BORROW,
      borrowed: None,
      exchange_rates: None,
      registered_tokens: None,
      leverage_parameters: None,
      borrowed_value: None,
      supplied: None,
      supplied_value: None,
      available_borrow: Some(available_borrow_params),
      borrow_apy: None,
      supply_apy: None,
    }
  }
  // creates a new borrow apy query.
  pub fn borrow_apy(borrow_apy_params: BorrowAPYParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_BORROW_APY,
      borrowed: None,
      exchange_rates: None,
      registered_tokens: None,
      leverage_parameters: None,
      borrowed_value: None,
      supplied: None,
      supplied_value: None,
      available_borrow: None,
      borrow_apy: Some(borrow_apy_params),
      supply_apy: None,
    }
  }
  // creates a new supply apy query.
  pub fn supply_apy(supply_apy_params: SupplyAPYParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_SUPPLY_APY,
      borrowed: None,
      exchange_rates: None,
      registered_tokens: None,
      leverage_parameters: None,
      borrowed_value: None,
      supplied: None,
      supplied_value: None,
      available_borrow: None,
      borrow_apy: None,
      supply_apy: Some(supply_apy_params),
    }
  }
}
