pub use validation::{SolValOptions, SolValResult};
pub(in crate::sol::svc) use vast::SolVast;
use vast::SolVastFitData;
use vaste_stats_fit::SolStatRes;
pub use vaste_vals_fit::{SolResUser, SolResValFail};

mod validation;
mod vast;
mod vaste_debug;
mod vaste_maintain;
mod vaste_stats_fit;
mod vaste_validate;
mod vaste_vals_fit;
