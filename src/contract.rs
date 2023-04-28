#[cfg(not(feature = "library"))]
use cosmwasm_std::{
  entry_point, from_binary, to_binary, to_vec, Addr, Binary, ContractResult, Deps, DepsMut, Env,
  MessageInfo, QueryRequest, Response, StdError, StdResult, SystemResult,
};
use cw2::set_contract_version;
use cw_umee_types::msg_leverage::{
  MsgMaxBorrowParams, MsgMaxWithDrawParams, SupplyCollateralParams,
};
use cw_umee_types::query_leverage::{
  BadDebtsParams, BadDebtsResponse, MaxBorrowParams, MaxBorrowResponse, MaxWithdrawParams,
  MaxWithdrawResponse,
};
use cw_umee_types::query_oracle::{
  MedianDeviationsParams, MedianDeviationsParamsResponse, MediansParams, MediansParamsResponse,
};
use cw_umee_types::{
  AccountBalancesParams, AccountBalancesResponse, AccountSummaryParams, ActiveExchangeRatesParams,
  ActiveExchangeRatesResponse, AggregatePrevoteParams, AggregatePrevoteResponse,
  AggregatePrevotesParams, AggregatePrevotesResponse, AggregateVoteParams, AggregateVoteResponse,
  AggregateVotesParams, AggregateVotesResponse, BorrowParams, CollateralizeParams,
  DecollateralizeParams, ExchangeRatesParams, ExchangeRatesResponse, FeederDelegationParams,
  FeederDelegationResponse, LeverageParametersParams, LeverageParametersResponse, LiquidateParams,
  LiquidationTargetsParams, LiquidationTargetsResponse, MarketSummaryParams, MarketSummaryResponse,
  MissCounterParams, MissCounterResponse, OracleParametersParams, OracleParametersResponse,
  RegisteredTokensParams, RegisteredTokensResponse, RepayParams, SlashWindowParams,
  SlashWindowResponse, StructUmeeMsg, StructUmeeQuery, SupplyParams, UmeeMsg, UmeeMsgLeverage,
  UmeeQuery, UmeeQueryLeverage, UmeeQueryOracle, WithdrawParams,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, OwnerResponse, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:umee-cosmwasm";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// smartcontract constructor
// starts by setting the sender of the msg as the owner
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  _: InstantiateMsg,
) -> Result<Response, ContractError> {
  let state = State {
    owner: info.sender.clone(),
  };
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  STATE.save(deps.storage, &state)?;

  Ok(
    Response::new()
      .add_attribute("method", "instantiate")
      .add_attribute("owner", info.sender),
  )
}

// executes changes to the state of the contract, it receives messages DepsMut
// that contains the contract state with write permissions
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response<StructUmeeMsg>, ContractError> {
  match msg {
    // receives the new owner and tries to change it in the contract state
    ExecuteMsg::ChangeOwner { new_owner } => try_change_owner(deps, info, new_owner),
    ExecuteMsg::Chain(cosmos_umee_msg) => msg_chain(*cosmos_umee_msg),
    ExecuteMsg::Umee(UmeeMsg::Leverage(execute_leverage_msg)) => {
      execute_leverage(execute_leverage_msg)
    }
    ExecuteMsg::Supply(supply_params) => execute_lend(supply_params),
  }
}

// tries to change the owner, but it could fail and respond as Unauthorized
pub fn try_change_owner(
  deps: DepsMut,
  info: MessageInfo,
  new_owner: Addr,
) -> Result<Response<StructUmeeMsg>, ContractError> {
  STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
    if info.sender != state.owner {
      return Err(ContractError::Unauthorized {});
    }
    state.owner = new_owner;
    Ok(state)
  })?;
  Ok(Response::<StructUmeeMsg>::new().add_attribute("method", "change_owner"))
}

// msg_chain sends any message in the chain native modules
fn msg_chain(umee_msg: StructUmeeMsg) -> Result<Response<StructUmeeMsg>, ContractError> {
  if !umee_msg.valid() {
    return Err(ContractError::CustomError {
      val: String::from("invalid umee msg"),
    });
  }

  let res = Response::new()
    .add_attribute("method", umee_msg.assigned_str())
    .add_message(umee_msg);

  Ok(res)
}

