use crate::{
    sol::{
        svc::debug::{check_attr, check_item},
        SolView,
    },
    util::DebugResult,
};

use super::SolDependencyRegister;

impl SolDependencyRegister {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> DebugResult {
        for (affector_attr_spec, affectee_attr_specs) in self.data.iter() {
            check_item(sol_view, &affector_attr_spec.item_id)?;
            check_attr(sol_view, &affector_attr_spec.attr_id)?;
            for affectee_attr_spec in affectee_attr_specs {
                check_item(sol_view, &affectee_attr_spec.item_id)?;
                check_attr(sol_view, &affectee_attr_spec.attr_id)?;
            }
        }
        for (item_id, attr_specs) in self.item_affector_map.iter() {
            check_item(sol_view, item_id)?;
            for attr_spec in attr_specs {
                check_item(sol_view, &attr_spec.item_id)?;
                check_attr(sol_view, &attr_spec.attr_id)?;
            }
        }
        for (item_id, spec_map) in self.item_affectee_map.iter() {
            check_item(sol_view, item_id)?;
            for (affector_attr_spec, affectee_attr_specs) in spec_map.iter() {
                check_item(sol_view, &affector_attr_spec.item_id)?;
                check_attr(sol_view, &affector_attr_spec.attr_id)?;
                for affectee_attr_spec in affectee_attr_specs {
                    check_item(sol_view, &affectee_attr_spec.item_id)?;
                    check_attr(sol_view, &affectee_attr_spec.attr_id)?;
                }
            }
        }
        Ok(())
    }
}
