use crate::query_leverage::{
  AccountBalancesParams, AccountSummaryParams, BadDebtsParams, LeverageParametersParams,
  LiquidationTargetsParams, MarketSummaryParams, MaxWithdrawParams, RegisteredTokensParams,
  UmeeQueryLeverage,
};
use crate::query_oracle::{
  ActiveExchangeRatesParams, AggregatePrevoteParams, AggregatePrevotesParams, AggregateVoteParams,
  AggregateVotesParams, ExchangeRatesParams, FeederDelegationParams, MedianDeviationsParams,
  MediansParams, MissCounterParams, OracleParametersParams, SlashWindowParams, UmeeQueryOracle,
};
use crate::MaxBorrowParams;
use cosmwasm_std::CustomQuery;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Define the implementation necessary for cosmwasm "custom" queries
impl CustomQuery for StructUmeeQuery {}
impl CustomQuery for UmeeQuery {}

// UmeeQuery combines all the native modules from umee as enum
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UmeeQuery {
  // Leverage wraps all the query enums from the leverage module
  Leverage(UmeeQueryLeverage),
  // Oracle wraps all the query enums from the oracle module
  Oracle(UmeeQueryOracle),
}

// StructUmeeQuery expected structure to query umee native modules
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StructUmeeQuery {
  exchange_rates: Option<ExchangeRatesParams>,
  leverage_parameters: Option<LeverageParametersParams>,
  market_summary: Option<MarketSummaryParams>,
  account_balances: Option<AccountBalancesParams>,
  account_summary: Option<AccountSummaryParams>,
  registered_tokens: Option<RegisteredTokensParams>,
  liquidation_targets: Option<LiquidationTargetsParams>,
  active_exchange_rates: Option<ActiveExchangeRatesParams>,
  feeder_delegation: Option<FeederDelegationParams>,
  miss_counter: Option<MissCounterParams>,
  slash_window: Option<SlashWindowParams>,
  aggregate_prevote: Option<AggregatePrevoteParams>,
  aggregate_prevotes: Option<AggregatePrevotesParams>,
  aggregate_vote: Option<AggregateVoteParams>,
  aggregate_votes: Option<AggregateVotesParams>,
  oracle_params: Option<OracleParametersParams>,
  bad_debts_params: Option<BadDebtsParams>,
  max_withdraw_params: Option<MaxWithdrawParams>,
  max_borrow_params: Option<MaxBorrowParams>,
  medians_params: Option<MediansParams>,
  median_deviations_params: Option<MedianDeviationsParams>,
}

fn default_struct_umee_query() -> StructUmeeQuery {
  StructUmeeQuery {
    exchange_rates: None,
    leverage_parameters: None,
    market_summary: None,
    account_balances: None,
    account_summary: None,
    registered_tokens: None,
    liquidation_targets: None,
    active_exchange_rates: None,
    feeder_delegation: None,
    miss_counter: None,
    slash_window: None,
    aggregate_prevote: None,
    aggregate_prevotes: None,
    aggregate_vote: None,
    aggregate_votes: None,
    oracle_params: None,
    bad_debts_params: None,
    max_withdraw_params: None,
    max_borrow_params: None,
    medians_params: None,
    median_deviations_params: None,
  }
}

