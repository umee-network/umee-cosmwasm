#[cfg(not(feature = "library"))]
use cosmwasm_std::{
  entry_point, from_binary, to_binary, to_vec, Addr, Binary, ContractResult, Deps, DepsMut, Env,
  MessageInfo, QueryRequest, Response, StdError, StdResult, SystemResult,
};
use cw2::set_contract_version;
use umee_types::{
  ActiveExchangeRatesParams, ActiveExchangeRatesResponse, AggregatePrevoteParams,
  AggregatePrevoteResponse, AggregatePrevotesParams, AggregatePrevotesResponse,
  AvailableBorrowParams, AvailableBorrowResponse, BorrowAPYParams, BorrowAPYResponse,
  BorrowLimitParams, BorrowLimitResponse, BorrowedParams, BorrowedResponse, BorrowedValueParams,
  BorrowedValueResponse, CollateralParams, CollateralResponse, CollateralValueParams,
  CollateralValueResponse, ExchangeRateParams, ExchangeRateResponse, ExchangeRatesParams,
  ExchangeRatesResponse, FeederDelegationParams, FeederDelegationResponse, LendAssetParams,
  LeverageParametersParams, LeverageParametersResponse, LiquidationTargetsParams,
  LiquidationTargetsResponse, LiquidationThresholdParams, LiquidationThresholdResponse,
  MarketSizeParams, MarketSizeResponse, MarketSummaryParams, MarketSummaryResponse,
  MissCounterParams, MissCounterResponse, RegisteredTokensParams, RegisteredTokensResponse,
  ReserveAmountParams, ReserveAmountResponse, StructUmeeMsg, StructUmeeQuery, SuppliedParams,
  SuppliedResponse, SuppliedValueParams, SuppliedValueResponse, SupplyAPYParams, SupplyAPYResponse,
  TokenMarketSizeParams, TokenMarketSizeResponse, TotalBorrowedParams, TotalBorrowedResponse,
  TotalCollateralParams, TotalCollateralResponse, UmeeMsg, UmeeMsgLeverage, UmeeQuery,
  UmeeQueryLeverage, UmeeQueryOracle,
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
    ExecuteMsg::Chain(cosmos_umee_msg) => msg_chain(cosmos_umee_msg),
    ExecuteMsg::Umee(UmeeMsg::Leverage(execute_leverage_msg)) => {
      execute_leverage(execute_leverage_msg)
    }
    ExecuteMsg::LendAsset(lend_asset_params) => execute_lend(lend_asset_params),
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
    UmeeMsgLeverage::LendAsset(lend_asset_params) => execute_lend(lend_asset_params),
    UmeeMsgLeverage::WithdrawAsset(withdraw_asset_params) => {
      execute_withdraw(withdraw_asset_params)
    }
  }
}

// execute_lend sends umee leverage module an message of LendAsset
fn execute_lend(
  lend_asset_params: LendAssetParams,
) -> Result<Response<StructUmeeMsg>, ContractError> {
  Ok(
    Response::new()
      .add_attribute("method", "lend_asset")
      .add_message(StructUmeeMsg::lend_asset(lend_asset_params)),
  )
}

