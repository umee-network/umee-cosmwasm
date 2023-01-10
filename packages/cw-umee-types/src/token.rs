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

  // Exponent is the power of ten by which to multiply, in order to convert
  // an amount of the token denoted in its symbol denom to the actual amount
  // of its base denom.
  exponent: u32,

  // Enable Msg Supply allows supplying for lending or collateral using this
  // token. `false` means that a token can no longer be supplied.
  // Note that withdrawing is always enabled. Disabling supply would
  // be one step in phasing out an asset type.
  enable_msg_supply: Option<bool>,

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
  // Max Collateral Share specifies how much of the system's overall collateral
  // can be provided by a given token. 1.0 means that the token has no restriction.
  // 0.1 means maximum 10% of system's total collateral value can be provided by this token.
  // Valid values: 0-1.
  max_collateral_share: Decimal,

  // Max Supply Utilization specifies the maximum supply utilization a token is
  // allowed to reach as a direct result of user borrowing. New borrows are not allowed when
  // the supply utilization is above `max_supply_utilization`.
  //    supply_utilization(token) = total_borrowed(token) / total_supply(token)
  // Valid values: 0-1.
  max_supply_utilization: Decimal,

  // Min Collateral Liquidity specifies min limit for the following function:
  //    collateral_liquidity(token) = available(token) / total_collateral(token)
  // Borrowing, collateralizing, or withdrawing assets is not allowed when the
  // result of such action invalidates min_collateral_liquidity.
  // Liquidity can only drop below this value due to interest or liquidations.
  // Valid values: 0 - 1
  min_collateral_liquidity: Decimal,

  // Max Supply is the maximum amount of tokens the protocol can hold.
  // Adding more supply of the given token to the protocol will return an error.
  // Must be a non negative value. 0 means that there is no limit.
  // To mark a token as not valid for supply, `msg_supply` must be set to false.
  max_supply: Decimal,
  // Historic Medians is the number of median historic prices to request from
  // the oracle module when evaluating new borrow positions containing this token.
  // All MsgBorrow, MsgWithdraw, and MsgDecollateralize must result in healthy
  // borrow positions under both current and historic prices. The default value of
  // zero for this field causes current price to be used in those calculations
  // for the affected Token.
  historic_medians: u32,
}
