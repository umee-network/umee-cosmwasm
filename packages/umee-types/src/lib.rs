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
  AvailableBorrowParams, AvailableBorrowResponse, BorrowAPYParams, BorrowAPYResponse,
  BorrowedParams, BorrowedResponse, BorrowedValueParams, BorrowedValueResponse, CollateralParams,
  CollateralResponse, CollateralValueParams, CollateralValueResponse, ExchangeRateParams,
  ExchangeRateResponse, LeverageParametersParams, LeverageParametersResponse, MarketSizeParams,
  MarketSizeResponse, RegisteredTokensParams, RegisteredTokensResponse, ReserveAmountParams,
  ReserveAmountResponse, SuppliedParams, SuppliedResponse, SuppliedValueParams,
  SuppliedValueResponse, SupplyAPYParams, SupplyAPYResponse, TokenMarketSizeParams,
  TokenMarketSizeResponse, UmeeQueryLeverage, ASSIGNED_QUERY_AVAILABLE_BORROW,
  ASSIGNED_QUERY_BORROWED, ASSIGNED_QUERY_BORROWED_VALUE, ASSIGNED_QUERY_BORROW_APY,
  ASSIGNED_QUERY_COLLATERAL, ASSIGNED_QUERY_COLLATERAL_VALUE, ASSIGNED_QUERY_EXCHANGE_RATE,
  ASSIGNED_QUERY_LEVERAGE_PARAMS, ASSIGNED_QUERY_MARKET_SIZE, ASSIGNED_QUERY_REGISTERED_TOKENS,
  ASSIGNED_QUERY_RESERVE_AMOUNT, ASSIGNED_QUERY_SUPPLIED, ASSIGNED_QUERY_SUPPLIED_VALUE,
  ASSIGNED_QUERY_SUPPLY_APY, ASSIGNED_QUERY_TOKEN_MARKET_SIZE,
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
