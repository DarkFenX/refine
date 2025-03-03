use crate::{
    defs::SolFitId,
    err::basic::FitFoundError,
    sol::{
        SolModRack, SolarSystem,
        info::SolModuleInfo,
        uad::{
            fit::{SolFits, SolItemVec},
            item::SolModule,
        },
    },
};

impl SolarSystem {
    pub(in crate::sol) fn make_module_info(&self, module: &SolModule) -> SolModuleInfo {
        let charge_info = module.get_charge_id().map(|i| self.get_charge(&i).unwrap());
        SolModuleInfo::from_mod_and_charge_with_source(&self.uad.src, module, charge_info)
    }
}

pub(super) fn get_fit_rack<'a>(
    uad_fits: &'a mut SolFits,
    fit_id: &SolFitId,
    rack: SolModRack,
) -> Result<&'a mut SolItemVec, FitFoundError> {
    let fit = uad_fits.get_fit_mut(fit_id)?;
    let fit_rack = match rack {
        SolModRack::High => &mut fit.mods_high,
        SolModRack::Mid => &mut fit.mods_mid,
        SolModRack::Low => &mut fit.mods_low,
    };
    Ok(fit_rack)
}
