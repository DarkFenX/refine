use crate::{
    ss::{
        svc::{
            debug::{check_effect, check_item},
            svce_calc::modifier::SsAttrMod,
        },
        SsView,
    },
    util::DebugResult,
};

pub(in crate::ss::svc) fn check_modifier(ss_view: &SsView, ss_mod: &SsAttrMod) -> DebugResult {
    check_item(ss_view, &ss_mod.src_item_id)?;
    check_effect(ss_view, &ss_mod.src_effect_id)?;
    Ok(())
}
