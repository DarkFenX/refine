use crate::info::{ItemInfoMode, RigInfo};

pub enum ItemInfo {
    Rig(RigInfo),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemInfo {
    pub(crate) fn from_core(core_item: &mut rc::ItemMut, item_mode: ItemInfoMode) -> Self {
        match core_item {
            rc::ItemMut::Rig(core_rig) => Self::from_core_rig(core_rig, item_mode),
            _ => panic!(),
        }
    }
    pub(crate) fn from_core_rig(core_rig: &mut rc::RigMut, item_mode: ItemInfoMode) -> Self {
        Self::Rig(RigInfo::from_core(core_rig, item_mode))
    }
}
