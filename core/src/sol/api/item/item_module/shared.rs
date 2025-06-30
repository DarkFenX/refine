use crate::{
    def::FitKey,
    misc::ModRack,
    uad::{Fits, ItemVec},
};

pub(super) fn get_fit_rack(uad_fits: &Fits, fit_key: FitKey, rack: ModRack) -> &ItemVec {
    let uad_fit = uad_fits.get(fit_key);
    match rack {
        ModRack::High => &uad_fit.mods_high,
        ModRack::Mid => &uad_fit.mods_mid,
        ModRack::Low => &uad_fit.mods_low,
    }
}

pub(super) fn get_fit_rack_mut(uad_fits: &mut Fits, fit_key: FitKey, rack: ModRack) -> &mut ItemVec {
    let uad_fit = uad_fits.get_mut(fit_key);
    match rack {
        ModRack::High => &mut uad_fit.mods_high,
        ModRack::Mid => &mut uad_fit.mods_mid,
        ModRack::Low => &mut uad_fit.mods_low,
    }
}
