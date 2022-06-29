use crate::leverage_parameters::LeverageParameters;
use crate::token::Token;
use cosmwasm_std::{Addr, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// All the queries must have an assigned query.
pub const ASSIGNED_QUERY_BORROWED: u16 = 1;
pub const ASSIGNED_QUERY_REGISTERED_TOKENS: u16 = 3;
pub const ASSIGNED_QUERY_LEVERAGE_PARAMS: u16 = 4;

// UmeeQueryLeverage defines all the available queries
// for the umee leverage native module.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UmeeQueryLeverage {
  // Borrowed returns an vec<sdk.Coin> representing how much of a given denom a
  // borrower currently owes. Expect to returns BorrowedResponse.
  Borrowed(BorrowedParams),
  // RegisteredTokens returns all the registered tokens from the x/leverage
  // module's KVStore. Expect to returns RegisteredTokensResponse.
  RegisteredTokens(RegisteredTokensParams),
  // LeverageParameters returns all the parameters from the x/leverage.
  // Expect to returns LeverageParametersResponse.
  LeverageParameters(LeverageParametersParams),
}

// BorrowedParams params to query Borrowed.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BorrowedParams {
  pub address: Addr,
  pub denom: Option<String>,
}

// BorrowedResponse response struct of Borrowed query.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BorrowedResponse {
  pub borrowed: Vec<Coin>,
}

// RegisteredTokensParams params to query RegisteredTokens.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct RegisteredTokensParams {}

// RegisteredTokensResponse response struct of RegisteredTokens query.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct RegisteredTokensResponse {
  pub registry: Vec<Token>,
}

// LeverageParametersParams params to query LeverageParameters.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct LeverageParametersParams {}

// LeverageParamsResponse response struct of LeverageParameters query.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct LeverageParametersResponse {
  pub params: LeverageParameters,
}
