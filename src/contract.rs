#[cfg(not(feature = "library"))]
use cosmwasm_std::{
  entry_point, Addr, Binary, ContractResult, Deps, DepsMut, Env,
  MessageInfo, QueryRequest, Response, StdError, StdResult, SystemResult,
};
use cosmwasm_std::{from_json, to_json_binary, to_json_vec};
use cw2::set_contract_version;
use cw_umee_types::error::ContractError;
use cw_umee_types::query_incentive::{
  AccountBondsParams, AccountBondsResponse, ActualRatesParams, ActualRatesResponse,
  CompletedIncentiveProgramsParams, CompletedIncentiveProgramsResponse, CurrentRatesParams,
  CurrentRatesResponse, IncentiveParametersParams, IncentiveParametersResponse,
  IncentiveProgramParams, IncentiveProgramResponse, LastRewardTimeParams, LastRewardTimeResponse,
  OngoingIncentiveProgramsParams, OngoingIncentiveProgramsResponse, PendingRewardsParams,
  PendingRewardsResponse, TotalBondedParams, TotalBondedResponse, TotalUnbondingParams,
  TotalUnbondingResponse, UpcomingIncentiveProgramsParams, UpcomingIncentiveProgramsResponse,
};
use cw_umee_types::query_leverage::{
  BadDebtsParams, BadDebtsResponse, MaxBorrowParams, MaxBorrowResponse, MaxWithdrawParams,
  MaxWithdrawResponse,
};
use cw_umee_types::query_metoken::{
  MetokenIndexPricesParams, MetokenIndexPricesResponse, MetokenIndexbalancesParams,
  MetokenIndexbalancesResponse, MetokenIndexesParams, MetokenIndexesResponse,
  MetokenParametersParams, MetokenParametersResponse, MetokenRedeemfeeParams,
  MetokenRedeemfeeResponse, MetokenSwapfeeParams, MetokenSwapfeeResponse, UmeeQueryMeToken,
};
use cw_umee_types::query_oracle::{
  MedianDeviationsParams, MedianDeviationsParamsResponse, MediansParams, MediansParamsResponse,
};
use cw_umee_types::{
  AccountBalancesParams, AccountBalancesResponse, AccountSummaryParams, ActiveExchangeRatesParams,
  ActiveExchangeRatesResponse, AggregatePrevoteParams, AggregatePrevoteResponse,
  AggregatePrevotesParams, AggregatePrevotesResponse, AggregateVoteParams, AggregateVoteResponse,
  AggregateVotesParams, AggregateVotesResponse, ExchangeRatesParams, ExchangeRatesResponse,
  FeederDelegationParams, FeederDelegationResponse, LeverageParametersParams,
  LeverageParametersResponse, LiquidationTargetsParams, LiquidationTargetsResponse,
  MarketSummaryParams, MarketSummaryResponse, MissCounterParams, MissCounterResponse,
  OracleParametersParams, OracleParametersResponse, RegisteredTokensParams,
  RegisteredTokensResponse, SlashWindowParams, SlashWindowResponse, StructUmeeMsg, StructUmeeQuery,
  UmeeMsg, UmeeMsgLeverage, UmeeQuery, UmeeQueryIncentive, UmeeQueryLeverage, UmeeQueryOracle,
};

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
    ExecuteMsg::Umee(UmeeMsg::Leverage(execute_leverage_msg)) => {
      execute_leverage(execute_leverage_msg)
    }
    ExecuteMsg::Supply(supply_params) => StructUmeeMsg::supply(supply_params),
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

