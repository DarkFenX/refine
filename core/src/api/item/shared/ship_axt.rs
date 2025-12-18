use crate::{
    rd,
    ud::{UData, UFitKey},
};

pub(in crate::api) fn get_ship_axt(u_data: &UData, fit_key: UFitKey) -> Option<&rd::RItemAXt> {
    let u_fit = u_data.fits.get(fit_key);
    let ship_key = u_fit.ship?;
    let u_ship = u_data.items.get(ship_key);
    u_ship.get_axt()
}
