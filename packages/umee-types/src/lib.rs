pub mod query;
pub mod query_leverage;
pub mod query_oracle;

pub use query::{
  UmeeQuery, StructUmeeQuery
};

pub use query_leverage::{
  UmeeQueryLeverage,
  ASSIGNED_QUERY_GET_BORROW, BorrowParams, BorrowResponse
};

pub use query_oracle::{
  UmeeQueryOracle,
  ASSIGNED_QUERY_GET_EXCHANGE_RATE_BASE, ExchangeRateBaseParams, ExchangeRateBaseResponse
};

// This is a signal, such that any contract that imports these helpers will only run on the
// umee blockchain, it makes mandatory that the blockchain have the "umee" inside
// the supported features when instantiating a new wasm keeper
#[no_mangle]
extern "C" fn requires_umee() {}