// execute_leverage handles the execution of every msg of leverage umee native modules
fn execute_leverage(
  execute_leverage_msg: UmeeMsgLeverage,
) -> Result<Response<StructUmeeMsg>, ContractError> {
  match execute_leverage_msg {
    UmeeMsgLeverage::Supply(supply_params) => StructUmeeMsg::supply(supply_params),
    UmeeMsgLeverage::Withdraw(withdraw_params) => StructUmeeMsg::withdraw(withdraw_params),
    UmeeMsgLeverage::MaxWithdraw(max_withdraw_params) => {
      StructUmeeMsg::max_withdraw(max_withdraw_params)
    }
    UmeeMsgLeverage::Collateralize(collateralize_params) => {
      StructUmeeMsg::collateralize(collateralize_params)
    }
    UmeeMsgLeverage::Decollateralize(decollateralize_params) => {
      StructUmeeMsg::decollateralize(decollateralize_params)
    }
    UmeeMsgLeverage::Borrow(borrow_params) => StructUmeeMsg::borrow(borrow_params),
    UmeeMsgLeverage::MaxBorrow(borrow_params) => StructUmeeMsg::max_borrow(borrow_params),
    UmeeMsgLeverage::Repay(repay_params) => StructUmeeMsg::repay(repay_params),
    UmeeMsgLeverage::Liquidate(liquidate_params) => StructUmeeMsg::liquidate(liquidate_params),
    UmeeMsgLeverage::SupplyCollateral(supply_collateralize_params) => {
      StructUmeeMsg::supply_collateral(supply_collateralize_params)
    }
  }
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
    QueryMsg::GetOwner {} => to_json_binary(&query_owner(deps)?),

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
      to_json_binary(&query_exchange_rates(deps, exchange_rates_params)?)
    }
    QueryMsg::RegisteredTokens(registered_tokens_params) => {
      to_json_binary(&query_registered_tokens(deps, registered_tokens_params)?)
    }
    QueryMsg::LeverageParameters(leverage_parameters_params) => to_json_binary(
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
    // incentive
    UmeeQuery::Incentive(incentive) => query_incentive(deps, _env, incentive),
    UmeeQuery::Metoken(metoken) => query_metoken(deps, _env, metoken),
  }
}

// returns the current owner of the contract from the state
fn query_owner(deps: Deps) -> StdResult<OwnerResponse> {
  let state = STATE.load(deps.storage)?;
  Ok(OwnerResponse { owner: state.owner })
}