// execute_leverage handles the execution of every msg of leverage umee native modules
fn execute_leverage(
  execute_leverage_msg: UmeeMsgLeverage,
) -> Result<Response<StructUmeeMsg>, ContractError> {
  match execute_leverage_msg {
    UmeeMsgLeverage::Supply(supply_params) => execute_lend(supply_params),
    UmeeMsgLeverage::Withdraw(withdraw_params) => execute_withdraw(withdraw_params),
    UmeeMsgLeverage::MaxWithDraw(max_withdraw_params) => execute_max_withdraw(max_withdraw_params),
    UmeeMsgLeverage::Collateralize(collateralize_params) => {
      execute_collateralize(collateralize_params)
    }
    UmeeMsgLeverage::Decollateralize(decollateralize_params) => {
      execute_decollateralize(decollateralize_params)
    }
    UmeeMsgLeverage::Borrow(borrow_params) => execute_borrow(borrow_params),
    UmeeMsgLeverage::MaxBorrow(borrow_params) => execute_max_borrow(borrow_params),
    UmeeMsgLeverage::Repay(repay_params) => execute_repay(repay_params),
    UmeeMsgLeverage::Liquidate(liquidate_params) => execute_liquidate(liquidate_params),
    UmeeMsgLeverage::SupplyCollateral(supply_collateralize_params) => {
      execute_supply_collateralize(supply_collateralize_params)
    }
  }
}

// execute_lend sends umee leverage module an message of Supply.
fn execute_lend(supply_params: SupplyParams) -> Result<Response<StructUmeeMsg>, ContractError> {
  let msg = StructUmeeMsg::supply(supply_params);
  Ok(
    Response::new()
      .add_attribute("method", msg.assigned_str())
      .add_message(msg),
  )
}

// execute_withdraw sends umee leverage module an message of Withdraw.
fn execute_withdraw(
  withdraw_params: WithdrawParams,
) -> Result<Response<StructUmeeMsg>, ContractError> {
  let msg = StructUmeeMsg::withdraw(withdraw_params);
  Ok(
    Response::new()
      .add_attribute("method", msg.assigned_str())
      .add_message(msg),
  )
}

// execute_max_withdraw sends umee leverage module an message of MaxWithdraw.
fn execute_max_withdraw(
  max_withdraw_params: MsgMaxWithDrawParams,
) -> Result<Response<StructUmeeMsg>, ContractError> {
  let msg = StructUmeeMsg::max_withdraw(max_withdraw_params);
  Ok(
    Response::new()
      .add_attribute("method", msg.assigned_str())
      .add_message(msg),
  )
}

// execute_collateralize sends umee leverage module an message of Collateralize.
fn execute_collateralize(
  collateralize_params: CollateralizeParams,
) -> Result<Response<StructUmeeMsg>, ContractError> {
  let msg = StructUmeeMsg::collateralize(collateralize_params);
  Ok(
    Response::new()
      .add_attribute("method", msg.assigned_str())
      .add_message(msg),
  )
}

// execute_decollateralize sends umee leverage module an message of Decollateralize.
fn execute_decollateralize(
  decollateralize_params: DecollateralizeParams,
) -> Result<Response<StructUmeeMsg>, ContractError> {
  let msg = StructUmeeMsg::decollateralize(decollateralize_params);
  Ok(
    Response::new()
      .add_attribute("method", msg.assigned_str())
      .add_message(msg),
  )
}

// execute_borrow sends umee leverage module an message of Borrow.
fn execute_borrow(borrow_params: BorrowParams) -> Result<Response<StructUmeeMsg>, ContractError> {
  let msg = StructUmeeMsg::borrow(borrow_params);
  Ok(
    Response::new()
      .add_attribute("method", msg.assigned_str())
      .add_message(msg),
  )
}

// execute_max_borrow sends umee leverage module an message of MaxBorrow.
fn execute_max_borrow(
  max_borrow_params: MsgMaxBorrowParams,
) -> Result<Response<StructUmeeMsg>, ContractError> {
  let msg = StructUmeeMsg::max_borrow(max_borrow_params);
  Ok(
    Response::new()
      .add_attribute("method", msg.assigned_str())
      .add_message(msg),
  )
}

// execute_repay sends umee leverage module an message of Repay.
fn execute_repay(repay_params: RepayParams) -> Result<Response<StructUmeeMsg>, ContractError> {
  let msg = StructUmeeMsg::repay(repay_params);
  Ok(
    Response::new()
      .add_attribute("method", msg.assigned_str())
      .add_message(msg),
  )
}

