use cosmwasm_std::{Addr, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// All the messages must have an assigned msg.
pub const ASSIGNED_MSG_SUPPLY: u16 = 1;
pub const ASSIGNED_MSG_WITHDRAW: u16 = 2;
pub const ASSIGNED_MSG_COLLATERALIZE: u16 = 3;
pub const ASSIGNED_MSG_DECOLLATERALIZE: u16 = 4;
pub const ASSIGNED_MSG_BORROW: u16 = 5;
pub const ASSIGNED_MSG_REPAY: u16 = 6;
pub const ASSIGNED_MSG_LIQUIDATE: u16 = 7;

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
  // Decollateralize disable amount of an selected uTokens
  // as collateral.
  Decollateralize(DecollateralizeParams),
  // Borrow allows a user to borrow tokens from the module if they have sufficient collateral.
  Borrow(BorrowParams),
  // Repay allows a user to repay previously borrowed tokens and interest.
  Repay(RepayParams),
  // Liquidate allows a user to repay a different user's borrowed coins in exchange for some
  // of their collateral.
  Liquidate(LiquidateParams),
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

// DecollateralizeParams to disable selected uTokens as collateral.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct DecollateralizeParams {
  // Borrower is the account address removing collateral and the signer of the message.
  pub borrower: Addr,
  pub coin: Coin,
}

// BorrowParams to borrow a base asset type from the module.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BorrowParams {
  // Borrower is the account address taking a loan and the signer of the message.
  pub borrower: Addr,
  pub asset: Coin,
}

// RepayParams allows a user to repay previously borrowed tokens and interest.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct RepayParams {
  // Borrower is the account address repaying a loan and the signer of the message.
  pub borrower: Addr,
  pub asset: Coin,
}

// LiquidateParams to repaying a different user's borrowed coins
// to the capital facility in exchange for some of their collateral.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct LiquidateParams {
  // Liquidator is the account address performing a liquidation and the signer
  // of the message.
  pub liquidator: Addr,
  pub borrower: Addr,
  pub repayment: Coin,
  pub reward: Coin,
}