// query_chain queries for any availabe query in the chain native modules
fn query_chain(deps: Deps, request: &QueryRequest<StructUmeeQuery>) -> StdResult<Binary> {
  let raw = to_json_vec(request).map_err(|serialize_err| {
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
    UmeeQueryLeverage::LeverageParameters(leverage_parameters_params) => to_json_binary(
      &query_leverage_parameters(deps, leverage_parameters_params)?,
    ),
    UmeeQueryLeverage::RegisteredTokens(registered_tokens_params) => {
      to_json_binary(&query_registered_tokens(deps, registered_tokens_params)?)
    }
    UmeeQueryLeverage::MarketSummary(market_summary_params) => {
      to_json_binary(&query_market_summary(deps, market_summary_params)?)
    }
    UmeeQueryLeverage::AccountBalances(account_balances_params) => {
      to_json_binary(&query_account_balances(deps, account_balances_params)?)
    }
    UmeeQueryLeverage::AccountSummary(account_summary_params) => {
      to_json_binary(&query_account_summary(deps, account_summary_params)?)
    }
    UmeeQueryLeverage::LiquidationTargets(liquidation_targets_params) => to_json_binary(
      &query_liquidation_targets(deps, liquidation_targets_params)?,
    ),
    UmeeQueryLeverage::BadDebts(bad_debts_params) => {
      to_json_binary(&query_bad_debts(deps, bad_debts_params)?)
    }
    UmeeQueryLeverage::MaxWithdraw(max_withdraw_params) => {
      to_json_binary(&query_max_withdraw(deps, max_withdraw_params)?)
    }
    UmeeQueryLeverage::MaxBorrow(max_borrow_params) => {
      to_json_binary(&query_max_borrow(deps, max_borrow_params)?)
    }
  }
}

// query_incentive
fn query_incentive(deps: Deps, _env: Env, msg: UmeeQueryIncentive) -> StdResult<Binary> {
  match msg {
    UmeeQueryIncentive::IncentiveParameters(incentive_params) => {
      to_json_binary(&query_incentive_params(deps, incentive_params)?)
    }
    UmeeQueryIncentive::TotalBonded(params) => to_json_binary(&query_total_bonded(deps, params)?),
    UmeeQueryIncentive::TotalUnbonding(params) => {
      to_json_binary(&query_total_unbonding(deps, params)?)
    }
    UmeeQueryIncentive::AccountBonds(params) => to_json_binary(&query_account_bonds(deps, params)?),
    UmeeQueryIncentive::PendingRewards(params) => {
      to_json_binary(&query_pending_rewards(deps, params)?)
    }
    UmeeQueryIncentive::CompletedIncentivePrograms(params) => {
      to_json_binary(&query_completed_incentive_programs(deps, params)?)
    }
    UmeeQueryIncentive::OngoingIncentivePrograms(params) => {
      to_json_binary(&query_ongoing_incentive_programs(deps, params)?)
    }
    UmeeQueryIncentive::UpcomingIncentivePrograms(params) => {
      to_json_binary(&query_upcoming_incentive_programs(deps, params)?)
    }
    UmeeQueryIncentive::IncentiveProgram(params) => {
      to_json_binary(&query_incentive_program(deps, params)?)
    }
    UmeeQueryIncentive::CurrentRates(params) => to_json_binary(&query_current_rates(deps, params)?),
    UmeeQueryIncentive::ActualRates(params) => to_json_binary(&query_actutal_rates(deps, params)?),
    UmeeQueryIncentive::LastRewardTime(params) => {
      to_json_binary(&query_last_reward_time(deps, params)?)
    }
  }
}

// query_metoken
fn query_metoken(deps: Deps, _env: Env, msg: UmeeQueryMeToken) -> StdResult<Binary> {
  match msg {
    UmeeQueryMeToken::MetokenParameters(params) => {
      to_json_binary(&query_metoken_params(deps, params)?)
    }
    UmeeQueryMeToken::MetokenIndexes(params) => {
      to_json_binary(&query_metoken_indexes(deps, params)?)
    }
    UmeeQueryMeToken::MetokenSwapfee(params) => {
      to_json_binary(&query_metoken_swapfee(deps, params)?)
    }
    UmeeQueryMeToken::MetokenRedeemfee(params) => {
      to_json_binary(&query_metoken_redeemfee(deps, params)?)
    }
    UmeeQueryMeToken::MetokenIndexbalances(params) => {
      to_json_binary(&query_metoken_indexbalances(deps, params)?)
    }
    UmeeQueryMeToken::MetokenIndexPrices(params) => {
      to_json_binary(&query_metoken_indexprice(deps, params)?)
    }
  }
}

// query_metoken_indexprice
fn query_metoken_indexprice(
  deps: Deps,
  params: MetokenIndexPricesParams,
) -> StdResult<MetokenIndexPricesResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::metoken_indexprice(params));
  let response: MetokenIndexPricesResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_json::<MetokenIndexPricesResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(resp) => response = resp,
      };
    }
  }

  Ok(response)
}

// query_metoken_indexbalances
fn query_metoken_indexbalances(
  deps: Deps,
  params: MetokenIndexbalancesParams,
) -> StdResult<MetokenIndexbalancesResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::metoken_indexbalances(params));
  let response: MetokenIndexbalancesResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_json::<MetokenIndexbalancesResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(resp) => response = resp,
      };
    }
  }

  Ok(response)
}

// query_metoken_redeemfee
fn query_metoken_redeemfee(
  deps: Deps,
  params: MetokenRedeemfeeParams,
) -> StdResult<MetokenRedeemfeeResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::metoken_redeemfee(params));
  let response: MetokenRedeemfeeResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_json::<MetokenRedeemfeeResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(resp) => response = resp,
      };
    }
  }

  Ok(response)
}

// query_metoken_swapfee
fn query_metoken_swapfee(
  deps: Deps,
  params: MetokenSwapfeeParams,
) -> StdResult<MetokenSwapfeeResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::metoken_swapfee(params));
  let response: MetokenSwapfeeResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_json::<MetokenSwapfeeResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(resp) => response = resp,
      };
    }
  }

  Ok(response)
}