// execute_liquidate sends umee leverage module an message of Liquidate.
fn execute_liquidate(
  liquidate_params: LiquidateParams,
) -> Result<Response<StructUmeeMsg>, ContractError> {
  let msg = StructUmeeMsg::liquidate(liquidate_params);
  Ok(
    Response::new()
      .add_attribute("method", msg.assigned_str())
      .add_message(msg),
  )
}

// execute_supply_collateralize sends umee leverage module an message of Supply Collateralize.
fn execute_supply_collateralize(
  supply_collateral_params: SupplyCollateralParams,
) -> Result<Response<StructUmeeMsg>, ContractError> {
  let msg = StructUmeeMsg::supply_collateral(supply_collateral_params);
  Ok(
    Response::new()
      .add_attribute("method", msg.assigned_str())
      .add_message(msg),
  )
}
// queries doesn't change the state, but it open the state with read permissions
// it can also query from native modules "bank, stake, custom..."
// returns an json wrapped data, like:
// {
//   "data": ...
// }
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
  match msg {
    // returns OwnerResponse the current contract owner
    // expected json input:
    // {
    //   "get_owner": {}
    // }
    // successful json output:
    // {
    //   "data": {
    //     "owner": "umee1y6xz2ggfc0pcsmyjlekh0j9pxh6hk87ymc9due"
    //   }
    // }
    QueryMsg::GetOwner {} => to_binary(&query_owner(deps)?),

    // queries for anything availabe from the blockchain native modules
    // "iterator, staking, stargate, custom"
    // example json input for custom module:
    // {
    //   "chain": {
    //     "custom": {
    //       "assigned_query": uint16,
    //       "query_func_name": {
    //         ...
    //       }
    //     }
    //   }
    // }
    // successful json output:
    // {
    //   "data": {
    //     ...
    //   }
    // }
    QueryMsg::Chain(request) => query_chain(deps, &request),

    QueryMsg::Umee(umee_query_box) => query_umee(deps, _env, *umee_query_box),

    // consumes the query_chain wrapping the JSON to call directly
    // the ExchangeRates query from the oracle umee native module
    // expected json input:
    // {
    //   "get_exchange_rate_base": {
    //     "denom": "uumee"
    //   }
    // }
    // successful json output:
    // {
    //   "data": {
    //     "borrowed": [
    //       {
    //         "denom": "uumee",
    //         "amount": "50001"
    //       }
    //     ]
    //   }
    // }
    QueryMsg::ExchangeRates(exchange_rates_params) => {
      to_binary(&query_exchange_rates(deps, exchange_rates_params)?)
    }
    QueryMsg::RegisteredTokens(registered_tokens_params) => {
      to_binary(&query_registered_tokens(deps, registered_tokens_params)?)
    }
    QueryMsg::LeverageParameters(leverage_parameters_params) => to_binary(
      &query_leverage_parameters(deps, leverage_parameters_params)?,
    ),
  }
}

// query_umee contains the umee leverage available queries
fn query_umee(deps: Deps, _env: Env, umee_msg: UmeeQuery) -> StdResult<Binary> {
  match umee_msg {
    // consumes the query_chain wrapped by Umee Leverage enums
    // to clarift the JSON queries to umee leverage native module
    // example json input:
    // {
    //   "umee": {
    //     "leverage": {
    //       "query_func_name": {
    //         ...
    //       }
    //     }
    //   }
    // }
    // successful json output:
    // {
    //   "data": {
    //     ...
    //   }
    // }
    UmeeQuery::Leverage(leverage) => query_leverage(deps, _env, leverage),

    // consumes the query_chain wrapped by Umee Leverage enums
    // to clarift the JSON queries to umee leverage native module
    // example json input:
    // {
    //   "umee": {
    //     "oracle": {
    //       "query_func_name": {
    //         ...
    //       }
    //     }
    //   }
    // }
    // successful json output:
    // {
    //   "data": {
    //     ...
    //   }
    // }
    UmeeQuery::Oracle(oracle) => query_oracle(deps, _env, oracle),
  }
}

// returns the current owner of the contract from the state
fn query_owner(deps: Deps) -> StdResult<OwnerResponse> {
  let state = STATE.load(deps.storage)?;
  Ok(OwnerResponse { owner: state.owner })
}

