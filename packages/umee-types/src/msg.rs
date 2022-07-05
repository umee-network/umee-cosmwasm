use crate::msg_leverage::{
  LendAssetParams, UmeeMsgLeverage, ASSIGNED_MSG_LEND, ASSIGNED_MSG_WITHDRAW,
};
// use crate::query_oracle::{
//   ExchangeRateBaseParams, UmeeMsgOracle, ASSIGNED_QUERY_EXCHANGE_RATES,
// };
use cosmwasm_std::{CosmosMsg, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Define the implementation necessary for cosmwasm "custom" msgs
impl CustomMsg for UmeeMsg {}
impl CustomMsg for StructUmeeMsg {}

// UmeeMsg combines all the native modules from umee as enum
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UmeeMsg {
  // Leverage wraps all the msg enums from the leverage module
  Leverage(UmeeMsgLeverage),
  // Oracle wraps all the query enums from the oracle module
  // Oracle(UmeeMsgOracle),
}

// StructUmeeMsg expected structure to send messages to the umee native modules.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StructUmeeMsg {
  assigned_msg: u16,
  lend_asset: Option<LendAssetParams>,
  withdraw_asset: Option<LendAssetParams>,
}

// Defines all the implementation related to the StructUmeeMsg
// like creating new messages structs, it is needed because
// the fields inside the struct are private, to avoid missmatching
// the msg property with the assigned_msg field
impl StructUmeeMsg {
  // valid returns true if is valid
  pub fn valid(&self) -> bool {
    return self.assigned_msg > 0;
  }

  pub fn assigned_str(&self) -> String {
    match self.assigned_msg {
      ASSIGNED_MSG_LEND => String::from("lend_asset"),
      ASSIGNED_MSG_WITHDRAW => String::from("withdraw_asset"),
      _ => String::from("unrecognized_msg"),
    }
  }
  // creates a new lend asset message.
  pub fn lend_asset(lend_asset_params: LendAssetParams) -> StructUmeeMsg {
    StructUmeeMsg {
      assigned_msg: ASSIGNED_MSG_LEND,
      lend_asset: Some(lend_asset_params),
      withdraw_asset: None,
    }
  }
  // creates a new withdraw asset message.
  pub fn withdraw_asset(withdraw_asset_params: LendAssetParams) -> StructUmeeMsg {
    StructUmeeMsg {
      assigned_msg: ASSIGNED_MSG_WITHDRAW,
      lend_asset: None,
      withdraw_asset: Some(withdraw_asset_params),
    }
  }
}

impl From<StructUmeeMsg> for CosmosMsg<StructUmeeMsg> {
  fn from(msg: StructUmeeMsg) -> Self {
    CosmosMsg::Custom(msg)
  }
}

impl From<UmeeMsg> for CosmosMsg<UmeeMsg> {
  fn from(msg: UmeeMsg) -> Self {
    CosmosMsg::Custom(msg)
  }
}
