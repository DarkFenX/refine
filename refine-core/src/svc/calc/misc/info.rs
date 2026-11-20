use crate::{rd::RAttrId, ud::UItemId};

#[derive(Copy, Clone)]
pub(crate) struct CalcModInfoAffector {
    pub(crate) item_uid: UItemId,
    pub(crate) attr_rid: Option<RAttrId>,
}
