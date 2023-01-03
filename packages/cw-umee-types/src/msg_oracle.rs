use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// messages types of oracle.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum UmeeMsgOracleTypes {
  AssignedAggregateExchangeRatePrevote,
  AssignedAggregateExchangeRateVote,
  AssignedDelegateFeedConsent,
}

// All the messages must have an assigned msg.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum UmeeMsgOracle {
  // AggregateExchangeRatePrevote defines a method for submitting an aggregate
  // exchange rate prevote.
  AggregateExchangeRatePrevote(MsgAggregateExchangeRatePrevote),
  // AggregateExchangeRateVote defines a method for submitting an aggregate
  // exchange rate vote.
  AggregateExchangeRateVote(MsgAggregateExchangeRateVote),
  // DelegateFeedConsent defines a method for setting the feeder delegation.
  DelegateFeedConsent(MsgDelegateFeedConsent),
}

// MsgAggregateExchangeRatePrevote represents a message to submit an aggregate
// exchange rate prevote.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, PartialOrd, Debug)]
pub struct MsgAggregateExchangeRatePrevote {
  pub hash: String,
  pub feeder: String,
  pub validator: String,
}

// MsgAggregateExchangeRateVote represents a message to submit anaggregate
// exchange rate vote.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, PartialOrd, Debug)]
pub struct MsgAggregateExchangeRateVote {
  pub salt: String,
  pub exchange_rates: String,
  pub feeder: String,
  pub validator: String,
}

// MsgDelegateFeedConsent represents a message to delegate oracle voting rights
// to another address.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, PartialOrd, Debug)]
pub struct MsgDelegateFeedConsent {
  pub operator: String,
  pub delegate: String,
}
