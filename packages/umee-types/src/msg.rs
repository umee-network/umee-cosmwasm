use crate::msg_leverage::{
  BorrowParams, CollateralizeParams, DecollateralizeParams, RepayParams, SupplyParams,
  UmeeMsgLeverage, WithdrawParams, ASSIGNED_MSG_BORROW, ASSIGNED_MSG_COLLATERALIZE,
  ASSIGNED_MSG_DECOLLATERALIZE, ASSIGNED_MSG_REPAY, ASSIGNED_MSG_SUPPLY, ASSIGNED_MSG_WITHDRAW,
};
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
}

// StructUmeeMsg expected structure to send messages to the umee native modules.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StructUmeeMsg {
  assigned_msg: u16,
  supply: Option<SupplyParams>,
  withdraw: Option<WithdrawParams>,
  collateralize: Option<CollateralizeParams>,
  decollateralize: Option<DecollateralizeParams>,
  borrow: Option<BorrowParams>,
  repay: Option<RepayParams>,
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
      ASSIGNED_MSG_COLLATERALIZE => String::from("collateralize"),
      ASSIGNED_MSG_DECOLLATERALIZE => String::from("decollateralize"),
      ASSIGNED_MSG_BORROW => String::from("borrow"),
      ASSIGNED_MSG_REPAY => String::from("repay"),
      _ => String::from("unrecognized_msg"),
    }
  }
  // creates a new lend message.
  pub fn supply(supply_params: SupplyParams) -> StructUmeeMsg {
    StructUmeeMsg {
      assigned_msg: ASSIGNED_MSG_SUPPLY,
      supply: Some(supply_params),
      withdraw: None,
      collateralize: None,
      decollateralize: None,
      borrow: None,
      repay: None,
    }
  }
  // creates a new withdraw message.
  pub fn withdraw(withdraw_params: WithdrawParams) -> StructUmeeMsg {
    StructUmeeMsg {
      assigned_msg: ASSIGNED_MSG_WITHDRAW,
      supply: None,
      withdraw: Some(withdraw_params),
      collateralize: None,
      decollateralize: None,
      borrow: None,
      repay: None,
    }
  }
  // creates a new collateralize message.
  pub fn collateralize(collateralize_params: CollateralizeParams) -> StructUmeeMsg {
    StructUmeeMsg {
      assigned_msg: ASSIGNED_MSG_COLLATERALIZE,
      supply: None,
      withdraw: None,
      collateralize: Some(collateralize_params),
      decollateralize: None,
      borrow: None,
      repay: None,
    }
  }
  // creates a new decollateralize message.
  pub fn decollateralize(decollateralize_params: DecollateralizeParams) -> StructUmeeMsg {
    StructUmeeMsg {
      assigned_msg: ASSIGNED_MSG_DECOLLATERALIZE,
      supply: None,
      withdraw: None,
      collateralize: None,
      decollateralize: Some(decollateralize_params),
      borrow: None,
      repay: None,
    }
  }
  // creates a new borrow message.
  pub fn borrow(borrow_params: BorrowParams) -> StructUmeeMsg {
    StructUmeeMsg {
      assigned_msg: ASSIGNED_MSG_BORROW,
      supply: None,
      withdraw: None,
      collateralize: None,
      decollateralize: None,
      borrow: Some(borrow_params),
      repay: None,
    }
  }
  // creates a new repay message.
  pub fn repay(repay_params: RepayParams) -> StructUmeeMsg {
    StructUmeeMsg {
      assigned_msg: ASSIGNED_MSG_REPAY,
      supply: None,
      withdraw: None,
      collateralize: None,
      decollateralize: None,
      borrow: None,
      repay: Some(repay_params),
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
