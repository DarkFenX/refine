use crate::sol::{
    debug::{DebugResult, check_a_attr_id, check_a_effect_id, check_item_key},
    uad::Uad,
};

use super::DependencyRegister;

impl DependencyRegister {
    pub(in crate::sol) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        for (affector_spec, affector_data) in self.data.iter() {
            check_item_key(uad, affector_spec.item_key, true)?;
            check_a_attr_id(uad, &affector_spec.a_attr_id)?;
            for affectee_spec in affector_data {
                check_item_key(uad, affectee_spec.item_key, true)?;
                check_a_attr_id(uad, &affectee_spec.a_attr_id)?;
            }
        }
        for (item_key, attrs_iter) in self.anonymous_by_item.iter() {
            check_item_key(uad, *item_key, true)?;
            for (affector_a_attr_id, affectee_a_attr_id) in attrs_iter {
                check_a_attr_id(uad, affector_a_attr_id)?;
                check_a_attr_id(uad, affectee_a_attr_id)?;
            }
        }
        for (source, specs) in self.by_source.iter() {
            check_item_key(uad, source.item_key, true)?;
            check_a_effect_id(uad, &source.a_effect_id)?;
            for (affector_spec, affectee_spec) in specs {
                check_item_key(uad, affector_spec.item_key, true)?;
                check_a_attr_id(uad, &affector_spec.a_attr_id)?;
                check_item_key(uad, affectee_spec.item_key, true)?;
                check_a_attr_id(uad, &affectee_spec.a_attr_id)?;
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
