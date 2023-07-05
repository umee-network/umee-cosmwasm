use cosmwasm_std::{Coin, Decimal, Decimal256};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UmeeQueryIncentive {
  IncentiveParameters(IncentiveParametersParams),
  TotalBonded(TotalBondedParams),
  TotalUnbonding(TotalUnbondingParams),
  AccountBonds(AccountBondsParams),
  PendingRewards(PendingRewardsParams),
  CompletedIncentivePrograms(CompletedIncentiveProgramsParams),
  OngoingIncentivePrograms(OngoingIncentiveProgramsParams),
  UpcomingIncentivePrograms(UpcomingIncentiveProgramsParams),
  IncentiveProgram(IncentiveProgramParams),
  CurrentRates(CurrentRatesParams),
  ActualRates(ActualRatesParams),
  LastRewardTime(LastRewardTimeParams),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct IncentiveParametersParams {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct IncentiveParametersResponse {
  pub params: IncentiveParameters,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct IncentiveParameters {
  pub max_unbondings: u32,
  pub unbonding_duration: i64,
  pub emergency_unbond_fee: Decimal256,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct TotalBondedParams {
  pub denom: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct TotalBondedResponse {
  pub bonded: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct TotalUnbondingParams {
  pub denom: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct TotalUnbondingResponse {
  pub unbonding: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AccountBondsParams {
  pub address: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AccountBondsResponse {
  pub bonded: Vec<Coin>,
  pub unbonding: Vec<Coin>,
  pub unbondings: Vec<Unbonding>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Unbonding {
  pub start: i64,
  pub end: i64,
  pub u_token: Coin,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PendingRewardsParams {
  pub address: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PendingRewardsResponse {
  pub rewards: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CompletedIncentiveProgramsParams {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CompletedIncentiveProgramsResponse {
  pub programs: Vec<IncentiveProgram>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OngoingIncentiveProgramsParams {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OngoingIncentiveProgramsResponse {
  pub programs: Vec<IncentiveProgram>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct UpcomingIncentiveProgramsParams {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct UpcomingIncentiveProgramsResponse {
  pub programs: Vec<IncentiveProgram>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct IncentiveProgramParams {
  pub id: u32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct IncentiveProgramResponse {
  pub program: IncentiveProgram,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CurrentRatesParams {
  pub u_token: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CurrentRatesResponse {
  pub reference_bond: Coin,
  pub rewards: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ActualRatesParams {
  pub u_token: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ActualRatesResponse {
  pub APY: Decimal,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct LastRewardTimeParams {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct LastRewardTimeResponse {
  pub time: i64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct IncentiveProgram {
  pub ID: u32,
  pub start_time: i64,
  pub duration: i64,
  pub u_token: String,
  pub funded: bool,
  pub total_rewards: Coin,
  pub remaining_rewards: Coin,
}
