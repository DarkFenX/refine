use crate::{
    misc::ModRack,
    uad::{UadFitKey, UadFits, UadItemVec},
};

pub(super) fn get_fit_rack(uad_fits: &UadFits, fit_key: UadFitKey, rack: ModRack) -> &UadItemVec {
    let uad_fit = uad_fits.get(fit_key);
    match rack {
        ModRack::High => &uad_fit.mods_high,
        ModRack::Mid => &uad_fit.mods_mid,
        ModRack::Low => &uad_fit.mods_low,
    }
}

pub(super) fn get_fit_rack_mut(uad_fits: &mut UadFits, fit_key: UadFitKey, rack: ModRack) -> &mut UadItemVec {
    let uad_fit = uad_fits.get_mut(fit_key);
    match rack {
        ModRack::High => &mut uad_fit.mods_high,
        ModRack::Mid => &mut uad_fit.mods_mid,
        ModRack::Low => &mut uad_fit.mods_low,
    }
}
