use cosmwasm_std::{Addr, QueryRequest};
use cw_umee_types::{
  ExchangeRatesParams, LeverageParametersParams, RegisteredTokensParams, StructUmeeQuery,
  SupplyParams, UmeeMsg, UmeeQuery,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
  // updates the state owner
  ChangeOwner { new_owner: Addr },
  Umee(UmeeMsg),
  Supply(SupplyParams),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  // GetOwner returns the current owner of the contract
  GetOwner {},
  // make requests directly to the blockchain using the struct
  Chain(Box<QueryRequest<StructUmeeQuery>>),
  // wraps to use the enums
  Umee(Box<UmeeQuery>),
  // it can also call an specific enum directly
  ExchangeRates(ExchangeRatesParams),
  RegisteredTokens(RegisteredTokensParams),
  LeverageParameters(LeverageParametersParams),
}

// returns the current contract owner
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnerResponse {
  pub owner: Addr,
}
