use super::DependencyRegister;
use crate::{dbg::DebugResult, ud::UData};

impl DependencyRegister {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let Self {
            data,
            by_source,
            source_by_item,
        } = self;
        for (affector_spec, affector_data) in data.iter() {
            affector_spec.consistency_check(u_data, true)?;
            for affectee_spec in affector_data {
                affectee_spec.consistency_check(u_data, true)?;
            }
        }
        for (source, specs) in by_source.iter() {
            source.consistency_check(u_data, true)?;
            for (affector_spec, affectee_spec) in specs {
                affector_spec.consistency_check(u_data, true)?;
                affectee_spec.consistency_check(u_data, true)?;
            }
        }
        for (&item_uid, sources) in source_by_item.iter() {
            item_uid.consistency_check(u_data, true)?;
            for source in sources {
                source.consistency_check(u_data, true)?;
            }
        }
        Ok(())
    }
}
