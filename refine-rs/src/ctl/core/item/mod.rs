pub(crate) use add_autodetect::ItemAutodetectAddCmdCtxFitGen;
pub use add_autodetect::{
    FitGetItemAutodetectAddError, ItemAutodetectAddCmd, ItemAutodetectAddCmdCtxFit, ItemAutodetectAddError,
};
pub(crate) use remove::ItemRemoveCmdCtxItemGen;
pub use remove::{ItemGetItemRemoveError, ItemRemoveCmd, ItemRemoveError};

mod add_autodetect;
mod remove;
