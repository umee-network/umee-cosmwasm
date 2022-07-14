use crate::msg_leverage::{
  SupplyParams, UmeeMsgLeverage, WithdrawParams, ASSIGNED_MSG_SUPPLY, ASSIGNED_MSG_WITHDRAW,
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
  supply: Option<SupplyParams>,
  withdraw: Option<WithdrawParams>,
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
      ASSIGNED_MSG_SUPPLY => String::from("supply"),
      ASSIGNED_MSG_WITHDRAW => String::from("withdraw"),
      _ => String::from("unrecognized_msg"),
    }
  }
  // creates a new lend message.
  pub fn supply(supply_params: SupplyParams) -> StructUmeeMsg {
    StructUmeeMsg {
      assigned_msg: ASSIGNED_MSG_SUPPLY,
      supply: Some(supply_params),
      withdraw: None,
    }
  }
  // creates a new withdraw message.
  pub fn withdraw(withdraw_params: WithdrawParams) -> StructUmeeMsg {
    StructUmeeMsg {
      assigned_msg: ASSIGNED_MSG_WITHDRAW,
      supply: None,
      withdraw: Some(withdraw_params),
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
