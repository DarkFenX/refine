pub(crate) use fit_fast::ValResultFitFast;
pub use fit_verbose::ValResultFit;
pub use sol::ValResultSol;
pub use validator::ValResult;

mod fit_fast;
mod fit_verbose;
mod sol;
mod validator;
