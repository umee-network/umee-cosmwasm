use cosmwasm_std::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// All the queries must have an assigned query
pub const ASSIGNED_QUERY_GET_EXCHANGE_RATE_BASE: u16 = 2;

// UmeeQueryOracle defines  all the available queries
// for the umee Oracle native module
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UmeeQueryOracle {
  // GetExchangeRateBase returns an sdk.Dec representing the exchange rate
  // of an denom. Expect to returns ExchangeRateBaseResponse.
  GetExchangeRateBase(ExchangeRateBaseParams),
}

// ExchangeRateBaseParams params to query GetExchangeRateBase
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ExchangeRateBaseParams {
  pub denom: String,
}

// ExchangeRateBaseResponse response struct of GetExchangeRateBase query
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ExchangeRateBaseResponse {
  pub exchange_rate_base: Decimal,
}
