use crate::sol::{
    FitKey, ModRack,
    uad::fit::{Fits, ItemVec},
};

pub(super) fn get_fit_rack(uad_fits: &mut Fits, fit_key: FitKey, rack: ModRack) -> &mut ItemVec {
    let fit = uad_fits.get_mut(fit_key);
    match rack {
        ModRack::High => &mut fit.mods_high,
        ModRack::Mid => &mut fit.mods_mid,
        ModRack::Low => &mut fit.mods_low,
    }
}
