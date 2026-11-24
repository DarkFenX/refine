pub use rc::err::{FitIdParseError, FleetIdParseError, ItemIdParseError};

pub use crate::{
    api::{
        FitGetError, FitHybridBatchError, FleetGetError, ItemGetError, SolAddError, SolGetError, SolHybridBatchError,
        SolIdParseError, SolRemoveError, SolSwitchSrcError,
    },
    shared::BrResolveError,
};

pub mod core {
    pub use rc::err::*;
}
