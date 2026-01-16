use super::main::RawModifier;
use crate::{dbg::DebugResult, ud::UData};

impl RawModifier {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.affector_espec.consistency_check(u_data, true)?;
        self.affectee_attr_rid.consistency_check(u_data)?;
        if let Some(attr_rid) = self.buff_type_attr_rid {
            attr_rid.consistency_check(u_data)?;
        }
        for attr_rid in self.proj_attr_rids.iter() {
            if let Some(attr_rid) = attr_rid {
                attr_rid.consistency_check(u_data)?;
            }
        }
        if let Some(attr_rid) = self.resist_attr_rid {
            attr_rid.consistency_check(u_data)?;
        }
        Ok(())
    }
}