// query_metoken_indexes
fn query_metoken_indexes(
  deps: Deps,
  params: MetokenIndexesParams,
) -> StdResult<MetokenIndexesResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::metoken_indexes(params));
  let response: MetokenIndexesResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_json::<MetokenIndexesResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(resp) => response = resp,
      };
    }
  }

  Ok(response)
}

// query_metoken_params
fn query_metoken_params(
  deps: Deps,
  params: MetokenParametersParams,
) -> StdResult<MetokenParametersResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::metoken_parameters(params));
  let response: MetokenParametersResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_json::<MetokenParametersResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(resp) => response = resp,
      };
    }
  }

  Ok(response)
}

// query_last_reward_time
fn query_last_reward_time(
  deps: Deps,
  params: LastRewardTimeParams,
) -> StdResult<LastRewardTimeResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::last_reward_time(params));

  let response: LastRewardTimeResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_json::<LastRewardTimeResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(resp) => response = resp,
      };
    }
  }

  Ok(response)
}

// query_actutal_rates
fn query_actutal_rates(deps: Deps, params: ActualRatesParams) -> StdResult<ActualRatesResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::actual_rates(params));

  let response: ActualRatesResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_json::<ActualRatesResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(resp) => response = resp,
      };
    }
  }

  Ok(response)
}

// query_current_rates
fn query_current_rates(deps: Deps, params: CurrentRatesParams) -> StdResult<CurrentRatesResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::current_rates(params));

  let response: CurrentRatesResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_json::<CurrentRatesResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(resp) => response = resp,
      };
    }
  }

  Ok(response)
}

// query_incentive_program
fn query_incentive_program(
  deps: Deps,
  params: IncentiveProgramParams,
) -> StdResult<IncentiveProgramResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::incentive_program(params));

  let response: IncentiveProgramResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_json::<IncentiveProgramResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(resp) => response = resp,
      };
    }
  }

  Ok(response)
}

// query_upcoming_incentive_programs
fn query_upcoming_incentive_programs(
  deps: Deps,
  params: UpcomingIncentiveProgramsParams,
) -> StdResult<UpcomingIncentiveProgramsResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::upcoming_incentive_programs(params));

  let response: UpcomingIncentiveProgramsResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_json::<UpcomingIncentiveProgramsResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(resp) => response = resp,
      };
    }
  }

  Ok(response)
}

// query_ongoing_incentive_programs
fn query_ongoing_incentive_programs(
  deps: Deps,
  params: OngoingIncentiveProgramsParams,
) -> StdResult<OngoingIncentiveProgramsResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::ongoing_incentive_programs(params));

  let response: OngoingIncentiveProgramsResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_json::<OngoingIncentiveProgramsResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(resp) => response = resp,
      };
    }
  }

  Ok(response)
}

// query_completed_incentive_programs
fn query_completed_incentive_programs(
  deps: Deps,
  params: CompletedIncentiveProgramsParams,
) -> StdResult<CompletedIncentiveProgramsResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::completed_incentive_programs(params));

  let response: CompletedIncentiveProgramsResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_json::<CompletedIncentiveProgramsResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(resp) => response = resp,
      };
    }
  }

  Ok(response)
}

// query_account_bonds
fn query_pending_rewards(
  deps: Deps,
  params: PendingRewardsParams,
) -> StdResult<PendingRewardsResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::pending_rewards(params));

  let response: PendingRewardsResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_json::<PendingRewardsResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(resp) => response = resp,
      };
    }
  }

  Ok(response)
}

// query_account_bonds
fn query_account_bonds(deps: Deps, params: AccountBondsParams) -> StdResult<AccountBondsResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::account_bonds(params));

  let response: AccountBondsResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_json::<AccountBondsResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(resp) => response = resp,
      };
    }
  }

  Ok(response)
}

// query_total_unbonding
fn query_total_unbonding(
  deps: Deps,
  params: TotalUnbondingParams,
) -> StdResult<TotalUnbondingResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::total_unbonding(params));

  let response: TotalUnbondingResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_json::<TotalUnbondingResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(resp) => response = resp,
      };
    }
  }

  Ok(response)
}

