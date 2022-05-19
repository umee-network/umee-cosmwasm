use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Binary,Addr, QueryRequest};
use umee_types::{ UmeeQuery, StructUmeeQuery };

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
  ChangeOwner { new_owner: Addr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  // GetOwner returns the current owner of the contract
  GetOwner {},
  // make requests directly to the blockchain using the struct
  Chain{
    request: QueryRequest<StructUmeeQuery>,
  },
  // wraps to use the enums
  Umee(UmeeQuery),
  // it can also call an specific enum directly
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnerResponse {
  pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ChainResponse {
  pub data: Binary,
}