// query_chain queries for any availabe query in the chain native modules
fn query_chain(deps: Deps, request: &QueryRequest<StructUmeeQuery>) -> StdResult<Binary> {
  let raw = to_vec(request).map_err(|serialize_err| {
    StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
  })?;
  match deps.querier.raw_query(&raw) {
    SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
      "Querier system error: {}",
      system_err
    ))),
    SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(format!(
      "Querier contract error: {}",
      contract_err
    ))),
    SystemResult::Ok(ContractResult::Ok(value)) => Ok(value),
  }
}

// query_leverage contains the umee leverage available queries
fn query_leverage(deps: Deps, _env: Env, msg: UmeeQueryLeverage) -> StdResult<Binary> {
  match msg {
    UmeeQueryLeverage::LeverageParameters(leverage_parameters_params) => to_binary(
      &query_leverage_parameters(deps, leverage_parameters_params)?,
    ),
    UmeeQueryLeverage::RegisteredTokens(registered_tokens_params) => {
      to_binary(&query_registered_tokens(deps, registered_tokens_params)?)
    }
    UmeeQueryLeverage::MarketSummary(market_summary_params) => {
      to_binary(&query_market_summary(deps, market_summary_params)?)
    }
    UmeeQueryLeverage::AccountBalances(account_balances_params) => {
      to_binary(&query_account_balances(deps, account_balances_params)?)
    }
    UmeeQueryLeverage::AccountSummary(account_summary_params) => {
      to_binary(&query_account_summary(deps, account_summary_params)?)
    }
    UmeeQueryLeverage::LiquidationTargets(liquidation_targets_params) => to_binary(
      &query_liquidation_targets(deps, liquidation_targets_params)?,
    ),
    UmeeQueryLeverage::BadDebts(bad_debts_params) => {
      to_binary(&query_bad_debts(deps, bad_debts_params)?)
    }
    UmeeQueryLeverage::MaxWithdraw(max_withdraw_params) => {
      to_binary(&query_max_withdraw(deps, max_withdraw_params)?)
    }
    UmeeQueryLeverage::MaxBorrow(max_borrow_params) => {
      to_binary(&query_max_borrow(deps, max_borrow_params)?)
    }
  }
}

// query_oracle contains the umee oracle available queries
fn query_oracle(deps: Deps, _env: Env, msg: UmeeQueryOracle) -> StdResult<Binary> {
  match msg {
    // consumes the query_chain wrapped by Umee Leverage enums
    // to clarift the JSON queries to umee leverage native module
    // example json input:
    // {
    //   "umee": {
    //     "oracle": {
    //       "exchange_rates": {
    //         "denom": "uumee"
    //       }
    //     }
    //   }
    // }
    // successful json output:
    // {
    //   "data": {
    //     "exchange_rate_base": "0.0000032"
    //   }
    // }
    UmeeQueryOracle::ExchangeRates(exchange_rates_params) => {
      to_binary(&query_exchange_rates(deps, exchange_rates_params)?)
    }
    UmeeQueryOracle::ActiveExchangeRates(active_exchange_rates_params) => to_binary(
      &query_active_exchange_rates(deps, active_exchange_rates_params)?,
    ),
    UmeeQueryOracle::FeederDelegation(feeder_delegation_params) => {
      to_binary(&query_feeder_delegation(deps, feeder_delegation_params)?)
    }
    UmeeQueryOracle::MissCounter(miss_counter_params) => {
      to_binary(&query_miss_counter(deps, miss_counter_params)?)
    }
    UmeeQueryOracle::SlashWindow(slash_window_params) => {
      to_binary(&query_slash_window(deps, slash_window_params)?)
    }
    UmeeQueryOracle::AggregatePrevote(aggregate_prevote_params) => {
      to_binary(&query_aggregate_prevote(deps, aggregate_prevote_params)?)
    }
    UmeeQueryOracle::AggregatePrevotes(aggregate_prevotes_params) => {
      to_binary(&query_aggregate_prevotes(deps, aggregate_prevotes_params)?)
    }
    UmeeQueryOracle::AggregateVote(aggregate_vote_params) => {
      to_binary(&query_aggregate_vote(deps, aggregate_vote_params)?)
    }
    UmeeQueryOracle::AggregateVotes(aggregate_votes_params) => {
      to_binary(&query_aggregate_votes(deps, aggregate_votes_params)?)
    }
    UmeeQueryOracle::OracleParameters(oracle_parameters_params) => {
      to_binary(&query_oracle_parameters(deps, oracle_parameters_params)?)
    }
    UmeeQueryOracle::Medians(median_params) => to_binary(&query_medians(deps, median_params)?),
    UmeeQueryOracle::MedianDeviations(median_deviations_params) => {
      to_binary(&query_median_deviations(deps, median_deviations_params)?)
    }
  }
}

