use crate::{
    svc::vast::vast::VastFitData,
    ud::{UFitKey, UItemKey},
    util::RMap,
};

pub(super) struct FDCache<'a> {
    fit_datas: Option<&'a mut RMap<UFitKey, VastFitData>>,
    fit_key: UFitKey,
    cached: Option<&'a mut VastFitData>,
}
impl<'a> FDCache<'a> {
    pub(super) fn new(fit_datas: &'a mut RMap<UFitKey, VastFitData>, fit_key: UItemKey) -> Self {
        Self {
            fit_datas: Some(fit_datas),
            fit_key,
            cached: None,
        }
    }
    pub(super) fn get(&mut self) -> &mut VastFitData {
        self.cached
            .get_or_insert_with(|| self.fit_datas.take().unwrap().get_mut(&self.fit_key).unwrap())
    }
}
