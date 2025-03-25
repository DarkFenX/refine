use crate::sol::{
    debug::{DebugResult, check_a_attr_id, check_a_effect_id, check_item_id},
    uad::Uad,
};

use super::DependencyRegister;

impl DependencyRegister {
    pub(in crate::sol) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        for (affector_spec, affectee_specs) in self.data.iter() {
            check_item_id(uad, &affector_spec.item_id, true)?;
            check_a_attr_id(uad, &affector_spec.a_attr_id)?;
            for affectee_spec in affectee_specs {
                check_item_id(uad, &affectee_spec.item_id, true)?;
                check_a_attr_id(uad, &affectee_spec.a_attr_id)?;
            }
        }
        for (item_id, specs) in self.affector_by_item.iter() {
            check_item_id(uad, item_id, true)?;
            for spec in specs {
                check_item_id(uad, &spec.item_id, true)?;
                check_a_attr_id(uad, &spec.a_attr_id)?;
            }
        }
        for (item_id, spec_map) in self.affectee_by_item.iter() {
            check_item_id(uad, item_id, true)?;
            for (affector_spec, affectee_specs) in spec_map.iter() {
                check_item_id(uad, &affector_spec.item_id, true)?;
                check_a_attr_id(uad, &affector_spec.a_attr_id)?;
                for affectee_spec in affectee_specs {
                    check_item_id(uad, &affectee_spec.item_id, true)?;
                    check_a_attr_id(uad, &affectee_spec.a_attr_id)?;
                }
            }
        }
        for ((item_id, a_effect_id), specs) in self.by_source.iter() {
            check_item_id(uad, item_id, true)?;
            check_a_effect_id(uad, a_effect_id)?;
            for (affector_spec, affectee_spec) in specs {
                check_item_id(uad, &affector_spec.item_id, true)?;
                check_a_attr_id(uad, &affector_spec.a_attr_id)?;
                check_item_id(uad, &affectee_spec.item_id, true)?;
                check_a_attr_id(uad, &affectee_spec.a_attr_id)?;
            }
        }
        for (item_id, sources) in self.source_by_item.iter() {
            check_item_id(uad, item_id, true)?;
            for (source_item_id, source_a_effect_id) in sources {
                check_item_id(uad, source_item_id, true)?;
                check_a_effect_id(uad, source_a_effect_id)?;
            }
        }
        Ok(())
    }
}
