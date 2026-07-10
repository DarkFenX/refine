use crate::{
    dbg::DebugResult,
    ud::{UData, item::base::UItemBase},
};

impl UItemBase {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        if let Some(attrs) = self.get_attrs() {
            for attr_rid in attrs.keys() {
                attr_rid.consistency_check(u_data)?;
            }
        }
        if let Some(reff_rids) = self.get_reffs() {
            for effect_rid in reff_rids.iter() {
                effect_rid.consistency_check(u_data)?;
            }
        }
        if let Some(cap_consumers) = self.get_cap_consumers() {
            for cap_consumer in cap_consumers.iter() {
                cap_consumer.consistency_check(u_data)?;
            }
        }
        if let Some(axt) = self.get_axt() {
            axt.consistency_check(u_data)?;
        }
        self.effect_modes.consistency_check(u_data)?;
        Ok(())
    }
}