// query_registered_tokens receives the get all registered tokens
// query params and creates an query request to the native modules
// with query_chain wrapping the response to the actual
// RegisteredTokensResponse struct
fn query_registered_tokens(
  deps: Deps,
  registered_tokens_params: RegisteredTokensParams,
) -> StdResult<RegisteredTokensResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::registered_tokens(registered_tokens_params));

  let registered_tokens_response: RegisteredTokensResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<RegisteredTokensResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => registered_tokens_response = response,
      };
    }
  }

  Ok(registered_tokens_response)
}

// query_leverage_parameters creates an query request to the native modules
// with query_chain wrapping the response to the actual
// LeverageParametersResponse struct
fn query_leverage_parameters(
  deps: Deps,
  leverage_parameters_params: LeverageParametersParams,
) -> StdResult<LeverageParametersResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::leverage_parameters(
    leverage_parameters_params,
  ));

  let leverage_parameters_response: LeverageParametersResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<LeverageParametersResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => leverage_parameters_response = response,
      };
    }
  }

  Ok(leverage_parameters_response)
}

// query_account_balances creates an query request to the native modules
// with query_chain wrapping the response to the actual
// AccountBalancesResponse struct.
fn query_account_balances(
  deps: Deps,
  account_balances_params: AccountBalancesParams,
) -> StdResult<AccountBalancesResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::account_balances(account_balances_params));

  let account_balances_response: AccountBalancesResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<AccountBalancesResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => account_balances_response = response,
      };
    }
  }

  Ok(account_balances_response)
}

// query_account_summary creates an query request to the native modules
// with query_chain wrapping the response to the actual
// AccountsummaryResponse struct.
fn query_account_summary(
  deps: Deps,
  account_summary_params: AccountSummaryParams,
) -> StdResult<AccountSummaryParams> {
  let request = QueryRequest::Custom(StructUmeeQuery::account_summary(account_summary_params));

  let account_summary_response: AccountSummaryParams;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<AccountSummaryParams>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => account_summary_response = response,
      };
    }
  }

  Ok(account_summary_response)
}

// query_liquidation_targets creates an query request to the native modules
// with query_chain wrapping the response to the actual
// LiquidationTargetsResponse struct.
fn query_liquidation_targets(
  deps: Deps,
  liquidation_targets_params: LiquidationTargetsParams,
) -> StdResult<LiquidationTargetsResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::liquidation_targets(
    liquidation_targets_params,
  ));

  let liquidation_targets_response: LiquidationTargetsResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<LiquidationTargetsResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => liquidation_targets_response = response,
      };
    }
  }

  Ok(liquidation_targets_response)
}

fn query_bad_debts(deps: Deps, bad_debts_params: BadDebtsParams) -> StdResult<BadDebtsResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::bad_debts_parameters(bad_debts_params));

  let bad_debts_response: BadDebtsResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<BadDebtsResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => bad_debts_response = response,
      };
    }
  }

  Ok(bad_debts_response)
}

// query_max_withdraw
fn query_max_withdraw(
  deps: Deps,
  max_withdraw_params: MaxWithdrawParams,
) -> StdResult<MaxWithdrawResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::max_withdraw_params(max_withdraw_params));

  let max_withdraw_response: MaxWithdrawResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<MaxWithdrawResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => max_withdraw_response = response,
      };
    }
  }

  Ok(max_withdraw_response)
}

// query_max_borrow
fn query_max_borrow(
  deps: Deps,
  max_borrow_params: MaxBorrowParams,
) -> StdResult<MaxBorrowResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::max_borrow_params(max_borrow_params));

  let max_borrow_response: MaxBorrowResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<MaxBorrowResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => max_borrow_response = response,
      };
    }
  }

  Ok(max_borrow_response)
}

