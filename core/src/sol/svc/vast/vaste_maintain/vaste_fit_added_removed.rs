use crate::{
    defs::SolFitId,
    sol::svc::vast::{SolVast, SolVastFitData},
};

impl SolVast {
    pub(in crate::sol::svc) fn fit_added(&mut self, fit_id: &SolFitId) {
        self.fit_datas.insert(*fit_id, SolVastFitData::new());
    }
    pub(in crate::sol::svc) fn fit_removed(&mut self, fit_id: &SolFitId) {
        self.fit_datas.remove(fit_id);
    }
}