// execute_withdraw sends umee leverage module an message of WithdrawAsset
fn execute_withdraw(
  withdraw_asset_params: LendAssetParams,
) -> Result<Response<StructUmeeMsg>, ContractError> {
  Ok(
    Response::new()
      .add_attribute("method", "lend_asset")
      .add_message(StructUmeeMsg::withdraw_asset(withdraw_asset_params)),
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
    QueryMsg::Umee(UmeeQuery::Leverage(leverage)) => query_leverage(deps, _env, leverage),

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
    QueryMsg::Umee(UmeeQuery::Oracle(oracle)) => query_oracle(deps, _env, oracle),

    // consumes the query_chain wrapping the JSON to call directly
    // the Borrowed query from the leverage umee native module
    // expected json input:
    // {
    //   "borrowed": {
    //     "address": "umee1y6xz2ggfc0pcsmyjlekh0j9pxh6hk87ymc9due",
    //     "denom": "uumee"
    //   }
    // }
    // successful json output:
    // {
    //   "data": {
    //     "borrowed_amount": {
    //       "denom": "uumee",
    //       "amount": "50001"
    //     }
    //   }
    // }
    QueryMsg::Borrowed(borrowed_params) => to_binary(&query_borrowed(deps, borrowed_params)?),

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
    QueryMsg::BorrowedValue(borrowed_value_params) => {
      to_binary(&query_borrowed_value(deps, borrowed_value_params)?)
    }
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
    // consumes the query_chain wrapped by Umee Leverage enums
    // to clarift the JSON queries to umee leverage native module
    // example json input:
    // {
    //   "umee": {
    //     "leverage": {
    //       "borrowed": {
    //         "address": "umee1y6xz2ggfc0pcsmyjlekh0j9pxh6hk87ymc9due",
    //         "denom": "uumee"
    //       }
    //     }
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
    UmeeQueryLeverage::Borrowed(borrowed_params) => {
      to_binary(&query_borrowed(deps, borrowed_params)?)
    }
    UmeeQueryLeverage::RegisteredTokens(registered_tokens_params) => {
      to_binary(&query_registered_tokens(deps, registered_tokens_params)?)
    }
    UmeeQueryLeverage::LeverageParameters(leverage_parameters_params) => to_binary(
      &query_leverage_parameters(deps, leverage_parameters_params)?,
    ),
    UmeeQueryLeverage::BorrowedValue(borrowed_params) => {
      to_binary(&query_borrowed_value(deps, borrowed_params)?)
    }
    UmeeQueryLeverage::Supplied(supplied_params) => {
      to_binary(&query_supplied(deps, supplied_params)?)
    }
    UmeeQueryLeverage::SuppliedValue(supplied_value_params) => {
      to_binary(&query_supplied_value(deps, supplied_value_params)?)
    }
    UmeeQueryLeverage::AvailableBorrow(available_borrow_params) => {
      to_binary(&query_available_borrow(deps, available_borrow_params)?)
    }
    UmeeQueryLeverage::BorrowAPY(borrow_apy_params) => {
      to_binary(&query_borrow_apy(deps, borrow_apy_params)?)
    }
    UmeeQueryLeverage::SupplyAPY(supply_apy_params) => {
      to_binary(&query_supply_apy(deps, supply_apy_params)?)
    }
    UmeeQueryLeverage::MarketSize(market_size_params) => {
      to_binary(&query_market_size(deps, market_size_params)?)
    }
    UmeeQueryLeverage::TokenMarketSize(token_market_size_params) => {
      to_binary(&query_token_market_size(deps, token_market_size_params)?)
    }
    UmeeQueryLeverage::ReserveAmount(reserve_amount_params) => {
      to_binary(&query_reserve_amount(deps, reserve_amount_params)?)
    }
    UmeeQueryLeverage::Collateral(collateral_params) => {
      to_binary(&query_collateral(deps, collateral_params)?)
    }
    UmeeQueryLeverage::CollateralValue(collateral_value_params) => {
      to_binary(&query_collateral_value(deps, collateral_value_params)?)
    }
    UmeeQueryLeverage::ExchangeRate(exchange_rate_params) => {
      to_binary(&query_exchange_rate(deps, exchange_rate_params)?)
    }
    UmeeQueryLeverage::BorrowLimit(borrow_limit_params) => {
      to_binary(&query_borrow_limit(deps, borrow_limit_params)?)
    }
    UmeeQueryLeverage::LiquidationThreshold(liquidation_threshold_params) => to_binary(
      &query_liquidation_threshold(deps, liquidation_threshold_params)?,
    ),
    UmeeQueryLeverage::LiquidationTargets(liquidation_targets_params) => to_binary(
      &query_liquidation_targets(deps, liquidation_targets_params)?,
    ),
    UmeeQueryLeverage::MarketSummary(market_summary_params) => {
      to_binary(&query_market_summary(deps, market_summary_params)?)
    }
    UmeeQueryLeverage::TotalCollateral(total_collateral_params) => {
      to_binary(&query_total_collateral(deps, total_collateral_params)?)
    }
    UmeeQueryLeverage::TotalBorrowed(total_borrowed_params) => {
      to_binary(&query_total_borrowed(deps, total_borrowed_params)?)
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
    UmeeQueryOracle::AggregatePrevote(aggregate_prevote_params) => {
      to_binary(&query_aggregate_prevote(deps, aggregate_prevote_params)?)
    }
    UmeeQueryOracle::AggregatePrevotes(aggregate_prevotes_params) => {
      to_binary(&query_aggregate_prevotes(deps, aggregate_prevotes_params)?)
    }
  }
}

// query_borrowed receives the get borrow query params and creates
// an query request to the native modules with query_chain wrapping
// the response to the actual BorrowedResponse struct
fn query_borrowed(deps: Deps, borrowed_params: BorrowedParams) -> StdResult<BorrowedResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::borrowed(borrowed_params));

  let borrowed_response: BorrowedResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<BorrowedResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => borrowed_response = response,
      };
    }
  }

  Ok(borrowed_response)
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

