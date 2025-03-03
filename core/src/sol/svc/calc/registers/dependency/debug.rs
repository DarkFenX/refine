use crate::sol::{
    debug::{SolDebugResult, check_attr, check_effect, check_item},
    uad::SolUad,
};

use super::SolDependencyRegister;

impl SolDependencyRegister {
    pub(in crate::sol) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        for (affector_spec, affectee_specs) in self.data.iter() {
            check_item(uad, &affector_spec.item_id, true)?;
            check_attr(uad, &affector_spec.attr_id)?;
            for affectee_spec in affectee_specs {
                check_item(uad, &affectee_spec.item_id, true)?;
                check_attr(uad, &affectee_spec.attr_id)?;
            }
        }
        for (item_id, specs) in self.affector_by_item.iter() {
            check_item(uad, item_id, true)?;
            for spec in specs {
                check_item(uad, &spec.item_id, true)?;
                check_attr(uad, &spec.attr_id)?;
            }
        }
        for (item_id, spec_map) in self.affectee_by_item.iter() {
            check_item(uad, item_id, true)?;
            for (affector_spec, affectee_specs) in spec_map.iter() {
                check_item(uad, &affector_spec.item_id, true)?;
                check_attr(uad, &affector_spec.attr_id)?;
                for affectee_spec in affectee_specs {
                    check_item(uad, &affectee_spec.item_id, true)?;
                    check_attr(uad, &affectee_spec.attr_id)?;
                }
            }
        }
        for ((item_id, effect_id), specs) in self.by_source.iter() {
            check_item(uad, item_id, true)?;
            check_effect(uad, effect_id)?;
            for (affector_spec, affectee_spec) in specs {
                check_item(uad, &affector_spec.item_id, true)?;
                check_attr(uad, &affector_spec.attr_id)?;
                check_item(uad, &affectee_spec.item_id, true)?;
                check_attr(uad, &affectee_spec.attr_id)?;
            }
        }
        for (item_id, sources) in self.source_by_item.iter() {
            check_item(uad, item_id, true)?;
            for (source_item_id, source_effect_id) in sources {
                check_item(uad, source_item_id, true)?;
                check_effect(uad, source_effect_id)?;
            }
        }
        Ok(())
    }
}
