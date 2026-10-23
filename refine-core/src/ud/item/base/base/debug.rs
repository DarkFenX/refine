use super::main::ItemBaseCache;
use crate::{
    dbg::DebugResult,
    ud::{UData, item::base::UItemBase},
};

impl UItemBase {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let Self {
            effect_modes,
            cache,
            // Not checked
            item_id: _,
            type_aid: _,
            state: _,
        } = self;
        if let Some(ItemBaseCache { r_item, reffs }) = cache {
            for cap_consumer in r_item.base.cap_consumers.iter() {
                cap_consumer.consistency_check(u_data)?;
            }
            r_item.attr_data.consistency_check(u_data)?;
            for attr_rid in r_item.attr_data.attrs.keys() {
                attr_rid.consistency_check(u_data)?;
            }
            for effect_rid in reffs.iter() {
                effect_rid.consistency_check(u_data)?;
            }
        }
        effect_modes.consistency_check(u_data)?;
        Ok(())
    }
}
