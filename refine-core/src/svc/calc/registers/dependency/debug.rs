use super::DependencyRegister;
use crate::{dbg::DebugResult, ud::UData};

impl DependencyRegister {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for (affector_spec, affector_data) in self.data.iter() {
            affector_spec.consistency_check(u_data, true)?;
            for affectee_spec in affector_data {
                affectee_spec.consistency_check(u_data, true)?;
            }
        }
        for (item_uid, item_data) in self.anonymous_by_item.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (attr1_rid, attr2_rid) in item_data {
                attr1_rid.consistency_check(u_data)?;
                attr2_rid.consistency_check(u_data)?;
            }
        }
        for (source, specs) in self.by_source.iter() {
            source.consistency_check(u_data, true)?;
            for (affector_spec, affectee_spec) in specs {
                affector_spec.consistency_check(u_data, true)?;
                affectee_spec.consistency_check(u_data, true)?;
            }
        }
        for (&item_uid, sources) in self.source_by_item.iter() {
            item_uid.consistency_check(u_data, true)?;
            for source in sources {
                source.consistency_check(u_data, true)?;
            }
        }
        Ok(())
    }
}
