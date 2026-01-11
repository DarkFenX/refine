use crate::{
    svc::vast::{Vast, VastFitData},
    ud::UFitId,
};

impl Vast {
    pub(in crate::svc) fn fit_added(&mut self, fit_uid: UFitId) {
        self.fit_datas.insert(fit_uid, VastFitData::new());
    }
    pub(in crate::svc) fn fit_removed(&mut self, fit_uid: &UFitId) {
        self.fit_datas.remove(fit_uid);
    }
}
