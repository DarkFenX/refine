use crate::{
    sol::{
        svc::{
            debug::{check_effect, check_item},
            svce_calc::SolAttrMod,
        },
        SolView,
    },
    util::DebugResult,
};

pub(in crate::sol::svc) fn check_modifier(sol_view: &SolView, modifier: &SolAttrMod) -> DebugResult {
    check_item(sol_view, &modifier.affector_item_id)?;
    check_effect(sol_view, &modifier.effect_id)?;
    Ok(())
}
