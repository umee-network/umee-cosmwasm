use crate::aggregate_exchange_rate_prevote::AggregateExchangeRatePrevote;
use crate::aggregate_exchange_rate_vote::AggregateExchangeRateVote;
use crate::oracle_parameters::OracleParameters;
use cosmwasm_std::{Addr, Decimal256};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// All the queries must have an assigned query
pub const ASSIGNED_QUERY_EXCHANGE_RATES: u16 = 2;
pub const ASSIGNED_QUERY_ACTIVE_EXCHANGE_RATES: u16 = 23;
pub const ASSIGNED_QUERY_FEEDER_DELEGATION: u16 = 24;
pub const ASSIGNED_QUERY_MISS_COUNTER: u16 = 25;
pub const ASSIGNED_QUERY_AGGREGATE_PREVOTE: u16 = 26;
pub const ASSIGNED_QUERY_AGGREGATE_PREVOTES: u16 = 27;
pub const ASSIGNED_QUERY_AGGREGATE_VOTE: u16 = 28;
pub const ASSIGNED_QUERY_AGGREGATE_VOTES: u16 = 29;
pub const ASSIGNED_QUERY_ORACLE_PARAMS: u16 = 30;

// UmeeQueryOracle defines  all the available queries
// for the umee Oracle native module
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UmeeQueryOracle {
  // ExchangeRates returns an sdk.Dec representing the exchange rate
  // of an denom. Expect to returns ExchangeRatesResponse.
  ExchangeRates(ExchangeRatesParams),
  // ActiveExchangeRates returns all active denoms.
  // Expect to returns ActiveExchangeRatesResponse.
  ActiveExchangeRates(ActiveExchangeRatesParams),
  // FeederDelegation returns feeder delegation of a validator.
  // Expect to returns FeederDelegationResponse.
  FeederDelegation(FeederDelegationParams),
  // MissCounter returns oracle miss counter of a validator.
  // Expect to returns MissCounterResponse.
  MissCounter(MissCounterParams),
  // AggregatePrevote returns an aggregate prevote of a validator.
  // Expect to returns AggregatePrevoteResponse.
  AggregatePrevote(AggregatePrevoteParams),
  // AggregatePrevotes returns an aggregate prevotes of all validators.
  // Expect to returns AggregatePrevotesResponse.
  AggregatePrevotes(AggregatePrevotesParams),
  // AggregateVote returns an aggregate vote of a validator.
  // Expect to returns AggregateVoteResponse.
  AggregateVote(AggregateVoteParams),
  // AggregateVotes returns an aggregate vote of all validators.
  // Expect to returns AggregateVotesResponse.
  AggregateVotes(AggregateVotesParams),
  // OracleParameters returns all oracle module parameters.
  // Expect to returns OracleParametersParams.
  OracleParameters(OracleParametersParams),
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
  pub amount: Decimal256,
}

// ActiveExchangeRatesParams params to query ActiveExchangeRates.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ActiveExchangeRatesParams {}

// ActiveExchangeRatesResponse response struct of ActiveExchangeRates.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ActiveExchangeRatesResponse {
  pub active_rates: Vec<String>,
}

// FeederDelegationParams params to query FeederDelegation.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct FeederDelegationParams {
  validator_addr: Addr,
}

// FeederDelegationResponse response struct of FeederDelegation.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct FeederDelegationResponse {
  pub feeder_addr: String,
}

// MissCounterParams params to query MissCounter.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MissCounterParams {
  validator_addr: Addr,
}

// MissCounterResponse response struct of MissCounter.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MissCounterResponse {
  pub miss_counter: u64,
}

// AggregatePrevoteParams params to query AggregatePrevote.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AggregatePrevoteParams {
  validator_addr: Addr,
}

// AggregatePrevoteResponse response struct of AggregatePrevote.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AggregatePrevoteResponse {
  pub aggregate_prevote: AggregateExchangeRatePrevote,
}

// AggregatePrevotesParams params to query AggregatePrevotes.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AggregatePrevotesParams {}

// AggregatePrevotesResponse response struct of AggregatePrevotes.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AggregatePrevotesResponse {
  pub aggregate_prevotes: Vec<AggregateExchangeRatePrevote>,
}

// AggregateVoteParams params to query AggregateVote.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AggregateVoteParams {
  validator_addr: Addr,
}

// AggregateVoteResponse response struct of AggregateVote.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AggregateVoteResponse {
  pub aggregate_vote: AggregateExchangeRateVote,
}

// AggregateVotesParams params to query AggregateVotes.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AggregateVotesParams {}

// AggregateVotesResponse response struct of AggregateVotes.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AggregateVotesResponse {
  pub aggregate_votes: Vec<AggregateExchangeRateVote>,
}

// OracleParametersParams params to query OracleParameters.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OracleParametersParams {}

// OracleParametersResponse response struct of OracleParameters.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OracleParametersResponse {
  pub params: OracleParameters,
}
