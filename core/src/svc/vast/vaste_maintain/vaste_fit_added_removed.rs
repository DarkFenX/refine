use crate::{
    svc::vast::{Vast, VastFitData},
    uad::UadFitKey,
};

impl Vast {
    pub(in crate::svc) fn fit_added(&mut self, fit_key: UadFitKey) {
        self.fit_datas.insert(fit_key, VastFitData::new());
    }
    pub(in crate::svc) fn fit_removed(&mut self, fit_key: &UadFitKey) {
        self.fit_datas.remove(fit_key);
    }
}
