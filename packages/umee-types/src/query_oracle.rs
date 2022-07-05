use cosmwasm_std::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// All the queries must have an assigned query
pub const ASSIGNED_QUERY_EXCHANGE_RATES: u16 = 2;

// UmeeQueryOracle defines  all the available queries
// for the umee Oracle native module
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UmeeQueryOracle {
  // ExchangeRates returns an sdk.Dec representing the exchange rate
  // of an denom. Expect to returns ExchangeRatesResponse.
  ExchangeRates(ExchangeRatesParams),
}

// ExchangeRatesParams params to query ExchangeRates
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ExchangeRatesParams {
  pub denom: String,
}

// ExchangeRatesResponse response struct of ExchangeRates query
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ExchangeRatesResponse {
  pub exchange_rates: Vec<DecCoin>,
}

// DecCoin defines a token with a denomination and a decimal amount.
//
// NOTE: The amount field is an Dec which implements the custom method
// signatures required by gogoproto.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct DecCoin {
  pub denom: String,
  pub amount: Decimal,
}