// Defines all the implementation related to the StructUmeeQuery
// like creating new query structs, it is needed because
// the fields inside the struct are private, to avoid missmatching
// the query property with the assigned_query field
impl StructUmeeQuery {
  // creates a new exchange_rates query.
  pub fn exchange_rates(exchange_rates_params: ExchangeRatesParams) -> StructUmeeQuery {
    let mut q = default_struct_umee_query();
    q.exchange_rates = Some(exchange_rates_params);
    return q;
  }
  // creates a new leverage_parameters query.
  pub fn leverage_parameters(
    leverage_parameters_params: LeverageParametersParams,
  ) -> StructUmeeQuery {
    let mut q: StructUmeeQuery = default_struct_umee_query();
    q.leverage_parameters = Some(leverage_parameters_params);
    return q;
  }
  // creates a market summary query.
  pub fn market_summary(market_summary_params: MarketSummaryParams) -> StructUmeeQuery {
    let mut q: StructUmeeQuery = default_struct_umee_query();
    q.market_summary = Some(market_summary_params);
    return q;
  }
  // creates a account balances query.
  pub fn account_balances(account_balances_params: AccountBalancesParams) -> StructUmeeQuery {
    let mut q: StructUmeeQuery = default_struct_umee_query();
    q.account_balances = Some(account_balances_params);
    return q;
  }
  // creates a account summary query.
  pub fn account_summary(account_summary_params: AccountSummaryParams) -> StructUmeeQuery {
    let mut q: StructUmeeQuery = default_struct_umee_query();
    q.account_summary = Some(account_summary_params);
    return q;
  }
  // creates a new registered_tokens query.
  pub fn registered_tokens(registered_tokens_params: RegisteredTokensParams) -> StructUmeeQuery {
    let mut q: StructUmeeQuery = default_struct_umee_query();
    q.registered_tokens = Some(registered_tokens_params);
    return q;
  }
  // creates a liquidation targets query.
  pub fn liquidation_targets(
    liquidation_targets_params: LiquidationTargetsParams,
  ) -> StructUmeeQuery {
    let mut q: StructUmeeQuery = default_struct_umee_query();
    q.liquidation_targets = Some(liquidation_targets_params);
    return q;
  }
  // creates a new bad debts parameters query.
  pub fn bad_debts_parameters(bad_debts_params: BadDebtsParams) -> StructUmeeQuery {
    let mut q: StructUmeeQuery = default_struct_umee_query();
    q.bad_debts_params = Some(bad_debts_params);
    return q;
  }
  // creates a new max withdraw params query.
  pub fn max_withdraw_params(max_withdraw_params: MaxWithdrawParams) -> StructUmeeQuery {
    let mut q: StructUmeeQuery = default_struct_umee_query();
    q.max_withdraw_params = Some(max_withdraw_params);
    return q;
  }
  // creates a new max borrows params query.
  pub fn max_borrow_params(max_borrow_params: MaxBorrowParams) -> StructUmeeQuery {
    let mut q: StructUmeeQuery = default_struct_umee_query();
    q.max_borrow_params = Some(max_borrow_params);
    return q;
  }
  // creates a active exchange rates query.
  pub fn active_exchange_rates(
    active_exchange_rates_params: ActiveExchangeRatesParams,
  ) -> StructUmeeQuery {
    let mut q: StructUmeeQuery = default_struct_umee_query();
    q.active_exchange_rates = Some(active_exchange_rates_params);
    return q;
  }
  // creates a feeder delegation query.
  pub fn feeder_delegation(feeder_delegation_params: FeederDelegationParams) -> StructUmeeQuery {
    let mut q: StructUmeeQuery = default_struct_umee_query();
    q.feeder_delegation = Some(feeder_delegation_params);
    return q;
  }
  // creates a miss counter query.
  pub fn miss_counter(miss_counter_params: MissCounterParams) -> StructUmeeQuery {
    let mut q: StructUmeeQuery = default_struct_umee_query();
    q.miss_counter = Some(miss_counter_params);
    return q;
  }
  // creates a slash window query.
  pub fn slash_window(slash_window_params: SlashWindowParams) -> StructUmeeQuery {
    let mut q: StructUmeeQuery = default_struct_umee_query();
    q.slash_window = Some(slash_window_params);
    return q;
  }
  // creates a aggregate prevote query.
  pub fn aggregate_prevote(aggregate_prevote_params: AggregatePrevoteParams) -> StructUmeeQuery {
    let mut q: StructUmeeQuery = default_struct_umee_query();
    q.aggregate_prevote = Some(aggregate_prevote_params);
    return q;
  }
  // creates a aggregate prevotes query.
  pub fn aggregate_prevotes(aggregate_prevotes_params: AggregatePrevotesParams) -> StructUmeeQuery {
    let mut q: StructUmeeQuery = default_struct_umee_query();
    q.aggregate_prevotes = Some(aggregate_prevotes_params);
    return q;
  }
  // creates a aggregate vote query.
  pub fn aggregate_vote(aggregate_vote_params: AggregateVoteParams) -> StructUmeeQuery {
    let mut q: StructUmeeQuery = default_struct_umee_query();
    q.aggregate_vote = Some(aggregate_vote_params);
    return q;
  }
  // creates a aggregate votes query.
  pub fn aggregate_votes(aggregate_votes_params: AggregateVotesParams) -> StructUmeeQuery {
    let mut q: StructUmeeQuery = default_struct_umee_query();
    q.aggregate_votes = Some(aggregate_votes_params);
    return q;
  }
  // creates a new oracle parameters query.
  pub fn oracle_parameters(oracle_parameters_params: OracleParametersParams) -> StructUmeeQuery {
    let mut q: StructUmeeQuery = default_struct_umee_query();
    q.oracle_params = Some(oracle_parameters_params);
    return q;
  }

  // creates a new medians query.
  pub fn medians_params(medians_params: MediansParams) -> StructUmeeQuery {
    let mut q: StructUmeeQuery = default_struct_umee_query();
    q.medians_params = Some(medians_params);
    return q;
  }
  // creates a new median deviations params query.
  pub fn median_deviations_params(
    median_deviations_params: MedianDeviationsParams,
  ) -> StructUmeeQuery {
    let mut q: StructUmeeQuery = default_struct_umee_query();
    q.median_deviations_params = Some(median_deviations_params);
    return q;
  }
}
