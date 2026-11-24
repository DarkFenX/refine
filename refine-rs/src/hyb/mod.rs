pub use fit::FitHybridCmdBr;
pub use sol::SolHybridCmdBr;

mod fit;
mod sol;

pub mod err {
    pub use crate::{
        api::{FitHybridBatchError, SolHybridBatchError},
        hyb::{fit::FitHybridError, sol::SolHybridError},
    };
}
