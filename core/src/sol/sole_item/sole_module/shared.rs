use crate::sol::{
    FitKey, ModRack,
    uad::{Fits, fit::ItemVec},
};

pub(super) fn get_fit_rack<'a>(uad_fits: &'a mut Fits, fit_key: FitKey, rack: ModRack) -> &'a mut ItemVec {
    let fit = uad_fits.get_mut(fit_key);
    match rack {
        ModRack::High => &mut fit.mods_high,
        ModRack::Mid => &mut fit.mods_mid,
        ModRack::Low => &mut fit.mods_low,
    }
}
