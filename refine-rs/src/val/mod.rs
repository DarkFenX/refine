pub use cmd_core::{FitValCmd, SolValCmd};
pub use fit_try_items::TryFitItemsCmd;
pub use fit_val::FitValInfo;
pub use info_mode::ValInfoMode;
pub use rc::val::{ValOptions, ValResultFit as FitValInfoDetails, ValResultSol as SolValInfoDetails};
pub use sol_val::SolValInfo;

mod cmd_core;
mod fit_try_items;
mod fit_val;
mod info_mode;
mod sol_val;
