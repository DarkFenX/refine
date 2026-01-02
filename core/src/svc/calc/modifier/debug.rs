use crate::{
    dbg::{DebugResult, check_attr_id, check_effect_id, check_fit_id, check_item_id},
    svc::calc::{Context, CtxModifier, RawModifier},
    ud::UData,
};

pub(in crate::svc) fn check_rmod(u_data: &UData, rmod: &RawModifier) -> DebugResult {
    check_item_id(u_data, rmod.affector_espec.item_key, true)?;
    check_effect_id(u_data, rmod.affector_espec.effect_key)?;
    check_attr_id(u_data, rmod.affectee_attr_key)?;
    if let Some(attr_key) = rmod.buff_type_attr_key {
        check_attr_id(u_data, attr_key)?;
    }
    for &attr_key in rmod.proj_attr_keys.iter() {
        if let Some(attr_key) = attr_key {
            check_attr_id(u_data, attr_key)?;
        }
    }
    if let Some(attr_key) = rmod.resist_attr_key {
        check_attr_id(u_data, attr_key)?;
    }
    Ok(())
}

pub(in crate::svc) fn check_cmod(u_data: &UData, cmod: &CtxModifier) -> DebugResult {
    match cmod.ctx {
        // Item modifier is applied to is not necessarily loaded (e.g. a module projected to a
        // non-loaded ship)
        Context::None => (),
        Context::Item(item_key) => check_item_id(u_data, item_key, false)?,
        Context::Fit(fit_key) => check_fit_id(u_data, fit_key)?,
        Context::FitItem(fit_key, item_key) => {
            check_fit_id(u_data, fit_key)?;
            check_item_id(u_data, item_key, false)?;
        }
    }
    check_rmod(u_data, &cmod.raw)?;
    Ok(())
}
