pub use add::{FitAddFighterError, GetFitAddFighterError};
pub(in crate::ctl) use add::{
    ICmdFighterAddFCtxBIds, ICmdFighterAddFCtxRIds, ICmdFighterAddICtxBIds, ICmdFighterAddICtxRIds,
    ICmdFighterAddShared,
};
pub use change::{GetItemChangeFighterError, ItemChangeFighterError};
pub(in crate::ctl) use change::{ICmdFighterChangeFCtxBIds, ICmdFighterChangeFCtxRIds, ICmdFighterChangeICtxRIds};

mod add;
mod change;
