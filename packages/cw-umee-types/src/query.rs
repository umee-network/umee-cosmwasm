use crate::query_leverage::{
  AccountBalancesParams, AccountSummaryParams, LeverageParametersParams, LiquidationTargetsParams,
  MarketSummaryParams, RegisteredTokensParams, UmeeQueryLeverage, ASSIGNED_QUERY_ACCOUNT_BALANCES,
  ASSIGNED_QUERY_ACCOUNT_SUMMARY, ASSIGNED_QUERY_LEVERAGE_PARAMS,
  ASSIGNED_QUERY_LIQUIDATION_TARGETS, ASSIGNED_QUERY_MARKET_SUMMARY,
  ASSIGNED_QUERY_REGISTERED_TOKENS,
};
use crate::query_oracle::{
  ActiveExchangeRatesParams, AggregatePrevoteParams, AggregatePrevotesParams, AggregateVoteParams,
  AggregateVotesParams, ExchangeRatesParams, FeederDelegationParams, MissCounterParams,
  OracleParametersParams, SlashWindowParams, UmeeQueryOracle, ASSIGNED_QUERY_ACTIVE_EXCHANGE_RATES,
  ASSIGNED_QUERY_AGGREGATE_PREVOTE, ASSIGNED_QUERY_AGGREGATE_PREVOTES,
  ASSIGNED_QUERY_AGGREGATE_VOTE, ASSIGNED_QUERY_AGGREGATE_VOTES, ASSIGNED_QUERY_EXCHANGE_RATES,
  ASSIGNED_QUERY_FEEDER_DELEGATION, ASSIGNED_QUERY_MISS_COUNTER, ASSIGNED_QUERY_ORACLE_PARAMS,
  ASSIGNED_QUERY_SLASH_WINDOW,
};
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
  assigned_query: u16,
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
}

