use crate::{
    ad,
    uad::{Uad, UadFitKey},
};

pub(in crate::sol::api) fn get_a_ship_xt(uad: &Uad, fit_key: UadFitKey) -> Option<&ad::AItemXt> {
    let uad_fit = uad.fits.get(fit_key);
    let ship_key = uad_fit.ship?;
    let uad_ship = uad.items.get(ship_key);
    uad_ship.get_a_xt()
}
