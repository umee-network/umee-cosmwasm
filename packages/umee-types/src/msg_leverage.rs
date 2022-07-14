use cosmwasm_std::{Addr, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// All the messages must have an assigned msg.
pub const ASSIGNED_MSG_SUPPLY: u16 = 1;
pub const ASSIGNED_MSG_WITHDRAW: u16 = 2;
pub const ASSIGNED_MSG_COLLATERALIZE: u16 = 3;

// UmeeMsgLeverage defines all the available msgs
// for the umee leverage native module.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UmeeMsgLeverage {
  // Supply expect to supply coins to the capital facility.
  Supply(SupplyParams),
  // Withdraw expect to withdraw previously loaned coins from
  // the capital facility.
  Withdraw(WithdrawParams),
  // Collateralize enables selected uTokens as collateral,
  // which moves them to the module.
  Collateralize(CollateralizeParams),
}

// SupplyParams params to lending coins to the capital facility.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SupplyParams {
  // Supplier is the account address supplying assets and the signer of the message.
  pub supplier: Addr,
  pub amount: Coin,
}

// WithdrawParams params to withdraw coins from the capital facility.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct WithdrawParams {
  // Supplier is the account address withdrawing assets and the signer of the message.
  pub supplier: Addr,
  pub amount: Coin,
}

// CollateralizeParams to enable selected uTokens as collateral.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CollateralizeParams {
	// Borrower is the account address adding collateral and the signer of the message.
  pub borrower: Addr,
  pub coin: Coin,
}