// Defines all the implementation related to the StructUmeeQuery
// like creating new query structs, it is needed because
// the fields inside the struct are private, to avoid missmatching
// the query property with the assigned_query field
impl StructUmeeQuery {
  // creates a new exchange_rates query.
  pub fn exchange_rates(exchange_rates_params: ExchangeRatesParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_EXCHANGE_RATES,
      exchange_rates: Some(exchange_rates_params),
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
    }
  }
  // creates a new leverage_parameters query.
  pub fn leverage_parameters(
    leverage_parameters_params: LeverageParametersParams,
  ) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_LEVERAGE_PARAMS,
      exchange_rates: None,
      leverage_parameters: Some(leverage_parameters_params),
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
    }
  }
  // creates a market summary query.
  pub fn market_summary(market_summary_params: MarketSummaryParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_MARKET_SUMMARY,
      exchange_rates: None,
      leverage_parameters: None,
      market_summary: Some(market_summary_params),
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
    }
  }
  // creates a account balances query.
  pub fn account_balances(account_balances_params: AccountBalancesParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_ACCOUNT_BALANCES,
      exchange_rates: None,
      leverage_parameters: None,
      market_summary: None,
      account_balances: Some(account_balances_params),
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
    }
  }
  // creates a account summary query.
  pub fn account_summary(account_summary_params: AccountSummaryParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_ACCOUNT_SUMMARY,
      exchange_rates: None,
      leverage_parameters: None,
      market_summary: None,
      account_balances: None,
      account_summary: Some(account_summary_params),
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
    }
  }
  // creates a new registered_tokens query.
  pub fn registered_tokens(registered_tokens_params: RegisteredTokensParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_REGISTERED_TOKENS,
      exchange_rates: None,
      leverage_parameters: None,
      market_summary: None,
      account_balances: None,
      account_summary: None,
      registered_tokens: Some(registered_tokens_params),
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
    }
  }

  // creates a liquidation targets query.
  pub fn liquidation_targets(
    liquidation_targets_params: LiquidationTargetsParams,
  ) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_LIQUIDATION_TARGETS,
      exchange_rates: None,
      leverage_parameters: None,
      market_summary: None,
      account_balances: None,
      account_summary: None,
      registered_tokens: None,
      liquidation_targets: Some(liquidation_targets_params),
      active_exchange_rates: None,
      feeder_delegation: None,
      miss_counter: None,
      slash_window: None,
      aggregate_prevote: None,
      aggregate_prevotes: None,
      aggregate_vote: None,
      aggregate_votes: None,
      oracle_params: None,
    }
  }
  // creates a active exchange rates query.
  pub fn active_exchange_rates(
    active_exchange_rates_params: ActiveExchangeRatesParams,
  ) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_ACTIVE_EXCHANGE_RATES,
      exchange_rates: None,
      leverage_parameters: None,
      market_summary: None,
      account_balances: None,
      account_summary: None,
      registered_tokens: None,
      liquidation_targets: None,
      active_exchange_rates: Some(active_exchange_rates_params),
      feeder_delegation: None,
      miss_counter: None,
      slash_window: None,
      aggregate_prevote: None,
      aggregate_prevotes: None,
      aggregate_vote: None,
      aggregate_votes: None,
      oracle_params: None,
    }
  }
  // creates a feeder delegation query.
  pub fn feeder_delegation(feeder_delegation_params: FeederDelegationParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_FEEDER_DELEGATION,
      exchange_rates: None,
      leverage_parameters: None,
      market_summary: None,
      account_balances: None,
      account_summary: None,
      registered_tokens: None,
      liquidation_targets: None,
      active_exchange_rates: None,
      feeder_delegation: Some(feeder_delegation_params),
      miss_counter: None,
      slash_window: None,
      aggregate_prevote: None,
      aggregate_prevotes: None,
      aggregate_vote: None,
      aggregate_votes: None,
      oracle_params: None,
    }
  }
  // creates a miss counter query.
  pub fn miss_counter(miss_counter_params: MissCounterParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_MISS_COUNTER,
      exchange_rates: None,
      leverage_parameters: None,
      market_summary: None,
      account_balances: None,
      account_summary: None,
      registered_tokens: None,
      liquidation_targets: None,
      active_exchange_rates: None,
      feeder_delegation: None,
      miss_counter: Some(miss_counter_params),
      slash_window: None,
      aggregate_prevote: None,
      aggregate_prevotes: None,
      aggregate_vote: None,
      aggregate_votes: None,
      oracle_params: None,
    }
  }
  // creates a slash window query.
  pub fn slash_window(slash_window_params: SlashWindowParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_SLASH_WINDOW,
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
      slash_window: Some(slash_window_params),
      aggregate_prevote: None,
      aggregate_prevotes: None,
      aggregate_vote: None,
      aggregate_votes: None,
      oracle_params: None,
    }
  }
  // creates a aggregate prevote query.
  pub fn aggregate_prevote(aggregate_prevote_params: AggregatePrevoteParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_AGGREGATE_PREVOTE,
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
      aggregate_prevote: Some(aggregate_prevote_params),
      aggregate_prevotes: None,
      aggregate_vote: None,
      aggregate_votes: None,
      oracle_params: None,
    }
  }
  // creates a aggregate prevotes query.
  pub fn aggregate_prevotes(aggregate_prevotes_params: AggregatePrevotesParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_AGGREGATE_PREVOTES,
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
      aggregate_prevotes: Some(aggregate_prevotes_params),
      aggregate_vote: None,
      aggregate_votes: None,
      oracle_params: None,
    }
  }
  // creates a aggregate vote query.
  pub fn aggregate_vote(aggregate_vote_params: AggregateVoteParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_AGGREGATE_VOTE,
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
      aggregate_vote: Some(aggregate_vote_params),
      aggregate_votes: None,
      oracle_params: None,
    }
  }
  // creates a aggregate votes query.
  pub fn aggregate_votes(aggregate_votes_params: AggregateVotesParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_AGGREGATE_VOTES,
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
      aggregate_votes: Some(aggregate_votes_params),
      oracle_params: None,
    }
  }
  // creates a new oracle parameters query.
  pub fn oracle_parameters(oracle_parameters_params: OracleParametersParams) -> StructUmeeQuery {
    StructUmeeQuery {
      assigned_query: ASSIGNED_QUERY_ORACLE_PARAMS,
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
      oracle_params: Some(oracle_parameters_params),
    }
  }
}
