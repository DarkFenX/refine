use super::DependencyRegister;
use crate::{
    dbg::{DebugResult, check_effect_key, check_item_key},
    ud::UData,
};

impl DependencyRegister {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for (affector_spec, affector_data) in self.data.iter() {
            check_item_key(u_data, affector_spec.item_key, true)?;
            for affectee_spec in affector_data {
                check_item_key(u_data, affectee_spec.item_key, true)?;
            }
        }
        for item_key in self.anonymous_by_item.keys() {
            check_item_key(u_data, *item_key, true)?;
        }
        for (source, specs) in self.by_source.iter() {
            check_item_key(u_data, source.item_key, true)?;
            check_effect_key(u_data, source.effect_key)?;
            for (affector_spec, affectee_spec) in specs {
                check_item_key(u_data, affector_spec.item_key, true)?;
                check_item_key(u_data, affectee_spec.item_key, true)?;
            }
        }
        for (&item_key, sources) in self.source_by_item.iter() {
            check_item_key(u_data, item_key, true)?;
            for source in sources {
                check_item_key(u_data, source.item_key, true)?;
                check_effect_key(u_data, source.effect_key)?;
            }
        }
        Ok(())
    }
}
