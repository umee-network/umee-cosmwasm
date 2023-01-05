use crate::bad_debt::BadDebt;
use crate::leverage_parameters::LeverageParameters;
use crate::token::Token;
use cosmwasm_std::{Addr, Coin, Decimal256};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// All the queries must have an assigned query.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub enum LeverageQueries {
  // NOTE: First leverage query types then oracle module query types
  // If you add any new query, Please increase the OracleQueryTypes first enum value
  // Please don't change the order, If you change the order please update the respective query type value in Umee wasm
  // query enum values also
  AssignedQueryLeverageParams = 0,
  AssignedQueryRegisteredTokens,
  AssignedQueryMarketSummary,
  AssignedQueryAccountBalances,
  AssignedQueryAccountSummary,
  AssignedQueryLiquidationTargets,
  AssignedQueryBadDebts,
  AssignedQueryMaxWithdraw,
}
// UmeeQueryLeverage defines all the available queries
// for the umee leverage native module.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UmeeQueryLeverage {
  // LeverageParameters returns all the parameters from the x/leverage.
  // Expect to returns LeverageParametersResponse.
  LeverageParameters(LeverageParametersParams),
  // RegisteredTokens returns all the registered tokens from the x/leverage
  // module's KVStore. Expect to returns RegisteredTokensResponse.
  RegisteredTokens(RegisteredTokensParams),
  // MarketSummary returns base asset's current borrowing and supplying conditions.
  // Expect to returns MarketSummaryResponse.
  MarketSummary(MarketSummaryParams),
  // AccountBalances returns account's current supply, collateral, and borrow positions.
  // Expect to returns AccountBalancesResponse.
  AccountBalances(AccountBalancesParams),
  // AccountSummary returns USD values representing an account's total.
  // Expect to returns AccountSummaryResponse.
  AccountSummary(AccountSummaryParams),
  // LiquidationTargets returns the list of all borrower addresses eligible
  // for liquidation.
  // Expect to returns LiquidationTargetsResponse.
  LiquidationTargets(LiquidationTargetsParams),
  // BadDebts returns a list of borrow positions that have been marked for bad debt repayment.
  BadDebts(BadDebtsParams),
  // MaxWithdraw returns the maximum amount of a given token an address can withdraw.
  MaxWithdraw(MaxWithdrawParams),
}

// LeverageParametersParams params to query LeverageParameters.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct LeverageParametersParams {}

// LeverageParamsResponse response struct of LeverageParameters query.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct LeverageParametersResponse {
  pub params: LeverageParameters,
}

// RegisteredTokensParams params to query RegisteredTokens.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct RegisteredTokensParams {}

// RegisteredTokensResponse response struct of RegisteredTokens query.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct RegisteredTokensResponse {
  pub registry: Vec<Token>,
}

// MarketSummaryParams params to query MarketSummary.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MarketSummaryParams {
  pub denom: String,
}

// MarketSummary base asset's current borrowing and supplying conditions.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MarketSummaryResponse {
  symbol_denom: String,
  exponent: u32,
  oracle_price: Decimal256,
  utoken_exchange_rate: Decimal256,
  supply_apy: Decimal256,
  borrow_apy: Decimal256,
  supplied: Decimal256,
  reserved: Decimal256,
  collateral: Decimal256,
  borrowed: Decimal256,
  liquidity: Decimal256,
  maximum_borrow: Decimal256,
  maximum_collateral: Decimal256,
  minimum_liquidity: Decimal256,
  utoken_supply: Decimal256,
  available_borrow: Decimal256,
  available_withdraw: Decimal256,
  available_collateralize: Decimal256,
}

// AccountBalancesParams params to query AccountBalances.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AccountBalancesParams {
  pub address: Addr,
}

// AccountBalancesResponse response struct of AccountBalances query.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AccountBalancesResponse {
  pub supplied: Vec<Coin>,
  pub collateral: Vec<Coin>,
  pub borrowed: Vec<Coin>,
}

// AccountSummaryParams params to query AccountSummary.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AccountSummaryParams {
  pub address: Addr,
}

// AccountSummaryResponse response struct of AccountSummary query.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AccountSummaryResponse {
  pub supplied_value: Decimal256,
  pub collateral_value: Decimal256,
  pub borrowed_value: Decimal256,
  pub borrow_limit: Decimal256,
  pub liquidation_threshold: Decimal256,
}

// LiquidationTargetsParams params to query LiquidationTargets.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct LiquidationTargetsParams {}

// LiquidationTargetsResponse response struct of LiquidationTargets.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct LiquidationTargetsResponse {
  pub targets: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BadDebtsParams {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BadDebtsResponse {
  pub targets: Vec<BadDebt>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MaxWithdrawParams {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MaxWithdrawResponse {
  pub uTokens: Coin,
  pub tokens: Coin,
}
