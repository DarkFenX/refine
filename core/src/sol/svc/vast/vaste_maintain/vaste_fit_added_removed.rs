use crate::sol::{
    FitKey,
    svc::vast::{Vast, VastFitData},
};

impl Vast {
    pub(in crate::sol::svc) fn fit_added(&mut self, fit_key: FitKey) {
        self.fit_datas.insert(fit_key, VastFitData::new());
    }
    pub(in crate::sol::svc) fn fit_removed(&mut self, fit_key: &FitKey) {
        self.fit_datas.remove(fit_key);
    }
}
