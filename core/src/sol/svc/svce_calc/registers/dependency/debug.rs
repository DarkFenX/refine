use crate::{
    sol::{
        svc::debug::{check_attr, check_effect, check_item},
        SolView,
    },
    util::DebugResult,
};

use super::SolDependencyRegister;

impl SolDependencyRegister {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> DebugResult {
        for (affector_spec, affectee_specs) in self.data.iter() {
            check_item(sol_view, &affector_spec.item_id)?;
            check_attr(sol_view, &affector_spec.attr_id)?;
            for affectee_spec in affectee_specs {
                check_item(sol_view, &affectee_spec.item_id)?;
                check_attr(sol_view, &affectee_spec.attr_id)?;
            }
        }
        for (item_id, specs) in self.affector_by_item.iter() {
            check_item(sol_view, item_id)?;
            for spec in specs {
                check_item(sol_view, &spec.item_id)?;
                check_attr(sol_view, &spec.attr_id)?;
            }
        }
        for (item_id, spec_map) in self.affectee_by_item.iter() {
            check_item(sol_view, item_id)?;
            for (affector_spec, affectee_specs) in spec_map.iter() {
                check_item(sol_view, &affector_spec.item_id)?;
                check_attr(sol_view, &affector_spec.attr_id)?;
                for affectee_spec in affectee_specs {
                    check_item(sol_view, &affectee_spec.item_id)?;
                    check_attr(sol_view, &affectee_spec.attr_id)?;
                }
            }
        }
        for ((item_id, effect_id), specs) in self.by_source.iter() {
            check_item(sol_view, item_id)?;
            check_effect(sol_view, effect_id)?;
            for (affector_spec, affectee_spec) in specs {
                check_item(sol_view, &affector_spec.item_id)?;
                check_attr(sol_view, &affector_spec.attr_id)?;
                check_item(sol_view, &affectee_spec.item_id)?;
                check_attr(sol_view, &affectee_spec.attr_id)?;
            }
        }
        for (item_id, sources) in self.source_by_item.iter() {
            check_item(sol_view, item_id)?;
            for (source_item_id, source_effect_id) in sources {
                check_item(sol_view, source_item_id)?;
                check_effect(sol_view, source_effect_id)?;
            }
        }
        Ok(())
    }
}
