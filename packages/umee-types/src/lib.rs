pub mod query;
pub mod query_leverage;
pub mod query_oracle;

pub use query::{StructUmeeQuery, UmeeQuery};

pub use query_leverage::{
  BorrowParams, BorrowResponse, UmeeQueryLeverage, ASSIGNED_QUERY_GET_BORROW,
};

pub use query_oracle::{
  ExchangeRateBaseParams, ExchangeRateBaseResponse, UmeeQueryOracle,
  ASSIGNED_QUERY_GET_EXCHANGE_RATE_BASE,
};

// This is a signal, such that any contract that imports these helpers will only run on the
// umee blockchain, it makes mandatory that the blockchain have the "umee" inside
// the supported features when instantiating a new wasm keeper
#[no_mangle]
extern "C" fn requires_umee() {}
