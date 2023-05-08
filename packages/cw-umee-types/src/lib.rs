pub mod aggregate_exchange_rate_prevote;
pub mod aggregate_exchange_rate_vote;
pub mod bad_debt;
pub mod error;
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
pub use bad_debt::BadDebt;
pub use leverage_parameters::LeverageParameters;
pub use oracle_parameters::{Denom, OracleParameters};
pub use token::Token;

pub use query::{StructUmeeQuery, UmeeQuery};

pub use query_leverage::{
  AccountBalancesParams, AccountBalancesResponse, AccountSummaryParams, AccountSummaryResponse,
  BadDebtsParams, BadDebtsResponse, LeverageParametersParams, LeverageParametersResponse,
  LeverageQueries, LiquidationTargetsParams, LiquidationTargetsResponse, MarketSummaryParams,
  MarketSummaryResponse, MaxBorrowParams, MaxWithdrawParams, MaxWithdrawResponse,
  RegisteredTokensParams, RegisteredTokensResponse, UmeeQueryLeverage,
};

pub use query_oracle::{
  ActiveExchangeRatesParams, ActiveExchangeRatesResponse, AggregatePrevoteParams,
  AggregatePrevoteResponse, AggregatePrevotesParams, AggregatePrevotesResponse,
  AggregateVoteParams, AggregateVoteResponse, AggregateVotesParams, AggregateVotesResponse,
  ExchangeRatesParams, ExchangeRatesResponse, FeederDelegationParams, FeederDelegationResponse,
  MedianDeviationsParams, MedianDeviationsParamsResponse, MediansParams, MediansParamsResponse,
  MissCounterParams, MissCounterResponse, OracleParametersParams, OracleParametersResponse,
  OracleQueries, SlashWindowParams, SlashWindowResponse, UmeeQueryOracle,
};

pub use msg_leverage::{
  BorrowParams, CollateralizeParams, DecollateralizeParams, LiquidateParams, MsgMaxBorrowParams,
  MsgMaxWithdrawParams, MsgTypes, RepayParams, SupplyCollateralParams, SupplyParams,
  UmeeMsgLeverage, WithdrawParams,
};

pub use msg::{StructUmeeMsg, UmeeMsg};

// This is a signal, such that any contract that imports these helpers will only run on the
// umee blockchain, it makes mandatory that the blockchain have the "umee" inside
// the supported features when instantiating a new wasm keeper
#[no_mangle]
extern "C" fn requires_umee() {}
