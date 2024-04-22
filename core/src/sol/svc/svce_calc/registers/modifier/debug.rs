use crate::{
    sol::{
        svc::{
            debug::{check_fit, check_item},
            svce_calc::debug::check_modifier,
        },
        SolView,
    },
    util::DebugResult,
};

use super::SolModifierRegister;

impl SolModifierRegister {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> DebugResult {
        for (item_id, modifiers) in self.by_affector.iter() {
            check_item(sol_view, item_id)?;
            for modifier in modifiers {
                check_modifier(sol_view, modifier)?;
            }
        }
        for (item_id, modifiers) in self.direct.iter() {
            check_item(sol_view, item_id)?;
            for modifier in modifiers {
                check_modifier(sol_view, modifier)?;
            }
        }
        for ((fit_id, _), modifiers) in self.root.iter() {
            check_fit(sol_view, fit_id)?;
            for modifier in modifiers {
                check_modifier(sol_view, modifier)?;
            }
        }
        for (item_id, modifiers) in self.other.iter() {
            check_item(sol_view, item_id)?;
            for modifier in modifiers {
                check_modifier(sol_view, modifier)?;
            }
        }
        for ((fit_id, _), modifiers) in self.loc.iter() {
            check_fit(sol_view, fit_id)?;
            for modifier in modifiers {
                check_modifier(sol_view, modifier)?;
            }
        }
        for ((fit_id, _, _), modifiers) in self.loc_grp.iter() {
            check_fit(sol_view, fit_id)?;
            for modifier in modifiers {
                check_modifier(sol_view, modifier)?;
            }
        }
        for ((fit_id, _, _), modifiers) in self.loc_srq.iter() {
            check_fit(sol_view, fit_id)?;
            for modifier in modifiers {
                check_modifier(sol_view, modifier)?;
            }
        }
        for ((fit_id, _), modifiers) in self.own_srq.iter() {
            check_fit(sol_view, fit_id)?;
            for modifier in modifiers {
                check_modifier(sol_view, modifier)?;
            }
        }
        for (fit_id, modifiers) in self.buff_all.iter() {
            check_fit(sol_view, fit_id)?;
            for modifier in modifiers {
                check_modifier(sol_view, modifier)?;
            }
        }
        for (fit_id, modifiers) in self.fleet_fit.iter() {
            check_fit(sol_view, fit_id)?;
            for modifier in modifiers {
                check_modifier(sol_view, modifier)?;
            }
        }
        for modifiers in self.sw.iter() {
            check_modifier(sol_view, modifiers)?;
        }
        Ok(())
    }
}
