use super::DependencyRegister;
use crate::{
    dbg::{DebugResult, check_a_effect_id, check_item_key},
    uad::Uad,
};

impl DependencyRegister {
    pub(in crate::svc::calc) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for (affector_spec, affector_data) in self.data.iter() {
            check_item_key(uad, affector_spec.item_key, true)?;
            for affectee_spec in affector_data {
                check_item_key(uad, affectee_spec.item_key, true)?;
            }
        }
        for item_key in self.anonymous_by_item.keys() {
            check_item_key(uad, *item_key, true)?;
        }
        for (source, specs) in self.by_source.iter() {
            check_item_key(uad, source.item_key, true)?;
            check_a_effect_id(uad, &source.a_effect_id)?;
            for (affector_spec, affectee_spec) in specs {
                check_item_key(uad, affector_spec.item_key, true)?;
                check_item_key(uad, affectee_spec.item_key, true)?;
            }
        }
        for (&item_key, sources) in self.source_by_item.iter() {
            check_item_key(uad, item_key, true)?;
            for source in sources {
                check_item_key(uad, source.item_key, true)?;
                check_a_effect_id(uad, &source.a_effect_id)?;
            }
        }
        Ok(())
    }
}
