use cosmwasm_std::{Addr, QueryRequest};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use umee_types::{
  BorrowedParams, ExchangeRatesParams, LeverageParametersParams, RegisteredTokensParams,
  StructUmeeMsg, StructUmeeQuery, UmeeMsg, UmeeQuery,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
  // updates the state owner
  ChangeOwner { new_owner: Addr },
  Chain(StructUmeeMsg),
  Umee(UmeeMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  // GetOwner returns the current owner of the contract
  GetOwner {},
  // make requests directly to the blockchain using the struct
  Chain(QueryRequest<StructUmeeQuery>),
  // wraps to use the enums
  Umee(UmeeQuery),
  // it can also call an specific enum directly
  Borrowed(BorrowedParams),
  ExchangeRates(ExchangeRatesParams),
  RegisteredTokens(RegisteredTokensParams),
  LeverageParameters(LeverageParametersParams),
  BorrowedValue(BorrowedParams),
}

// returns the current contract owner
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnerResponse {
  pub owner: Addr,
}