// query_market_summary creates an query request to the native modules
// with query_chain wrapping the response to the actual
// MarketSummaryResponse struct.
fn query_market_summary(
  deps: Deps,
  market_summary_params: MarketSummaryParams,
) -> StdResult<MarketSummaryResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::market_summary(market_summary_params));

  let market_summary_response: MarketSummaryResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<MarketSummaryResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => market_summary_response = response,
      };
    }
  }

  Ok(market_summary_response)
}

// query_exchange_rates receives the get exchange rate base
// query params and creates an query request to the native modules
// with query_chain wrapping the response to the actual
// ExchangeRatesResponse struct
fn query_exchange_rates(
  deps: Deps,
  exchange_rates_params: ExchangeRatesParams,
) -> StdResult<ExchangeRatesResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::exchange_rates(exchange_rates_params));

  let exchange_rates_resp: ExchangeRatesResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<ExchangeRatesResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => exchange_rates_resp = response,
      };
    }
  }

  Ok(exchange_rates_resp)
}

// query_active_exchange_rates receives the get exchange rate base
// query params and creates an query request to the native modules
// with query_chain wrapping the response to the actual
// ActiveExchangeRatesResponse struct
fn query_active_exchange_rates(
  deps: Deps,
  active_exchange_rates_params: ActiveExchangeRatesParams,
) -> StdResult<ActiveExchangeRatesResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::active_exchange_rates(
    active_exchange_rates_params,
  ));

  let active_exchange_rates_resp: ActiveExchangeRatesResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<ActiveExchangeRatesResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => active_exchange_rates_resp = response,
      };
    }
  }

  Ok(active_exchange_rates_resp)
}

// query_feeder_delegation receives the get exchange rate base
// query params and creates an query request to the native modules
// with query_chain wrapping the response to the actual
// FeederDelegationResponse struct
fn query_feeder_delegation(
  deps: Deps,
  feeder_delegation_params: FeederDelegationParams,
) -> StdResult<FeederDelegationResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::feeder_delegation(feeder_delegation_params));

  let feeder_delegation_resp: FeederDelegationResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<FeederDelegationResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => feeder_delegation_resp = response,
      };
    }
  }

  Ok(feeder_delegation_resp)
}

// query_miss_counter receives the get exchange rate base
// query params and creates an query request to the native modules
// with query_chain wrapping the response to the actual
// MissCounterResponse struct
fn query_miss_counter(
  deps: Deps,
  miss_counter_params: MissCounterParams,
) -> StdResult<MissCounterResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::miss_counter(miss_counter_params));

  let miss_counter_resp: MissCounterResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<MissCounterResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => miss_counter_resp = response,
      };
    }
  }

  Ok(miss_counter_resp)
}

// query_slash_window receives the slash window
// query params and creates an query request to the native modules
// with query_chain wrapping the response to the actual
// SlashWindowResponse struct
fn query_slash_window(
  deps: Deps,
  slash_window_params: SlashWindowParams,
) -> StdResult<SlashWindowResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::slash_window(slash_window_params));

  let slash_window_resp: SlashWindowResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<SlashWindowResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => slash_window_resp = response,
      };
    }
  }

  Ok(slash_window_resp)
}

// query_aggregate_prevote receives the get exchange rate base
// query params and creates an query request to the native modules
// with query_chain wrapping the response to the actual
// AggregatePrevoteResponse struct
fn query_aggregate_prevote(
  deps: Deps,
  aggregate_prevote_params: AggregatePrevoteParams,
) -> StdResult<AggregatePrevoteResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::aggregate_prevote(aggregate_prevote_params));

  let aggregate_prevote_resp: AggregatePrevoteResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<AggregatePrevoteResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => aggregate_prevote_resp = response,
      };
    }
  }

  Ok(aggregate_prevote_resp)
}

// query_aggregate_prevotes receives the get exchange rate base
// query params and creates an query request to the native modules
// with query_chain wrapping the response to the actual
// AggregatePrevotesResponse struct
fn query_aggregate_prevotes(
  deps: Deps,
  aggregate_prevotes_params: AggregatePrevotesParams,
) -> StdResult<AggregatePrevotesResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::aggregate_prevotes(
    aggregate_prevotes_params,
  ));

  let aggregate_prevotes_resp: AggregatePrevotesResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<AggregatePrevotesResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => aggregate_prevotes_resp = response,
      };
    }
  }

  Ok(aggregate_prevotes_resp)
}

