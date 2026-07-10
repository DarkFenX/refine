use serde_tuple::Serialize_tuple;

use crate::shared::HNpcProp;

#[derive(Serialize_tuple)]
pub(in crate::info::item::item_drone) struct HItemNpcPropInfo {
    value: HNpcProp,
    overridden: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HItemNpcPropInfo {
    pub(in crate::info::item::item_drone) fn from_core(core_npc_prop: rc::ItemNpcPropInfo) -> Self {
        Self {
            value: HNpcProp::from_core(core_npc_prop.value),
            overridden: core_npc_prop.overridden,
        }
    }
}
