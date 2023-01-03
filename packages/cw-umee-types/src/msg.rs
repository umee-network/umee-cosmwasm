use crate::{
  msg_leverage::{
    BorrowParams, CollateralizeParams, DecollateralizeParams, LiquidateParams,
    MsgMaxWithDrawParams, RepayParams, SupplyCollateralParams, SupplyParams, UmeeMsgLeverage,
    UmeeMsgLeverageTypes, WithdrawParams,
  },
  msg_oracle::{
    MsgAggregateExchangeRatePrevote, MsgAggregateExchangeRateVote, MsgDelegateFeedConsent,
    UmeeMsgOracle, UmeeMsgOracleTypes,
  },
};

use cosmwasm_std::{CosmosMsg, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Define the implementation necessary for cosmwasm "custom" msgs
impl CustomMsg for UmeeMsg {}
impl CustomMsg for StructUmeeMsg {}

// UmeeMsgTypes contains umee msg types as enum
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UmeeMsgTypes {
  // Leverage wraps all the msg enums from the leverage module
  Leverage(UmeeMsgLeverageTypes),
  // Oracle wraps all the msg enums from the oracle module
  Oracle(UmeeMsgOracleTypes),
}

// UmeeMsg combines all the native modules from umee as enum
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UmeeMsg {
  // Leverage wraps all the msg enums from the leverage module
  Leverage(UmeeMsgLeverage),
  // Oracle wraps all the msg enums from the oracle module
  Oracle(UmeeMsgOracle),
}

// StructUmeeMsg expected structure to send messages to the umee native modules.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StructUmeeMsg {
  assigned_msg: UmeeMsgTypes,
  // leverage messages
  supply: Option<SupplyParams>,
  withdraw: Option<WithdrawParams>,
  max_withdraw: Option<MsgMaxWithDrawParams>,
  collateralize: Option<CollateralizeParams>,
  decollateralize: Option<DecollateralizeParams>,
  borrow: Option<BorrowParams>,
  repay: Option<RepayParams>,
  liquidate: Option<LiquidateParams>,
  supply_collateralize: Option<SupplyCollateralParams>,
  // oracle messages
  aggregate_exchange_rate_prevote: Option<MsgAggregateExchangeRatePrevote>,
  aggregate_exchange_rate_vote: Option<MsgAggregateExchangeRateVote>,
  delegate_feed_consent: Option<MsgDelegateFeedConsent>,
}