// query_borrowed_value creates an query request to the native modules
// with query_chain wrapping the response to the actual
// BorrowedValueResponse struct
fn query_borrowed_value(
  deps: Deps,
  borrowed_value_params: BorrowedValueParams,
) -> StdResult<BorrowedValueResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::borrowed_value(borrowed_value_params));

  let borrowed_value_response: BorrowedValueResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<BorrowedValueResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => borrowed_value_response = response,
      };
    }
  }

  Ok(borrowed_value_response)
}

// query_supplied creates an query request to the native modules
// with query_chain wrapping the response to the actual
// SuppliedResponse struct
fn query_supplied(deps: Deps, supplied_params: SuppliedParams) -> StdResult<SuppliedResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::supplied(supplied_params));

  let supplied_response: SuppliedResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<SuppliedResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => supplied_response = response,
      };
    }
  }

  Ok(supplied_response)
}

// query_supplied_value creates an query request to the native modules
// with query_chain wrapping the response to the actual
// SuppliedValueResponse struct.
fn query_supplied_value(
  deps: Deps,
  supplied_value_params: SuppliedValueParams,
) -> StdResult<SuppliedValueResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::supplied_value(supplied_value_params));

  let supplied_value_response: SuppliedValueResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<SuppliedValueResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => supplied_value_response = response,
      };
    }
  }

  Ok(supplied_value_response)
}

// query_available_borrow creates an query request to the native modules
// with query_chain wrapping the response to the actual
// AvailableBorrowResponse struct.
fn query_available_borrow(
  deps: Deps,
  available_borrow_params: AvailableBorrowParams,
) -> StdResult<AvailableBorrowResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::available_borrow(available_borrow_params));

  let available_borrow_response: AvailableBorrowResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<AvailableBorrowResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => available_borrow_response = response,
      };
    }
  }

  Ok(available_borrow_response)
}

// query_borrow_apy creates an query request to the native modules
// with query_chain wrapping the response to the actual
// BorrowAPYResponse struct.
fn query_borrow_apy(
  deps: Deps,
  borrow_apy_params: BorrowAPYParams,
) -> StdResult<BorrowAPYResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::borrow_apy(borrow_apy_params));

  let borrow_apy_response: BorrowAPYResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<BorrowAPYResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => borrow_apy_response = response,
      };
    }
  }

  Ok(borrow_apy_response)
}

// query_supply_apy creates an query request to the native modules
// with query_chain wrapping the response to the actual
// SupplyAPYResponse struct.
fn query_supply_apy(
  deps: Deps,
  supply_apy_params: SupplyAPYParams,
) -> StdResult<SupplyAPYResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::supply_apy(supply_apy_params));

  let supply_apy_response: SupplyAPYResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<SupplyAPYResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => supply_apy_response = response,
      };
    }
  }

  Ok(supply_apy_response)
}

// query_market_size creates an query request to the native modules
// with query_chain wrapping the response to the actual
// MarketSizeResponse struct.
fn query_market_size(
  deps: Deps,
  market_size_params: MarketSizeParams,
) -> StdResult<MarketSizeResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::market_size(market_size_params));

  let market_size_response: MarketSizeResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<MarketSizeResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => market_size_response = response,
      };
    }
  }

  Ok(market_size_response)
}

