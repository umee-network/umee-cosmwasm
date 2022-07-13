use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// AggregateExchangeRatePrevote struct for aggregate prevoting on the
// ExchangeRateVote. The purpose of aggregate prevote is to hide vote
// exchange rates with hash which is formatted as hex string in
// SHA256("{salt}:{exchange rate}{denom},...,{exchange rate}{denom}:{voter}")
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AggregateExchangeRatePrevote {
  hash: String,
  voter: String,
  submit_block: u64,
}
