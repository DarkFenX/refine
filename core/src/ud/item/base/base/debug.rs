use crate::{
    dbg::{DebugResult, check_attr_rid, check_effect_rid},
    ud::{UData, item::base::UItemBase},
};

impl UItemBase {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        if let Some(attrs) = self.get_attrs() {
            for &attr_rid in attrs.keys() {
                check_attr_rid(u_data, attr_rid)?;
            }
        }
        if let Some(reff_rids) = self.get_reffs() {
            for &effect_rid in reff_rids.iter() {
                check_effect_rid(u_data, effect_rid)?;
            }
        }
        if let Some(attr_rids) = self.get_cap_use_attr_rids() {
            for &attr_rid in attr_rids.iter() {
                check_attr_rid(u_data, attr_rid)?;
            }
        }
        if let Some(axt) = self.get_axt() {
            axt.consistency_check(u_data)?;
        }
        self.effect_modes.consistency_check(u_data)?;
        Ok(())
    }
}
