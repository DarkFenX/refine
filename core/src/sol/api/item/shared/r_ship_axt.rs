use crate::{
    rd,
    uad::{Uad, UadFitKey},
};

pub(in crate::sol::api) fn get_r_ship_axt(uad: &Uad, fit_key: UadFitKey) -> Option<&rd::RItemAXt> {
    let uad_fit = uad.fits.get(fit_key);
    let ship_key = uad_fit.ship?;
    let uad_ship = uad.items.get(ship_key);
    uad_ship.get_r_axt()
}
