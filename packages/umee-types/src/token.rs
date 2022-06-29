use cosmwasm_std::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Token defines a token, along with its capital metadata, in the Umee capital
// facility that can be loaned and borrowed.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Token {
  // The base_denom defines the denomination of the underlying base token.
  base_denom: Option<String>,
  // The reserve factor defines what portion of accrued interest of the asset
  // type goes to reserves.
  reserve_factor: Decimal,
  // The collateral_weight defines what amount of the total value of the asset
  // can contribute to a users borrowing power. If the collateral_weight is
  // zero, using this asset as collateral against borrowing will be disabled.
  collateral_weight: Decimal,
  // The liquidation_threshold defines what amount of the total value of the
  // asset can contribute to a user's liquidation threshold (above which they
  // become eligible for liquidation).
  liquidation_threshold: Decimal,
  // The base_borrow_rate defines the base interest rate for borrowing this
  // asset.
  base_borrow_rate: Decimal,
  // The kink_borrow_rate defines the interest rate for borrowing this
  // asset when utilization equals to 'kink_utilization'.
  kink_borrow_rate: Decimal,
  // The max_borrow_rate defines the interest rate for borrowing this
  // asset (seen when utilization is 100%).
  max_borrow_rate: Decimal,
  // The kink_utilization defines the value where the kink rate kicks off for
  // borrow rates.
  kink_utilization: Decimal,
  // The liquidation_incentive determines the portion of bonus collateral of
  // a token type liquidators receive as a liquidation reward.
  liquidation_incentive: Decimal,
  // The symbol_denom and exponent are solely used to update the oracle's accept
  // list of allowed tokens.
  symbol_denom: Option<String>,
  exponent: u32,

  // Allows lending and setting a collateral using this token. Note that
  // withdrawing is always enabled. Disabling lending would be one step in
  // phasing out an asset type.
  enable_msg_lend: Option<bool>,
  // Allows borrowing of this token. Note that repaying is always enabled.
  // Disabling borrowing would be one step in phasing out an asset type, but
  // could also be used from the start for asset types meant to be collateral
  // only, like meTokens.
  enable_msg_borrow: Option<bool>,
  // This should only be used to eliminate an asset completely. A blacklisted
  // asset is treated as though its oracle price is zero, and thus ignored by
  // calculations such as collateral value and borrow limit. Can still be repaid
  // or withdrawn, but not liquidated. A blacklisted token must have enable_lend
  // and enable_borrow set to false. Such tokens can be safely removed from the
  // oracle and price feeder as well.
  blacklist: Option<bool>,
}
