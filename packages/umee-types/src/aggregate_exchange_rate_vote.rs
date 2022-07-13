use cosmwasm_std::Decimal256;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// AggregateExchangeRateVote struct for voting on
// the exchange rates of USD denominated in various assets.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AggregateExchangeRateVote {
  exchange_rate_tuples: Vec<ExchangeRateTuple>,
  voter: String,
}

// ExchangeRateTuple struct to store interpreted
// exchange rates data to store.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ExchangeRateTuple {
  denom: String,
  exchange_rate: Decimal256,
}
