pub mod aggregate_exchange_rate_prevote;
pub mod aggregate_exchange_rate_vote;
pub mod leverage_parameters;
pub mod msg;
pub mod msg_leverage;
pub mod oracle_parameters;
pub mod query;
pub mod query_leverage;
pub mod query_oracle;
pub mod token;

pub use aggregate_exchange_rate_prevote::AggregateExchangeRatePrevote;
pub use aggregate_exchange_rate_vote::{AggregateExchangeRateVote, ExchangeRateTuple};
pub use leverage_parameters::LeverageParameters;
pub use oracle_parameters::{Denom, OracleParameters};
pub use token::Token;

pub use query::{StructUmeeQuery, UmeeQuery};

pub use query_leverage::{
  AccountBalancesParams, AccountBalancesResponse, AccountSummaryParams, AccountSummaryResponse,
  LeverageParametersParams, LeverageParametersResponse, LiquidationTargetsParams,
  LiquidationTargetsResponse, MarketSummaryParams, MarketSummaryResponse, RegisteredTokensParams,
  RegisteredTokensResponse, UmeeQueryLeverage, ASSIGNED_QUERY_ACCOUNT_BALANCES,
  ASSIGNED_QUERY_ACCOUNT_SUMMARY, ASSIGNED_QUERY_LEVERAGE_PARAMS,
  ASSIGNED_QUERY_LIQUIDATION_TARGETS, ASSIGNED_QUERY_MARKET_SUMMARY,
  ASSIGNED_QUERY_REGISTERED_TOKENS,
};

pub use query_oracle::{
  ActiveExchangeRatesParams, ActiveExchangeRatesResponse, AggregatePrevoteParams,
  AggregatePrevoteResponse, AggregatePrevotesParams, AggregatePrevotesResponse,
  AggregateVoteParams, AggregateVoteResponse, AggregateVotesParams, AggregateVotesResponse,
  ExchangeRatesParams, ExchangeRatesResponse, FeederDelegationParams, FeederDelegationResponse,
  MissCounterParams, MissCounterResponse, OracleParametersParams, OracleParametersResponse,
  SlashWindowParams, SlashWindowResponse, UmeeQueryOracle, ASSIGNED_QUERY_ACTIVE_EXCHANGE_RATES,
  ASSIGNED_QUERY_AGGREGATE_PREVOTE, ASSIGNED_QUERY_AGGREGATE_PREVOTES,
  ASSIGNED_QUERY_AGGREGATE_VOTES, ASSIGNED_QUERY_EXCHANGE_RATES, ASSIGNED_QUERY_FEEDER_DELEGATION,
  ASSIGNED_QUERY_MISS_COUNTER, ASSIGNED_QUERY_SLASH_WINDOW,
};

pub use msg_leverage::{
  BorrowParams, CollateralizeParams, DecollateralizeParams, LiquidateParams, RepayParams,
  SupplyParams, UmeeMsgLeverage, WithdrawParams, ASSIGNED_MSG_BORROW, ASSIGNED_MSG_COLLATERALIZE,
  ASSIGNED_MSG_DECOLLATERALIZE, ASSIGNED_MSG_LIQUIDATE, ASSIGNED_MSG_REPAY, ASSIGNED_MSG_SUPPLY,
  ASSIGNED_MSG_WITHDRAW,
};

pub use msg::{StructUmeeMsg, UmeeMsg};

// This is a signal, such that any contract that imports these helpers will only run on the
// umee blockchain, it makes mandatory that the blockchain have the "umee" inside
// the supported features when instantiating a new wasm keeper
#[no_mangle]
extern "C" fn requires_umee() {}
