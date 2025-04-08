use crate::sol::{
    FitId,
    svc::vast::{Vast, VastFitData},
};

impl Vast {
    pub(in crate::sol::svc) fn fit_added(&mut self, fit_id: FitId) {
        self.fit_datas.insert(fit_id, VastFitData::new());
    }
    pub(in crate::sol::svc) fn fit_removed(&mut self, fit_id: &FitId) {
        self.fit_datas.remove(fit_id);
    }
}
