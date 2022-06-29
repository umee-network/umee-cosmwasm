#[cfg(not(feature = "library"))]
use cosmwasm_std::{
  entry_point, from_binary, to_binary, to_vec, Addr, Binary, ContractResult, Deps, DepsMut, Env,
  MessageInfo, QueryRequest, Response, StdError, StdResult, SystemResult,
};
use cw2::set_contract_version;
use umee_types::{
  BorrowParams, BorrowResponse, ExchangeRateBaseParams, ExchangeRateBaseResponse,
  LeverageParametersParams, LeverageParametersResponse, RegisteredTokensParams,
  RegisteredTokensResponse, StructUmeeQuery, UmeeQuery, UmeeQueryLeverage, UmeeQueryOracle,
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
) -> Result<Response, ContractError> {
  match msg {
    // receives the new owner and tries to change it in the contract state
    ExecuteMsg::ChangeOwner { new_owner } => try_change_owner(deps, info, new_owner),
  }
}

// tries to change the owner, but it could fail and respond as Unauthorized
pub fn try_change_owner(
  deps: DepsMut,
  info: MessageInfo,
  new_owner: Addr,
) -> Result<Response, ContractError> {
  STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
    if info.sender != state.owner {
      return Err(ContractError::Unauthorized {});
    }
    state.owner = new_owner;
    Ok(state)
  })?;
  Ok(Response::new().add_attribute("method", "change_owner"))
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
    // the GetBorrow query from the leverage umee native module
    // expected json input:
    // {
    //   "get_borrow": {
    //     "borrower_addr": "umee1y6xz2ggfc0pcsmyjlekh0j9pxh6hk87ymc9due",
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
    QueryMsg::GetBorrow(borrow_params) => to_binary(&query_get_borrow(deps, borrow_params)?),

    // consumes the query_chain wrapping the JSON to call directly
    // the GetExchangeRateBase query from the oracle umee native module
    // expected json input:
    // {
    //   "get_exchange_rate_base": {
    //     "denom": "uumee"
    //   }
    // }
    // successful json output:
    // {
    //   "data": {
    //     "exchange_rate_base": "0.0000032"
    //   }
    // }
    QueryMsg::GetExchangeRateBase(exchange_rate_base_params) => to_binary(
      &query_get_exchange_rate_base(deps, exchange_rate_base_params)?,
    ),
    QueryMsg::RegisteredTokens(registered_tokens_params) => {
      to_binary(&query_registered_tokens(deps, registered_tokens_params)?)
    }
    QueryMsg::LeverageParameters(leverage_parameters_params) => to_binary(
      &query_leverage_parameters(deps, leverage_parameters_params)?,
    ),
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
    //       "get_borrow": {
    //         "borrower_addr": "umee1y6xz2ggfc0pcsmyjlekh0j9pxh6hk87ymc9due",
    //         "denom": "uumee"
    //       }
    //     }
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
    UmeeQueryLeverage::GetBorrow(borrow_params) => {
      to_binary(&query_get_borrow(deps, borrow_params)?)
    }
    UmeeQueryLeverage::RegisteredTokens(registered_tokens_params) => {
      to_binary(&query_registered_tokens(deps, registered_tokens_params)?)
    }
    UmeeQueryLeverage::LeverageParameters(leverage_parameters_params) => to_binary(
      &query_leverage_parameters(deps, leverage_parameters_params)?,
    ),
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
    //       "get_exchange_rate_base": {
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
    UmeeQueryOracle::GetExchangeRateBase(exchange_rate_base_params) => to_binary(
      &query_get_exchange_rate_base(deps, exchange_rate_base_params)?,
    ),
  }
}

// query_get_borrow receives the get borrow query params and creates
// an query request to the native modules with query_chain wrapping
// the response to the actual BorrowResponse struct
fn query_get_borrow(deps: Deps, borrow_params: BorrowParams) -> StdResult<BorrowResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::get_borrow(borrow_params));

  let borrow_response: BorrowResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<BorrowResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => borrow_response = response,
      };
    }
  }

  Ok(borrow_response)
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

// query_get_exchange_rate_base receives the get exchange rate base
// query params and creates an query request to the native modules
// with query_chain wrapping the response to the actual
// ExchangeRateBaseResponse struct
fn query_get_exchange_rate_base(
  deps: Deps,
  exchange_rate_base_params: ExchangeRateBaseParams,
) -> StdResult<ExchangeRateBaseResponse> {
  let request = QueryRequest::Custom(StructUmeeQuery::get_exchange_rate_base(
    exchange_rate_base_params,
  ));

  let exchange_rate_base_resp: ExchangeRateBaseResponse;
  match query_chain(deps, &request) {
    Err(err) => {
      return Err(err);
    }
    Ok(binary) => {
      match from_binary::<ExchangeRateBaseResponse>(&binary) {
        Err(err) => {
          return Err(err);
        }
        Ok(response) => exchange_rate_base_resp = response,
      };
    }
  }

  Ok(exchange_rate_base_resp)
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
