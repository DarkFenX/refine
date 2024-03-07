use crate::{
    defs::{AttrVal, SsItemId},
    ec,
    ss::{svc::SsSvcs, SsView},
    util::{Error, ErrorKind, Result},
};

pub(in crate::ss::svc::svce_calc::modifier) fn get_mod_val(
    svc: &mut SsSvcs,
    ss_view: &SsView,
    item_id: &SsItemId,
) -> Result<AttrVal> {
    let speed_boost = svc
        .calc_get_item_attr_val(ss_view, item_id, &ec::attrs::SPEED_FACTOR)
        .map_err(|_| Error::new(ErrorKind::CustomModCalc))?;
    let thrust = svc
        .calc_get_item_attr_val(ss_view, item_id, &ec::attrs::SPEED_BOOST_FACTOR)
        .map_err(|_| Error::new(ErrorKind::CustomModCalc))?;
    let prop_item = ss_view
        .items
        .get_item(item_id)
        .map_err(|_| Error::new(ErrorKind::CustomModCalc))?;
    let fit_id = prop_item
        .get_fit_id()
        .ok_or_else(|| Error::new(ErrorKind::CustomModCalc))?;
    let fit = ss_view.fits.get_fit(&fit_id)?;
    let ship_id = fit.ship.ok_or_else(|| Error::new(ErrorKind::CustomModCalc))?;
    let mass = svc
        .calc_get_item_attr_val(ss_view, &ship_id, &ec::attrs::MASS)
        .map_err(|_| Error::new(ErrorKind::CustomModCalc))?;
    let perc = speed_boost.dogma * thrust.dogma / mass.dogma;
    if perc.is_infinite() {
        return Err(Error::new(ErrorKind::CustomModCalc));
    }
    let val = 1.0 + perc / 100.0;
    Ok(val)
}
