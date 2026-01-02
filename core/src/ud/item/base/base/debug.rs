use crate::{
    dbg::{DebugResult, check_attr_id, check_effect_id},
    ud::{UData, item::base::UItemBase},
};

impl UItemBase {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        if let Some(attrs) = self.get_attrs() {
            for &attr_key in attrs.keys() {
                check_attr_id(u_data, attr_key)?;
            }
        }
        if let Some(reffs) = self.get_reffs() {
            for &effect_key in reffs.iter() {
                check_effect_id(u_data, effect_key)?;
            }
        }
        if let Some(attr_keys) = self.get_cap_use_attr_keys() {
            for &attr_key in attr_keys.iter() {
                check_attr_id(u_data, attr_key)?;
            }
        }
        if let Some(axt) = self.get_axt() {
            axt.consistency_check(u_data)?;
        }
        self.effect_modes.consistency_check(u_data)?;
        Ok(())
    }
}
