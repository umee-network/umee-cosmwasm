use cosmwasm_std::{Addr, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// All the messages must have an assigned msg.
pub const ASSIGNED_MSG_SUPPLY: u16 = 1;
pub const ASSIGNED_MSG_WITHDRAW: u16 = 2;

// UmeeMsgLeverage defines all the available msgs
// for the umee leverage native module.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UmeeMsgLeverage {
  // Supply expect to supply coins to the capital facility.
  Supply(SupplyParams),
  // Withdraw expect to withdraw previously loaned coins from
  // the capital facility. (use the same params as Supply)
  Withdraw(WithdrawParams),
}

// SupplyParams params to lending coins to the capital facility.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SupplyParams {
  // Supplier is the account address supplying assets and the signer of the message.
  pub supplier: Addr,
  pub amount: Coin,
}

// SupplyParams params to lending coins to the capital facility.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct WithdrawParams {
  // Supplier is the account address withdrawing assets and the signer of the message.
  pub supplier: Addr,
  pub amount: Coin,
}
