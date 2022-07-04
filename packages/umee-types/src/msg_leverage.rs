use cosmwasm_std::{Addr, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// All the messages must have an assigned msg.
pub const ASSIGNED_MSG_LEND: u16 = 1;
pub const ASSIGNED_MSG_WITHDRAW: u16 = 2;

// UmeeMsgLeverage defines all the available msgs
// for the umee leverage native module.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UmeeMsgLeverage {
  // LendAsset expect to lend coins to the capital facility.
  LendAsset(LendAssetParams),
  // WithdrawAsset expect to withdraw previously loaned coins from
  // the capital facility. (use the same params as LendAsset)
  WithdrawAsset(LendAssetParams),
}

// LendAssetParams params to lending coins to the capital facility.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct LendAssetParams {
  pub lender: Addr,
  pub amount: Coin,
}
