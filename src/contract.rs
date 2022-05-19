#[cfg(not(feature = "library"))]
use cosmwasm_std::{
  QueryRequest, entry_point, to_binary, Binary, Deps, DepsMut,
  Env, MessageInfo, Response, StdResult, Addr, Coin, Uint128,
  to_vec, StdError, SystemResult, ContractResult
};
use cw2::set_contract_version;
use umee_types::{UmeeQuery, BorrowResponse, StructUmeeQuery, BorrowParams, UmeeQueryLeverage};

use crate::error::ContractError;
use crate::msg::{OwnerResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:umee-cosmwasm";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

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

  Ok(Response::new()
    .add_attribute("method", "instantiate")
    .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  match msg {
    ExecuteMsg::ChangeOwner { new_owner } => try_change_owner(deps, info, new_owner),
  }
}

pub fn try_change_owner(deps: DepsMut, info: MessageInfo, new_owner: Addr) -> Result<Response, ContractError> {
  STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
    if info.sender != state.owner {
      return Err(ContractError::Unauthorized {});
    }
    state.owner = new_owner;
    Ok(state)
  })?;
  Ok(Response::new().add_attribute("method", "change_owner"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
  match msg {
    QueryMsg::GetOwner {} => to_binary(&query_owner(deps)?),
    QueryMsg::Chain { request } => to_binary(&query_chain(deps, &request)?),
    QueryMsg::ChainStruct { request } => to_binary(&query_chain_struct(deps, &request)?),
    QueryMsg::Umee(UmeeQuery::Leverage(UmeeQueryLeverage::GetBorrow(borrow_params))) => to_binary(&query_get_borrow(deps, borrow_params)?),
  }
}

fn query_chain(
  deps: Deps,
  request: &QueryRequest<UmeeQuery>,
) -> StdResult<Binary> {
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

fn query_chain_struct(
  deps: Deps,
  request: &QueryRequest<StructUmeeQuery>,
) -> StdResult<Binary> {
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

fn query_owner(deps: Deps) -> StdResult<OwnerResponse> {
  let state = STATE.load(deps.storage)?;
  Ok(OwnerResponse { owner : state.owner })
}

fn query_get_borrow(_: Deps, borrow_params: BorrowParams) -> StdResult<BorrowResponse> {
  // let request = UmeeQuery::GetBorrow{
  //   borrower_addr: borrower_addr,
  //   denom: denom,
  // };

  println!("request {}, {}", borrow_params.borrower_addr, borrow_params.denom);

  // let request_chain = Chain{

  // }

  // query_chain(deps, QueryRequest::Custom{} request);

  let coin = Coin{
    amount: Uint128::new(123),
    denom: String::from("umee"),
  };

  Ok(BorrowResponse {  borrowed_amount: coin })
}

#[cfg(test)]
mod tests {
  use super::*;
  use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
  use cosmwasm_std::{coins, from_binary};

  #[test]
  fn proper_initialization() {
    let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

    let msg = InstantiateMsg { };
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
    let msg = InstantiateMsg { };
    let info = mock_info(first_owner, &coins(2, "token"));
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();


    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
    let value: OwnerResponse = from_binary(&res).unwrap();
    assert_eq!(first_owner, value.owner);

    let new_owner = "new_owner";

    // only the original creator can change the owner the counter
    let auth_info = mock_info(new_owner, &coins(2, "token"));
    let msg = ExecuteMsg::ChangeOwner { new_owner: cosmwasm_std::Addr::unchecked(new_owner) };
    let res = execute(deps.as_mut(), mock_env(), auth_info, msg);
    match res {
      Err(ContractError::Unauthorized {}) => {}
      _ => panic!("Must return unauthorized error"),
    }

    let auth_info = mock_info(first_owner, &coins(2, "token"));
    let msg = ExecuteMsg::ChangeOwner { new_owner: cosmwasm_std::Addr::unchecked(new_owner) };
    let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
    let value: OwnerResponse = from_binary(&res).unwrap();
    assert_eq!(new_owner, value.owner);
  }
}
