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
  Umee(UmeeQuery),
  // make requests directly to the blockchain
  Chain {
    request: QueryRequest<UmeeQuery>,
  },
  ChainStruct{
    request: QueryRequest<StructUmeeQuery>,
  },
  // GetOwner returns the current owner of the contract
  GetOwner {},
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
