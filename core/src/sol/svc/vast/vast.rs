use crate::{
    defs::{SolFitId, SolItemId},
    err::basic::FitFoundError,
    util::{StMap, StSet},
};

// Vast stands for VAlidation and STats.
#[derive(Clone)]
pub(in crate::sol) struct SolVast {
    pub(in crate::sol::svc::vast) fit_datas: StMap<SolFitId, SolVastFitData>,
}
impl SolVast {
    pub(in crate::sol::svc) fn new() -> Self {
        Self {
            fit_datas: StMap::new(),
        }
    }
    pub(in crate::sol::svc::vast) fn get_fit_data(&self, fit_id: &SolFitId) -> Result<&SolVastFitData, FitFoundError> {
        self.fit_datas.get(fit_id).ok_or_else(|| FitFoundError::new(*fit_id))
    }
    pub(in crate::sol::svc::vast) fn get_fit_data_mut(
        &mut self,
        fit_id: &SolFitId,
    ) -> Result<&mut SolVastFitData, FitFoundError> {
        self.fit_datas
            .get_mut(fit_id)
            .ok_or_else(|| FitFoundError::new(*fit_id))
    }
}

#[derive(Clone)]
pub(in crate::sol::svc::vast) struct SolVastFitData {
    // Modules with "online" effect active
    pub(in crate::sol::svc::vast) mods_online: StSet<SolItemId>,
    // Rigs with "rigSlot" effect active
    pub(in crate::sol::svc::vast) rigs_rigslot: StSet<SolItemId>,
}
impl SolVastFitData {
    pub(in crate::sol::svc) fn new() -> Self {
        Self {
            mods_online: StSet::new(),
            rigs_rigslot: StSet::new(),
        }
    }
}
