use crate::leverage_parameters::LeverageParameters;
use crate::token::Token;
use cosmwasm_std::{Addr, Coin, Decimal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// All the queries must have an assigned query.
pub const ASSIGNED_QUERY_BORROWED: u16 = 1;
pub const ASSIGNED_QUERY_REGISTERED_TOKENS: u16 = 3;
pub const ASSIGNED_QUERY_LEVERAGE_PARAMS: u16 = 4;
pub const ASSIGNED_QUERY_BORROWED_VALUE: u16 = 5;
pub const ASSIGNED_QUERY_SUPPLIED: u16 = 6;
pub const ASSIGNED_QUERY_SUPPLIED_VALUE: u16 = 7;
pub const ASSIGNED_QUERY_AVAILABLE_BORROW: u16 = 8;
pub const ASSIGNED_QUERY_BORROW_APY: u16 = 9;
pub const ASSIGNED_QUERY_SUPPLY_APY: u16 = 10;

// UmeeQueryLeverage defines all the available queries
// for the umee leverage native module.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UmeeQueryLeverage {
  // Borrowed returns an vec<sdk.Coin> representing how much an
  // borrower currently owes. Expect to returns BorrowedResponse.
  Borrowed(BorrowedParams),
  // RegisteredTokens returns all the registered tokens from the x/leverage
  // module's KVStore. Expect to returns RegisteredTokensResponse.
  RegisteredTokens(RegisteredTokensParams),
  // LeverageParameters returns all the parameters from the x/leverage.
  // Expect to returns LeverageParametersResponse.
  LeverageParameters(LeverageParametersParams),
  // BorrowedValue returns an sdk.Dec representing how much in USD an
  // borrower currently owes. Expect to returns BorrowedValueResponse.
  BorrowedValue(BorrowedValueParams),
  // Supplied returns an slice of sdk.Dec representing the amount of tokens
  // by a user by denomination. If the denomination is not specified,
  // the total for each supplied token is returned.
  // Expect to returns SuppliedResponse.
  Supplied(SuppliedParams),
  // Supplied returns the USD value representing the amount of tokens
  // by a user by denomination. If the denomination is not specified,
  // the total for each supplied token is returned.
  // Expect to returns SuppliedValueResponse.
  SuppliedValue(SuppliedValueParams),
  // Supplied returns the available amount to borrow of a specified denomination.
  // Expect to returns AvailableBorrowResponse.
  AvailableBorrow(AvailableBorrowParams),
  // Supplied returns current borrow interest rate on a token denom.
  // Expect to returns BorrowAPYResponse.
  BorrowAPY(BorrowAPYParams),
  // Supplied returns current borrow interest rate on a token denom.
  // Expect to returns SupplyAPYResponse.
  SupplyAPY(SupplyAPYParams),
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

// SuppliedParams params to query Supplied.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SuppliedParams {
  pub address: Addr,
  pub denom: Option<String>,
}

// SuppliedResponse response struct of Supplied query.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SuppliedResponse {
  pub supplied: Vec<Coin>,
}

// SuppliedValueParams params to query SuppliedValue.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SuppliedValueParams {
  pub address: Addr,
  pub denom: Option<String>,
}

// SuppliedValueResponse response struct of SuppliedValue query.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SuppliedValueResponse {
  pub supplied_value: Decimal,
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

// BorrowedValueParams params to query BorrowedValue.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BorrowedValueParams {
  pub address: Addr,
  pub denom: Option<String>,
}

// BorrowedValueResponse response struct of Borrowed query in USD.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BorrowedValueResponse {
  pub borrowed_value: Decimal,
}

// AvailableBorrowParams params to query AvailableBorrow.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AvailableBorrowParams {
  pub denom: String,
}

// AvailableBorrowResponse response struct of AvailableBorrow.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AvailableBorrowResponse {
  pub amount: Decimal,
}

// BorrowAPYParams params to query BorrowAPY.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BorrowAPYParams {
  pub denom: String,
}

// BorrowAPYResponse response struct of BorrowAPY.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BorrowAPYResponse {
  pub amount: Decimal,
}

// SupplyAPYParams params to query SupplyAPY.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SupplyAPYParams {
  pub denom: String,
}

// SupplyAPYResponse response struct of SupplyAPY.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SupplyAPYResponse {
  pub apy: Decimal,
}
