pub mod leverage_parameters;
pub mod msg;
pub mod msg_leverage;
pub mod query;
pub mod query_leverage;
pub mod query_oracle;
pub mod token;

pub use leverage_parameters::LeverageParameters;
pub use token::Token;

pub use query::{StructUmeeQuery, UmeeQuery};

pub use query_leverage::{
  AvailableBorrowParams, AvailableBorrowResponse, BorrowedParams, BorrowedResponse,
  BorrowedValueParams, BorrowedValueResponse, LeverageParametersParams, LeverageParametersResponse,
  RegisteredTokensParams, RegisteredTokensResponse, SuppliedParams, SuppliedResponse,
  SuppliedValueParams, SuppliedValueResponse, UmeeQueryLeverage, ASSIGNED_QUERY_AVAILABLE_BORROW,
  ASSIGNED_QUERY_BORROWED, ASSIGNED_QUERY_BORROWED_VALUE, ASSIGNED_QUERY_LEVERAGE_PARAMS,
  ASSIGNED_QUERY_REGISTERED_TOKENS, ASSIGNED_QUERY_SUPPLIED, ASSIGNED_QUERY_SUPPLIED_VALUE,
};

pub use query_oracle::{
  ExchangeRatesParams, ExchangeRatesResponse, UmeeQueryOracle, ASSIGNED_QUERY_EXCHANGE_RATES,
};

pub use msg_leverage::{
  LendAssetParams, UmeeMsgLeverage, ASSIGNED_MSG_LEND, ASSIGNED_MSG_WITHDRAW,
};

pub use msg::{StructUmeeMsg, UmeeMsg};

// This is a signal, such that any contract that imports these helpers will only run on the
// umee blockchain, it makes mandatory that the blockchain have the "umee" inside
// the supported features when instantiating a new wasm keeper
#[no_mangle]
extern "C" fn requires_umee() {}
