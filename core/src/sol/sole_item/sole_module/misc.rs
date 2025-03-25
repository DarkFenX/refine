use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, ModRack, SolarSystem,
        info::ModuleInfo,
        uad::{
            fit::{Fits, ItemVec},
            item::Module,
        },
    },
};

impl SolarSystem {
    pub(in crate::sol) fn make_module_info(&self, module: &Module) -> ModuleInfo {
        let charge_info = module.get_charge_item_id().map(|i| self.get_charge(&i).unwrap());
        ModuleInfo::from_mod_and_charge_with_source(&self.uad.src, module, charge_info)
    }
}

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
