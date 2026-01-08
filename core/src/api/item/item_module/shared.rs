use crate::{
    misc::ModRack,
    ud::{UFitId, UFits, UItemVec},
};

pub(super) fn get_fit_rack(u_fits: &UFits, fit_uid: UFitId, rack: ModRack) -> &UItemVec {
    let u_fit = u_fits.get(fit_uid);
    match rack {
        ModRack::High => &u_fit.mods_high,
        ModRack::Mid => &u_fit.mods_mid,
        ModRack::Low => &u_fit.mods_low,
    }
}

pub(super) fn get_fit_rack_mut(u_fits: &mut UFits, fit_uid: UFitId, rack: ModRack) -> &mut UItemVec {
    let u_fit = u_fits.get_mut(fit_uid);
    match rack {
        ModRack::High => &mut u_fit.mods_high,
        ModRack::Mid => &mut u_fit.mods_mid,
        ModRack::Low => &mut u_fit.mods_low,
    }
}
