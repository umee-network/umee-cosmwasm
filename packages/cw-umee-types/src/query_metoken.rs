use cosmwasm_std::{Coin, Decimal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UmeeQueryMeToken {
  MetokenParameters(MetokenParametersParams),
  MetokenIndexes(MetokenIndexesParams),
  MetokenSwapfee(MetokenSwapfeeParams),
  MetokenRedeemfee(MetokenRedeemfeeParams),
  MetokenIndexbalances(MetokenIndexbalancesParams),
  MetokenIndexPrice(MetokenIndexPriceParams),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MetokenParametersParams {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MetokenParametersResponse {
  pub params: MetokenParameters,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MetokenParameters {
  pub rebalancing_frequency: i64,
  pub claiming_frequency: i64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MetokenIndexesParams {
  pub metoken_denom: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MetokenIndexesResponse {
  pub registry: Vec<Index>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Index {
  pub denom: String,
  pub max_supply: i64,
  pub exponent: u32,
  pub fee: Fee,
  pub accepted_assets: Vec<AcceptedAsset>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Fee {
  pub min_fee: Decimal,
  pub balanced_fee: Decimal,
  pub max_fee: Decimal,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AcceptedAsset {
  pub denom: String,
  pub reserve_portion: Decimal,
  pub target_allocation: Decimal,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MetokenSwapfeeParams {
  pub metoken_denom: String,
  pub asset: Coin,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MetokenSwapfeeResponse {
  pub asset: Coin,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MetokenRedeemfeeParams {
  pub metoken: Coin,
  pub asset_denom: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MetokenRedeemfeeResponse {
  pub asset: Coin,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MetokenIndexbalancesParams {
  pub metoken_denom: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MetokenIndexbalancesResponse {
  pub index_balances: Vec<IndexBalances>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct IndexBalances {
  pub metoken_supply: Coin,
  pub asset_balances: Vec<AssetBalance>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AssetBalance {
  pub denom: String,
  pub leveraged: Decimal,
  pub reserved: Decimal,
  pub fees: Decimal,
  pub interest: Decimal,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MetokenIndexPriceParams {
  pub metoken_denom: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MetokenIndexPriceResponse {
  pub price: Vec<Price>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Price {
  pub denom: String,
  pub price: Decimal,
  pub exponent: u32,
}
