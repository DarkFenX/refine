use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, ModRack,
        uad::fit::{Fits, ItemVec},
    },
};

pub(super) fn get_fit_rack<'a>(
    uad_fits: &'a mut Fits,
    fit_id: &FitId,
    rack: ModRack,
) -> Result<&'a mut ItemVec, FitFoundError> {
    let fit = uad_fits.get_fit_mut(fit_id)?;
    let fit_rack = match rack {
        ModRack::High => &mut fit.mods_high,
        ModRack::Mid => &mut fit.mods_mid,
        ModRack::Low => &mut fit.mods_low,
    };
    Ok(fit_rack)
}
