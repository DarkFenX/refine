use crate::{
    dbg::DebugResult,
    ud::{UData, item::base::UItemBase},
};

impl UItemBase {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        if let Some(rib) = self.get_r_item_base() {
            for cap_consumer in rib.cap_consumers.iter() {
                cap_consumer.consistency_check(u_data)?;
            }
        }
        if let Some(riad) = self.get_r_item_attr_data() {
            riad.consistency_check(u_data)?;
            for attr_rid in riad.attrs.keys() {
                attr_rid.consistency_check(u_data)?;
            }
        }
        if let Some(reff_rids) = self.get_reffs() {
            for effect_rid in reff_rids.iter() {
                effect_rid.consistency_check(u_data)?;
            }
        }
        self.effect_modes.consistency_check(u_data)?;
        Ok(())
    }
}
