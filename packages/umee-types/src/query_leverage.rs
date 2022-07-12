use crate::leverage_parameters::LeverageParameters;
use crate::token::Token;
use cosmwasm_std::{Addr, Coin, Decimal256};
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
pub const ASSIGNED_QUERY_MARKET_SIZE: u16 = 11;
pub const ASSIGNED_QUERY_TOKEN_MARKET_SIZE: u16 = 12;
pub const ASSIGNED_QUERY_RESERVE_AMOUNT: u16 = 13;
pub const ASSIGNED_QUERY_COLLATERAL: u16 = 14;
pub const ASSIGNED_QUERY_COLLATERAL_VALUE: u16 = 15;
pub const ASSIGNED_QUERY_EXCHANGE_RATE: u16 = 16;

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
  // SuppliedValue returns the USD value representing the amount of tokens
  // by a user by denomination. If the denomination is not specified,
  // the total for each supplied token is returned.
  // Expect to returns SuppliedValueResponse.
  SuppliedValue(SuppliedValueParams),
  // AvailableBorrow returns the available amount to borrow of a specified denomination.
  // Expect to returns AvailableBorrowResponse.
  AvailableBorrow(AvailableBorrowParams),
  // BorrowAPY returns current borrow interest rate on a token denom.
  // Expect to returns BorrowAPYResponse.
  BorrowAPY(BorrowAPYParams),
  // SupplyAPY returns current borrow interest rate on a token denom.
  // Expect to returns SupplyAPYResponse.
  SupplyAPY(SupplyAPYParams),
  // MarketSize returns the Market Size in USD of a specified denomination,
  // which is the USD value of total tokens supplied by all users plus borrow
  // interest owed by all users.
  // Expect to returns MarketSizeResponse.
  MarketSize(MarketSizeParams),
  // TokenMarketSize returns the Market Size in base tokens of a specified
  // denomination, which is the total tokens supplied by all users plus borrow
  // interest owed by all users.
  // Expect to returns TokenMarketSizeResponse.
  TokenMarketSize(TokenMarketSizeParams),
  // ReserveAmount returns the amount reserved of a specified denomination.
  // If the token is not valid, the reserved amount is zero.
  // Expect to returns ReserveAmountResponse.
  ReserveAmount(ReserveAmountParams),
  // Collateral returns the collateral amount of a user by token denomination.
  // If the denomination is not specified, all of the user's collateral tokens
  // are returned. Expect to returns CollateralResponse.
  Collateral(CollateralParams),
  // CollateralValue returns the total USD value of a user's collateral, or
  // the USD value held as a given base asset's associated uToken denomination.
  // Expect to returns CollateralValueResponse.
  CollateralValue(CollateralValueParams),
  // ExchangeRate returns the uToken exchange rate of a given uToken denomination.
  // Expect to returns ExchangeRateResponse.
  ExchangeRate(ExchangeRateParams),
}

// BorrowedParams params to query Borrowed.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BorrowedParams {
  pub address: Addr,
  pub denom: String,
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
  pub denom: String,
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
  pub denom: String,
}

// SuppliedValueResponse response struct of SuppliedValue query.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SuppliedValueResponse {
  pub supplied_value: Decimal256,
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
  pub borrowed_value: Decimal256,
}

// AvailableBorrowParams params to query AvailableBorrow.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AvailableBorrowParams {
  pub denom: String,
}

// AvailableBorrowResponse response struct of AvailableBorrow.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AvailableBorrowResponse {
  pub amount: Decimal256,
}

// BorrowAPYParams params to query BorrowAPY.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BorrowAPYParams {
  pub denom: String,
}

// BorrowAPYResponse response struct of BorrowAPY.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BorrowAPYResponse {
  pub amount: Decimal256,
}

// SupplyAPYParams params to query SupplyAPY.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SupplyAPYParams {
  pub denom: String,
}

// SupplyAPYResponse response struct of SupplyAPY.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SupplyAPYResponse {
  pub apy: Decimal256,
}

// MarketSizeParams params to query MarketSize.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MarketSizeParams {
  pub denom: String,
}

// MarketSizeResponse response struct of MarketSize.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MarketSizeResponse {
  pub market_size_usd: Decimal256,
}

// TokenMarketSizeParams params to query TokenMarketSize.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct TokenMarketSizeParams {
  pub denom: String,
}

// TokenMarketSizeResponse response struct of TokenMarketSize.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct TokenMarketSizeResponse {
  pub market_size_usd: Decimal256,
}

// ReserveAmountParams params to query ReserveAmount.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ReserveAmountParams {
  pub denom: String,
}

// ReserveAmountResponse response struct of ReserveAmount.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ReserveAmountResponse {
  pub amount: Decimal256,
}

// CollateralParams params to query Collateral.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CollateralParams {
  pub address: Addr,
  pub denom: Option<String>,
}

// CollateralResponse response struct of Collateral.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CollateralResponse {
  pub collateral: Vec<Coin>,
}

// CollateralValueParams params to query CollateralValue.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CollateralValueParams {
  pub address: Addr,
  pub denom: Option<String>,
}

// CollateralValueResponse response struct of CollateralValue.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CollateralValueResponse {
  pub collateral_value: Decimal256,
}

// ExchangeRateParams params to query ExchangeRate.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ExchangeRateParams {
  pub denom: String,
}

// ExchangeRateResponse response struct of ExchangeRate.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ExchangeRateResponse {
  pub exchange_rate: Decimal256,
}
