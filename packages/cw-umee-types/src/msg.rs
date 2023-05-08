use crate::{
  error::ContractError,
  msg_leverage::{
    BorrowParams, CollateralizeParams, DecollateralizeParams, LiquidateParams, MsgMaxBorrowParams,
    MsgMaxWithdrawParams, MsgTypes, RepayParams, SupplyCollateralParams, SupplyParams,
    UmeeMsgLeverage, WithdrawParams,
  },
};
use cosmwasm_std::{CosmosMsg, CustomMsg, Response};
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
  assigned_msg: MsgTypes,
  supply: Option<SupplyParams>,
  withdraw: Option<WithdrawParams>,
  max_withdraw: Option<MsgMaxWithdrawParams>,
  collateralize: Option<CollateralizeParams>,
  decollateralize: Option<DecollateralizeParams>,
  borrow: Option<BorrowParams>,
  max_borrow: Option<MsgMaxBorrowParams>,
  repay: Option<RepayParams>,
  liquidate: Option<LiquidateParams>,
  supply_collateral: Option<SupplyCollateralParams>,
}

fn default_struct_umee_msg(m: MsgTypes) -> StructUmeeMsg {
  StructUmeeMsg {
    assigned_msg: m,
    supply: None,
    withdraw: None,
    collateralize: None,
    decollateralize: None,
    borrow: None,
    max_borrow: None,
    repay: None,
    liquidate: None,
    max_withdraw: None,
    supply_collateral: None,
  }
}

// msg_chain sends any message in the chain native modules
pub fn msg_chain(umee_msg: StructUmeeMsg) -> Result<Response<StructUmeeMsg>, ContractError> {
  if !umee_msg.valid() {
    return Err(ContractError::CustomError {
      val: String::from("invalid umee msg"),
    });
  }

  let res = Response::new()
    .add_attribute("method", umee_msg.assigned_str())
    .add_message(umee_msg);

  Ok(res)
}

// Defines all the implementation related to the StructUmeeMsg
// like creating new messages structs, it is needed because
// the fields inside the struct are private, to avoid missmatching
// the msg property with the assigned_msg field
impl StructUmeeMsg {
  // valid returns true if is valid
  pub fn valid(&self) -> bool {
    return self.assigned_str() != String::from("unrecognized_msg");
  }

  pub fn assigned_str(&self) -> String {
    match self.assigned_msg {
      MsgTypes::AssignedMsgSupply => String::from("supply"),
      MsgTypes::AssignedMsgWithdraw => String::from("withdraw"),
      MsgTypes::AssignedMsgMaxWithdraw => String::from("max_withdraw"),
      MsgTypes::AssignedMsgCollateralize => String::from("collateralize"),
      MsgTypes::AssignedMsgDecollateralize => String::from("decollateralize"),
      MsgTypes::AssignedMsgBorrow => String::from("borrow"),
      MsgTypes::AssignedMsgMaxBorrow => String::from("max_borrow"),
      MsgTypes::AssignedMsgRepay => String::from("repay"),
      MsgTypes::AssignedMsgLiquidate => String::from("liquidate"),
      MsgTypes::AssignedMsgSupplyCollateralize => String::from("supply_collateral"),
    }
  }
  // creates a new lend message.
  pub fn supply(supply_params: SupplyParams) -> Result<Response<StructUmeeMsg>, ContractError> {
    let mut m = default_struct_umee_msg(MsgTypes::AssignedMsgSupply);
    m.supply = Some(supply_params);
    return msg_chain(m);
  }

  // creates a new withdraw message.
  pub fn withdraw(
    withdraw_params: WithdrawParams,
  ) -> Result<Response<StructUmeeMsg>, ContractError> {
    let mut m = default_struct_umee_msg(MsgTypes::AssignedMsgWithdraw);
    m.withdraw = Some(withdraw_params);
    return msg_chain(m);
  }
  // creates a new maximum withdraw message.
  pub fn max_withdraw(
    msg_max_withdraw_params: MsgMaxWithdrawParams,
  ) -> Result<Response<StructUmeeMsg>, ContractError> {
    let mut m = default_struct_umee_msg(MsgTypes::AssignedMsgMaxWithdraw);
    m.max_withdraw = Some(msg_max_withdraw_params);
    return msg_chain(m);
  }
  // creates a new collateralize message.
  pub fn collateralize(
    collateralize_params: CollateralizeParams,
  ) -> Result<Response<StructUmeeMsg>, ContractError> {
    let mut m = default_struct_umee_msg(MsgTypes::AssignedMsgCollateralize);
    m.collateralize = Some(collateralize_params);
    return msg_chain(m);
  }
  // creates a new decollateralize message.
  pub fn decollateralize(
    decollateralize_params: DecollateralizeParams,
  ) -> Result<Response<StructUmeeMsg>, ContractError> {
    let mut m = default_struct_umee_msg(MsgTypes::AssignedMsgDecollateralize);
    m.decollateralize = Some(decollateralize_params);
    return msg_chain(m);
  }
  // creates a new borrow message.
  pub fn borrow(borrow_params: BorrowParams) -> Result<Response<StructUmeeMsg>, ContractError> {
    let mut m = default_struct_umee_msg(MsgTypes::AssignedMsgBorrow);
    m.borrow = Some(borrow_params);
    return msg_chain(m);
  }
  // creates a new max borrow message.
  pub fn max_borrow(
    max_borrow_params: MsgMaxBorrowParams,
  ) -> Result<Response<StructUmeeMsg>, ContractError> {
    let mut m = default_struct_umee_msg(MsgTypes::AssignedMsgMaxBorrow);
    m.max_borrow = Some(max_borrow_params);
    return msg_chain(m);
  }
  // creates a new repay message.
  pub fn repay(repay_params: RepayParams) -> Result<Response<StructUmeeMsg>, ContractError> {
    let mut m = default_struct_umee_msg(MsgTypes::AssignedMsgRepay);
    m.repay = Some(repay_params);
    return msg_chain(m);
  }
  // creates a new liquidate message.
  pub fn liquidate(
    liquidate_params: LiquidateParams,
  ) -> Result<Response<StructUmeeMsg>, ContractError> {
    let mut m = default_struct_umee_msg(MsgTypes::AssignedMsgLiquidate);
    m.liquidate = Some(liquidate_params);
    return msg_chain(m);
  }

  // creates a new supply collateralize message.
  pub fn supply_collateral(
    supply_collateral_params: SupplyCollateralParams,
  ) -> Result<Response<StructUmeeMsg>, ContractError> {
    let mut m = default_struct_umee_msg(MsgTypes::AssignedMsgSupplyCollateralize);
    m.supply_collateral = Some(supply_collateral_params);
    return msg_chain(m);
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