fn default_struct_umee_msg(m: UmeeMsgTypes) -> StructUmeeMsg {
  StructUmeeMsg {
    assigned_msg: m,
    supply: None,
    withdraw: None,
    collateralize: None,
    decollateralize: None,
    borrow: None,
    repay: None,
    liquidate: None,
    max_withdraw: None,
    supply_collateralize: None,
    aggregate_exchange_rate_prevote: None,
    aggregate_exchange_rate_vote: None,
    delegate_feed_consent: None,
  }
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
      // Leverage
      UmeeMsgTypes::Leverage(UmeeMsgLeverageTypes::AssignedMsgSupply) => String::from("supply"),
      UmeeMsgTypes::Leverage(UmeeMsgLeverageTypes::AssignedMsgWithdraw) => String::from("withdraw"),
      UmeeMsgTypes::Leverage(UmeeMsgLeverageTypes::AssignedMsgCollateralize) => {
        String::from("collateralize")
      }
      UmeeMsgTypes::Leverage(UmeeMsgLeverageTypes::AssignedMsgDecollateralize) => {
        String::from("decollateralize")
      }
      UmeeMsgTypes::Leverage(UmeeMsgLeverageTypes::AssignedMsgBorrow) => String::from("borrow"),
      UmeeMsgTypes::Leverage(UmeeMsgLeverageTypes::AssignedMsgRepay) => String::from("repay"),
      UmeeMsgTypes::Leverage(UmeeMsgLeverageTypes::AssignedMsgLiquidate) => {
        String::from("liquidate")
      }
      UmeeMsgTypes::Leverage(UmeeMsgLeverageTypes::AssignedMsgMaxWithdraw) => {
        String::from("max_withdraw")
      }
      UmeeMsgTypes::Leverage(UmeeMsgLeverageTypes::AssignedMsgSupplyCollateralize) => {
        String::from("supply_collateralize")
      }
      // Oracle
      UmeeMsgTypes::Oracle(UmeeMsgOracleTypes::AssignedAggregateExchangeRatePrevote) => {
        String::from("aggregate_exchange_rate_prevote")
      }
      UmeeMsgTypes::Oracle(UmeeMsgOracleTypes::AssignedAggregateExchangeRateVote) => {
        String::from("aggregate_exchange_rate_vote")
      }
      UmeeMsgTypes::Oracle(UmeeMsgOracleTypes::AssignedDelegateFeedConsent) => {
        String::from("delegate_feed_consent")
      }
      _ => String::from("unrecognized_msg"),
    }
  }
  // creates a new lend message.
  pub fn supply(supply_params: SupplyParams) -> StructUmeeMsg {
    let mut m = default_struct_umee_msg(UmeeMsgTypes::Leverage(
      UmeeMsgLeverageTypes::AssignedMsgSupply,
    ));
    m.supply = Some(supply_params);
    return m;
  }
  // creates a new withdraw message.
  pub fn withdraw(withdraw_params: WithdrawParams) -> StructUmeeMsg {
    let mut m = default_struct_umee_msg(UmeeMsgTypes::Leverage(
      UmeeMsgLeverageTypes::AssignedMsgWithdraw,
    ));
    m.withdraw = Some(withdraw_params);
    return m;
  }
  // creates a new collateralize message.
  pub fn collateralize(collateralize_params: CollateralizeParams) -> StructUmeeMsg {
    let mut m = default_struct_umee_msg(UmeeMsgTypes::Leverage(
      UmeeMsgLeverageTypes::AssignedMsgCollateralize,
    ));
    m.collateralize = Some(collateralize_params);
    return m;
  }
  // creates a new decollateralize message.
  pub fn decollateralize(decollateralize_params: DecollateralizeParams) -> StructUmeeMsg {
    let mut m = default_struct_umee_msg(UmeeMsgTypes::Leverage(
      UmeeMsgLeverageTypes::AssignedMsgDecollateralize,
    ));
    m.decollateralize = Some(decollateralize_params);
    return m;
  }
  // creates a new borrow message.
  pub fn borrow(borrow_params: BorrowParams) -> StructUmeeMsg {
    let mut m = default_struct_umee_msg(UmeeMsgTypes::Leverage(
      UmeeMsgLeverageTypes::AssignedMsgBorrow,
    ));
    m.borrow = Some(borrow_params);
    return m;
  }
  // creates a new repay message.
  pub fn repay(repay_params: RepayParams) -> StructUmeeMsg {
    let mut m = default_struct_umee_msg(UmeeMsgTypes::Leverage(
      UmeeMsgLeverageTypes::AssignedMsgRepay,
    ));
    m.repay = Some(repay_params);
    return m;
  }
  // creates a new liquidate message.
  pub fn liquidate(liquidate_params: LiquidateParams) -> StructUmeeMsg {
    let mut m = default_struct_umee_msg(UmeeMsgTypes::Leverage(
      UmeeMsgLeverageTypes::AssignedMsgLiquidate,
    ));
    m.liquidate = Some(liquidate_params);
    return m;
  }
  // creates a new maximum withdraw message.
  pub fn max_withdraw(msg_max_withdraw_params: MsgMaxWithDrawParams) -> StructUmeeMsg {
    let mut m = default_struct_umee_msg(UmeeMsgTypes::Leverage(
      UmeeMsgLeverageTypes::AssignedMsgMaxWithdraw,
    ));
    m.max_withdraw = Some(msg_max_withdraw_params);
    return m;
  }
  // creates a new supply collateralize message.
  pub fn supply_collateralize(supply_collateral_params: SupplyCollateralParams) -> StructUmeeMsg {
    let mut m = default_struct_umee_msg(UmeeMsgTypes::Leverage(
      UmeeMsgLeverageTypes::AssignedMsgSupplyCollateralize,
    ));
    m.supply_collateralize = Some(supply_collateral_params);
    return m;
  }

  /*
    Oracle module messages
  */

  // creates a new  aggregate exchange rate prevote message.
  pub fn aggregate_exchange_rate_prevote(
    msg_aggregate_exchange_rate_prevote: MsgAggregateExchangeRatePrevote,
  ) -> StructUmeeMsg {
    let mut m = default_struct_umee_msg(UmeeMsgTypes::Oracle(
      UmeeMsgOracleTypes::AssignedAggregateExchangeRatePrevote,
    ));
    m.aggregate_exchange_rate_prevote = Some(msg_aggregate_exchange_rate_prevote);
    return m;
  }
  // creates a new aggregate exchange rate vote message.
  pub fn aggregate_exchange_rate_vote(
    msg_aggregate_exchange_rate_vote: MsgAggregateExchangeRateVote,
  ) -> StructUmeeMsg {
    let mut m = default_struct_umee_msg(UmeeMsgTypes::Oracle(
      UmeeMsgOracleTypes::AssignedAggregateExchangeRatePrevote,
    ));
    m.aggregate_exchange_rate_vote = Some(msg_aggregate_exchange_rate_vote);
    return m;
  }
  // creates a new delegate feed consent message.
  pub fn delegate_feed_consent(msg_delegate_feed_consent: MsgDelegateFeedConsent) -> StructUmeeMsg {
    let mut m = default_struct_umee_msg(UmeeMsgTypes::Oracle(
      UmeeMsgOracleTypes::AssignedDelegateFeedConsent,
    ));
    m.delegate_feed_consent = Some(msg_delegate_feed_consent);
    return m;
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