// query_token_market_size creates an query request to the native modules
// with query_chain wrapping the response to the actual
// TokenMarketSizeResponse struct.
fn query_token_market_size(
  deps: Deps,
  token_market_size_params: TokenMarketSizeParams,
) -> StdResult<TokenMarketSizeResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::token_market_size(token_market_size_params));

  let market_size_response: TokenMarketSizeResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<TokenMarketSizeResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => market_size_response = response,
      };
    }
  }

  Ok(market_size_response)
}

// query_reserve_amount creates an query request to the native modules
// with query_chain wrapping the response to the actual
// ReserveAmountResponse struct.
fn query_reserve_amount(
  deps: Deps,
  reserve_amount_params: ReserveAmountParams,
) -> StdResult<ReserveAmountResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::reserve_amount(reserve_amount_params));

  let reserve_amount_response: ReserveAmountResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<ReserveAmountResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => reserve_amount_response = response,
      };
    }
  }

  Ok(reserve_amount_response)
}

// query_collateral creates an query request to the native modules
// with query_chain wrapping the response to the actual
// CollateralResponse struct.
fn query_collateral(
  deps: Deps,
  collateral_params: CollateralParams,
) -> StdResult<CollateralResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::collateral(collateral_params));

  let collateral_response: CollateralResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<CollateralResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => collateral_response = response,
      };
    }
  }

  Ok(collateral_response)
}

// query_collateral_value creates an query request to the native modules
// with query_chain wrapping the response to the actual
// CollateralValueResponse struct.
fn query_collateral_value(
  deps: Deps,
  collateral_value_params: CollateralValueParams,
) -> StdResult<CollateralValueResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::collateral_value(collateral_value_params));

  let collateral_response: CollateralValueResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<CollateralValueResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => collateral_response = response,
      };
    }
  }

  Ok(collateral_response)
}

// query_exchange_rate creates an query request to the native modules
// with query_chain wrapping the response to the actual
// ExchangeRateResponse struct.
fn query_exchange_rate(
  deps: Deps,
  exchange_rate_params: ExchangeRateParams,
) -> StdResult<ExchangeRateResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::exchange_rate(exchange_rate_params));

  let exchange_rate_response: ExchangeRateResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<ExchangeRateResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => exchange_rate_response = response,
      };
    }
  }

  Ok(exchange_rate_response)
}

// query_borrow_limit creates an query request to the native modules
// with query_chain wrapping the response to the actual
// BorrowLimitResponse struct.
fn query_borrow_limit(
  deps: Deps,
  borrow_limit_params: BorrowLimitParams,
) -> StdResult<BorrowLimitResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::borrow_limit(borrow_limit_params));

  let borrow_limit_response: BorrowLimitResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<BorrowLimitResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => borrow_limit_response = response,
      };
    }
  }

  Ok(borrow_limit_response)
}

// query_liquidation_threshold creates an query request to the native modules
// with query_chain wrapping the response to the actual
// LiquidationThresholdResponse struct.
fn query_liquidation_threshold(
  deps: Deps,
  liquidation_threshold_params: LiquidationThresholdParams,
) -> StdResult<LiquidationThresholdResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::liquidation_threshold(
    liquidation_threshold_params,
  ));

  let liquidation_threshold_response: LiquidationThresholdResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<LiquidationThresholdResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => liquidation_threshold_response = response,
      };
    }
  }

  Ok(liquidation_threshold_response)
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

// query_total_collateral creates an query request to the native modules
// with query_chain wrapping the response to the actual
// TotalCollateralResponse struct.
fn query_total_collateral(
  deps: Deps,
  total_collateral_params: TotalCollateralParams,
) -> StdResult<TotalCollateralResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::total_collateral(total_collateral_params));

  let total_collateral_response: TotalCollateralResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<TotalCollateralResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => total_collateral_response = response,
      };
    }
  }

  Ok(total_collateral_response)
}

// query_total_borrowed creates an query request to the native modules
// with query_chain wrapping the response to the actual
// TotalBorrowedResponse struct.
fn query_total_borrowed(
  deps: Deps,
  total_borrowed_params: TotalBorrowedParams,
) -> StdResult<TotalBorrowedResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::total_borrowed(total_borrowed_params));

  let total_borrowed_response: TotalBorrowedResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<TotalBorrowedResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => total_borrowed_response = response,
      };
    }
  }

  Ok(total_borrowed_response)
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
