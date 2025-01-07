use crate::{
    defs::{SolFitId, SolItemId},
    err::basic::FitFoundError,
    util::{StMap, StSet},
};

// Rest stands for REstrictions and STats.
#[derive(Clone)]
pub(in crate::sol::svc) struct SolRest {
    pub(in crate::sol::svc::rest) data: StMap<SolFitId, SolRestFitData>,
}
impl SolRest {
    pub(in crate::sol::svc) fn new() -> Self {
        Self { data: StMap::new() }
    }
    pub(in crate::sol::svc::rest) fn add_fit(&mut self, fit_id: SolFitId) {
        self.data.insert(fit_id, SolRestFitData::new());
    }
    pub(in crate::sol::svc::rest) fn remove_fit(&mut self, fit_id: &SolFitId) {
        self.data.remove(fit_id);
    }
    pub(in crate::sol::svc::rest) fn get_data(&self, fit_id: &SolFitId) -> Result<&SolRestFitData, FitFoundError> {
        self.data.get(fit_id).ok_or_else(|| FitFoundError::new(*fit_id))
    }
    pub(in crate::sol::svc::rest) fn get_data_mut(
        &mut self,
        fit_id: &SolFitId,
    ) -> Result<&mut SolRestFitData, FitFoundError> {
        self.data.get_mut(fit_id).ok_or_else(|| FitFoundError::new(*fit_id))
    }
}

#[derive(Clone)]
pub(in crate::sol::svc::rest) struct SolRestFitData {
    pub(in crate::sol::svc::rest) mods_online: StSet<SolItemId>,
}
impl SolRestFitData {
    pub(in crate::sol::svc) fn new() -> Self {
        Self {
            mods_online: StSet::new(),
        }
    }
}
