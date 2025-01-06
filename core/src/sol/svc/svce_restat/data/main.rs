use crate::{
    defs::SolFitId, err::basic::FitFoundError, sol::svc::svce_restat::reg_mods_online::SolRestatRegModsOnline,
    util::StMap,
};

#[derive(Clone)]
pub(in crate::sol::svc) struct SolSvcRestatData {
    pub(in crate::sol::svc::svce_restat) data: StMap<SolFitId, SolSvcRestatFitData>,
}
impl SolSvcRestatData {
    pub(in crate::sol::svc) fn new() -> Self {
        Self { data: StMap::new() }
    }
    pub(in crate::sol::svc::svce_restat) fn add_fit(&mut self, fit_id: SolFitId) {
        self.data.insert(fit_id, SolSvcRestatFitData::new());
    }
    pub(in crate::sol::svc::svce_restat) fn remove_fit(&mut self, fit_id: &SolFitId) {
        self.data.remove(fit_id);
    }
    pub(in crate::sol::svc::svce_restat) fn get_data(
        &self,
        fit_id: &SolFitId,
    ) -> Result<&SolSvcRestatFitData, FitFoundError> {
        self.data.get(fit_id).ok_or_else(|| FitFoundError::new(*fit_id))
    }
    pub(in crate::sol::svc::svce_restat) fn get_data_mut(
        &mut self,
        fit_id: &SolFitId,
    ) -> Result<&mut SolSvcRestatFitData, FitFoundError> {
        self.data.get_mut(fit_id).ok_or_else(|| FitFoundError::new(*fit_id))
    }
}

#[derive(Clone)]
pub(in crate::sol::svc::svce_restat) struct SolSvcRestatFitData {
    pub(in crate::sol::svc::svce_restat) mods_online: SolRestatRegModsOnline,
}
impl SolSvcRestatFitData {
    pub(in crate::sol::svc) fn new() -> Self {
        Self {
            mods_online: SolRestatRegModsOnline::new(),
        }
    }
}
