pub mod query;
pub mod query_leverage;

pub use query::{
  UmeeQuery, StructUmeeQuery
};

pub use query_leverage::{
  UmeeQueryLeverage,
  ASSIGNED_QUERY_GET_BORROW, BorrowParams, BorrowResponse
};

// This is a signal, such that any contract that imports these helpers will only run on the
// umee blockchain
#[no_mangle]
extern "C" fn requires_umee() {}
