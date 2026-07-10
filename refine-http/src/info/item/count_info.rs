use serde_tuple::Serialize_tuple;

#[derive(Serialize_tuple)]
pub(in crate::info) struct HItemCountInfo {
    current: u32,
    max: u32,
    overridden: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HItemCountInfo {
    pub(in crate::info::item) fn from_core_item_spool(core_item_spool: rc::ItemSpoolInfo) -> Self {
        Self {
            current: core_item_spool.current.into_u32(),
            max: core_item_spool.max.into_u32(),
            overridden: core_item_spool.overridden,
        }
    }
    pub(in crate::info::item) fn from_core_fighter_count(core_fighter_count: rc::FighterCountInfo) -> Self {
        Self {
            current: core_fighter_count.current.into_u32(),
            max: core_fighter_count.max.into_u32(),
            overridden: core_fighter_count.overridden,
        }
    }
}
