use cosmwasm_std::Decimal256;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Params defines the parameters for the oracle module.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OracleParameters {
  vote_period: u64,
  vote_threshold: Decimal256,
  reward_band: Decimal256,
  reward_distribution_window: u64,
  accept_list: Vec<Denom>,
  slash_fraction: Decimal256,
  slash_window: u64,
  min_valid_per_window: Decimal256,
  stamp_period: u64,
  prune_period: u64,
  median_period: u64,
  historic_accept_list: Vec<Denom>,
}

// Denom object to hold configurations of each denom.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Denom {
  base_denom: String,
  symbol_denom: String,
  exponent: u32,
}
