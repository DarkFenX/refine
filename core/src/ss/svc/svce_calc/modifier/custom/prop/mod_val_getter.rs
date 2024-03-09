use crate::{
    defs::{AttrVal, SsItemId},
    ss::{svc::SsSvcs, SsView},
    util::{Error, ErrorKind, Result},
};

use super::{
    attr::{PROP_BOOST, PROP_THRUST, SHIP_MASS},
    deps::reg_dependencies,
    misc::get_ship_id,
};

pub(in crate::ss::svc::svce_calc::modifier) fn get_mod_val(
    svc: &mut SsSvcs,
    ss_view: &SsView,
    item_id: &SsItemId,
) -> Result<AttrVal> {
    let speed_boost = svc
        .calc_get_item_attr_val(ss_view, item_id, &PROP_BOOST)
        .map_err(|_| Error::new(ErrorKind::CustomModCalc))?;
    let thrust = svc
        .calc_get_item_attr_val(ss_view, item_id, &PROP_THRUST)
        .map_err(|_| Error::new(ErrorKind::CustomModCalc))?;
    let ship_id = get_ship_id(ss_view, item_id)
        .map_err(|_| Error::new(ErrorKind::CustomModCalc))?
        .ok_or_else(|| Error::new(ErrorKind::CustomModCalc))?;
    let mass = svc
        .calc_get_item_attr_val(ss_view, &ship_id, &SHIP_MASS)
        .map_err(|_| Error::new(ErrorKind::CustomModCalc))?;
    let perc = speed_boost.dogma * thrust.dogma / mass.dogma;
    if perc.is_infinite() {
        return Err(Error::new(ErrorKind::CustomModCalc));
    }
    let val = 1.0 + perc / 100.0;
    // Register dependencies, so that target attribute is properly cleared up when any of source
    // attributes change
    reg_dependencies(svc, *item_id, ship_id);
    Ok(val)
}