// query_aggregate_vote receives the get exchange rate base
// query params and creates an query request to the native modules
// with query_chain wrapping the response to the actual
// AggregateVoteResponse struct
fn query_aggregate_vote(
  deps: Deps,
  aggregate_vote_params: AggregateVoteParams,
) -> StdResult<AggregateVoteResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::aggregate_vote(aggregate_vote_params));

  let aggregate_vote_resp: AggregateVoteResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<AggregateVoteResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => aggregate_vote_resp = response,
      };
    }
  }

  Ok(aggregate_vote_resp)
}

// query_aggregate_votes receives the get exchange rate base
// query params and creates an query request to the native modules
// with query_chain wrapping the response to the actual
// AggregateVotesResponse struct
fn query_aggregate_votes(
  deps: Deps,
  aggregate_votes_params: AggregateVotesParams,
) -> StdResult<AggregateVotesResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::aggregate_votes(aggregate_votes_params));

  let aggregate_votes_resp: AggregateVotesResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<AggregateVotesResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => aggregate_votes_resp = response,
      };
    }
  }

  Ok(aggregate_votes_resp)
}

// query_oracle_parameters receives the get exchange rate base
// query params and creates an query request to the native modules
// with query_chain wrapping the response to the actual
// OracleParametersResponse struct
fn query_oracle_parameters(
  deps: Deps,
  oracle_parameters_params: OracleParametersParams,
) -> StdResult<OracleParametersResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::oracle_parameters(oracle_parameters_params));

  let oracle_parameters_resp: OracleParametersResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<OracleParametersResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => oracle_parameters_resp = response,
      };
    }
  }

  Ok(oracle_parameters_resp)
}

fn query_medians(deps: Deps, medians_params: MediansParams) -> StdResult<MediansParamsResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::medians_params(medians_params));

  let medians_response: MediansParamsResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<MediansParamsResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => medians_response = response,
      };
    }
  }

  Ok(medians_response)
}

fn query_median_deviations(
  deps: Deps,
  medians_deviations_params: MedianDeviationsParams,
) -> StdResult<MedianDeviationsParamsResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::median_deviations_params(
    medians_deviations_params,
  ));

  let median_deviations_response: MedianDeviationsParamsResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<MedianDeviationsParamsResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => median_deviations_response = response,
      };
    }
  }

  Ok(median_deviations_response)
}

// -----------------------------------TESTS---------------------------------------

#[cfg(test)]
mod tests {
  use super::*;
  use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
  use cosmwasm_std::{coins, from_binary};

  #[test]
  fn proper_initialization() {
    let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(1000, "earth"));

    // we can just call .unwrap() to assert this was a success
    let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());

    // it worked, let's query the state
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
    let value: OwnerResponse = from_binary(&res).unwrap();
    assert_eq!("creator", value.owner);
  }

  #[test]
  fn change_owner() {
    let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

    let first_owner = "creator";
    let msg = InstantiateMsg {};
    let info = mock_info(first_owner, &coins(2, "token"));
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
    let value: OwnerResponse = from_binary(&res).unwrap();
    assert_eq!(first_owner, value.owner);

    let new_owner = "new_owner";

    // only the original creator can change the owner the counter
    let auth_info = mock_info(new_owner, &coins(2, "token"));
    let msg = ExecuteMsg::ChangeOwner {
      new_owner: cosmwasm_std::Addr::unchecked(new_owner),
    };
    let res = execute(deps.as_mut(), mock_env(), auth_info, msg);
    match res {
      Err(ContractError::Unauthorized {}) => {}
      _ => panic!("Must return unauthorized error"),
    }

    let auth_info = mock_info(first_owner, &coins(2, "token"));
    let msg = ExecuteMsg::ChangeOwner {
      new_owner: cosmwasm_std::Addr::unchecked(new_owner),
    };
    let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
    let value: OwnerResponse = from_binary(&res).unwrap();
    assert_eq!(new_owner, value.owner);
  }
}
