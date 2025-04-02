use crate::sol::{
    debug::{DebugResult, check_a_attr_id, check_a_effect_id, check_item_id},
    uad::Uad,
};

use super::DependencyRegister;

impl DependencyRegister {
    pub(in crate::sol) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        for (affector_spec, affector_data) in self.data.iter() {
            check_item_id(uad, &affector_spec.item_id, true)?;
            check_a_attr_id(uad, &affector_spec.a_attr_id)?;
            for (affectee_spec, sources) in affector_data.iter() {
                check_item_id(uad, &affectee_spec.item_id, true)?;
                check_a_attr_id(uad, &affectee_spec.a_attr_id)?;
                for source in sources {
                    if let Some(source) = source {
                        check_item_id(uad, &source.item_id, true)?;
                        check_a_effect_id(uad, &source.a_effect_id)?;
                    }
                }
            }
        }
        for (item_id, attrs_iter) in self.anonymous_by_item.iter() {
            check_item_id(uad, item_id, true)?;
            for (affector_a_attr_id, affectee_a_attr_id) in attrs_iter {
                check_a_attr_id(uad, affector_a_attr_id)?;
                check_a_attr_id(uad, affectee_a_attr_id)?;
            }
        }
        for (source, specs) in self.by_source.iter() {
            check_item_id(uad, &source.item_id, true)?;
            check_a_effect_id(uad, &source.a_effect_id)?;
            for (affector_spec, affectee_spec) in specs {
                check_item_id(uad, &affector_spec.item_id, true)?;
                check_a_attr_id(uad, &affector_spec.a_attr_id)?;
                check_item_id(uad, &affectee_spec.item_id, true)?;
                check_a_attr_id(uad, &affectee_spec.a_attr_id)?;
            }
        }
        for (item_id, sources) in self.source_by_item.iter() {
            check_item_id(uad, item_id, true)?;
            for source in sources {
                check_item_id(uad, &source.item_id, true)?;
                check_a_effect_id(uad, &source.a_effect_id)?;
            }
        }
        Ok(())
    }
}
