use cosmwasm_std::Decimal256;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// LeverageParameters defines the parameters for the leverage module.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct LeverageParameters {
  // The complete_liquidation_threshold determines how far over their borrow
  // limit a borrower must be in order for their positions to be liquidated
  // fully in a single event.
  complete_liquidation_threshold: Decimal256,
  // The minimum_close_factor determines the portion of a borrower's position
  // that can be liquidated in a single event, when the borrower is just barely
  // over their borrow limit.
  minimum_close_factor: Decimal256,
  // The oracle_reward_factor determines the portion of interest accrued on
  // borrows that is sent to the oracle module to fund its reward pool.
  oracle_reward_factor: Decimal256,
  // The small_liquidation_size determines the USD value at which a borrow is
  // considered small enough to be liquidated in a single transaction, bypassing
  // dynamic close factor.
  small_liquidation_size: Decimal256,
  // Direct Liquidation Fee is a reduction factor in liquidation incentive
  // experienced by liquidators who choose to receive base assets instead of
  // uTokens as liquidation rewards.
  // Valid values: 0-1.
  direct_liquidation_fee: Decimal256,
}