// query_total_bonded
fn query_total_bonded(deps: Deps, params: TotalBondedParams) -> StdResult<TotalBondedResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::total_bonded(params));

  let response: TotalBondedResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_json::<TotalBondedResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(resp) => response = resp,
      };
    }
  }

  Ok(response)
}

// query_incentive_params
fn query_incentive_params(
  deps: Deps,
  incentive_params: IncentiveParametersParams,
) -> StdResult<IncentiveParametersResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::incentive_params(incentive_params));

  let incentive_params_response: IncentiveParametersResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_json::<IncentiveParametersResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => incentive_params_response = response,
      };
    }
  }

  Ok(incentive_params_response)
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
      to_json_binary(&query_exchange_rates(deps, exchange_rates_params)?)
    }
    UmeeQueryOracle::ActiveExchangeRates(active_exchange_rates_params) => to_json_binary(
      &query_active_exchange_rates(deps, active_exchange_rates_params)?,
    ),
    UmeeQueryOracle::FeederDelegation(feeder_delegation_params) => {
      to_json_binary(&query_feeder_delegation(deps, feeder_delegation_params)?)
    }
    UmeeQueryOracle::MissCounter(miss_counter_params) => {
      to_json_binary(&query_miss_counter(deps, miss_counter_params)?)
    }
    UmeeQueryOracle::SlashWindow(slash_window_params) => {
      to_json_binary(&query_slash_window(deps, slash_window_params)?)
    }
    UmeeQueryOracle::AggregatePrevote(aggregate_prevote_params) => {
      to_json_binary(&query_aggregate_prevote(deps, aggregate_prevote_params)?)
    }
    UmeeQueryOracle::AggregatePrevotes(aggregate_prevotes_params) => {
      to_json_binary(&query_aggregate_prevotes(deps, aggregate_prevotes_params)?)
    }
    UmeeQueryOracle::AggregateVote(aggregate_vote_params) => {
      to_json_binary(&query_aggregate_vote(deps, aggregate_vote_params)?)
    }
    UmeeQueryOracle::AggregateVotes(aggregate_votes_params) => {
      to_json_binary(&query_aggregate_votes(deps, aggregate_votes_params)?)
    }
    UmeeQueryOracle::OracleParameters(oracle_parameters_params) => {
      to_json_binary(&query_oracle_parameters(deps, oracle_parameters_params)?)
    }
    UmeeQueryOracle::Medians(median_params) => to_json_binary(&query_medians(deps, median_params)?),
    UmeeQueryOracle::MedianDeviations(median_deviations_params) => {
      to_json_binary(&query_median_deviations(deps, median_deviations_params)?)
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
      match from_json::<RegisteredTokensResponse>(&binary) {
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
      match from_json::<LeverageParametersResponse>(&binary) {
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
      match from_json::<AccountBalancesResponse>(&binary) {
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
      match from_json::<AccountSummaryParams>(&binary) {
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
      match from_json::<LiquidationTargetsResponse>(&binary) {
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
      match from_json::<BadDebtsResponse>(&binary) {
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
      match from_json::<MaxWithdrawResponse>(&binary) {
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
      match from_json::<MaxBorrowResponse>(&binary) {
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
      match from_json::<MarketSummaryResponse>(&binary) {
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
      match from_json::<ExchangeRatesResponse>(&binary) {
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
      match from_json::<ActiveExchangeRatesResponse>(&binary) {
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
      match from_json::<FeederDelegationResponse>(&binary) {
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
      match from_json::<MissCounterResponse>(&binary) {
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
      match from_json::<SlashWindowResponse>(&binary) {
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
      match from_json::<AggregatePrevoteResponse>(&binary) {
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
      match from_json::<AggregatePrevotesResponse>(&binary) {
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
      match from_json::<AggregateVoteResponse>(&binary) {
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
      match from_json::<AggregateVotesResponse>(&binary) {
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
      match from_json::<OracleParametersResponse>(&binary) {
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
      match from_json::<MediansParamsResponse>(&binary) {
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
      match from_json::<MedianDeviationsParamsResponse>(&binary) {
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
