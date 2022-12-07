use crate::msg_leverage::{
  BorrowParams, CollateralizeParams, DecollateralizeParams, LiquidateParams, RepayParams,
  SupplyParams, UmeeMsgLeverage, WithdrawParams, ASSIGNED_MSG_BORROW, ASSIGNED_MSG_COLLATERALIZE,
  ASSIGNED_MSG_DECOLLATERALIZE, ASSIGNED_MSG_LIQUIDATE, ASSIGNED_MSG_REPAY, ASSIGNED_MSG_SUPPLY,
  ASSIGNED_MSG_WITHDRAW, MsgMaxWithDrawParams, SupplyCollateralParams, ASSIGNED_MSG_MAX_WITHDRAW, ASSIGNED_MSG_SUPPLY_COLLATERALIZE,
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
  max_withdraw: Option<MsgMaxWithDrawParams>,
  collateralize: Option<CollateralizeParams>,
  decollateralize: Option<DecollateralizeParams>,
  borrow: Option<BorrowParams>,
  repay: Option<RepayParams>,
  liquidate: Option<LiquidateParams>,
  supply_collateralize: Option<SupplyCollateralParams>,
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
      ASSIGNED_MSG_LIQUIDATE => String::from("liquidate"),
      ASSIGNED_MSG_MAX_WITHDRAW => String::from("max_withdraw"),
      ASSIGNED_MSG_SUPPLY_COLLATERALIZE => String::from("supply_collateralize"),
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
      liquidate: None,
      max_withdraw:None,
      supply_collateralize:None,
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
      liquidate: None,
      max_withdraw:None,
      supply_collateralize:None,
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
      liquidate: None,
      max_withdraw:None,
      supply_collateralize:None,
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
      liquidate: None,
      max_withdraw:None,
      supply_collateralize:None,
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
      liquidate: None,
      max_withdraw:None,
      supply_collateralize:None,
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
      liquidate: None,
      max_withdraw:None,
      supply_collateralize:None,
    }
  }
  // creates a new liquidate message.
  pub fn liquidate(liquidate_params: LiquidateParams) -> StructUmeeMsg {
    StructUmeeMsg {
      assigned_msg: ASSIGNED_MSG_LIQUIDATE,
      supply: None,
      withdraw: None,
      collateralize: None,
      decollateralize: None,
      borrow: None,
      repay: None,
      liquidate: Some(liquidate_params),
      max_withdraw:None,
      supply_collateralize:None,
    }
  }
   // creates a new liquidate message.
   pub fn max_withdraw(msg_max_withdraw_params: MsgMaxWithDrawParams) -> StructUmeeMsg {
    StructUmeeMsg {
      assigned_msg: ASSIGNED_MSG_MAX_WITHDRAW,
      supply: None,
      withdraw: None,
      collateralize: None,
      decollateralize: None,
      borrow: None,
      repay: None,
      liquidate: None,
      max_withdraw:Some(msg_max_withdraw_params),
      supply_collateralize:None,
    }
  }

     // creates a new liquidate message.
   pub fn supply_collateralize(supply_collateral_params: SupplyCollateralParams) -> StructUmeeMsg {
    StructUmeeMsg {
      assigned_msg: ASSIGNED_MSG_SUPPLY_COLLATERALIZE,
      supply: None,
      withdraw: None,
      collateralize: None,
      decollateralize: None,
      borrow: None,
      repay: None,
      liquidate:None, 
      max_withdraw:None,
      supply_collateralize:Some(supply_collateral_params),
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
