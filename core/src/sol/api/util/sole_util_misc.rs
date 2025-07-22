use itertools::Itertools;

use crate::{
    ad,
    sol::SolarSystem,
    svc::{calc::CalcAttrVal, err::KeyedItemLoadedError},
    uad::UadItemKey,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_get_item_attr(
        &mut self,
        item_key: UadItemKey,
        a_attr_id: &ad::AAttrId,
    ) -> Result<CalcAttrVal, KeyedItemLoadedError> {
        self.svc.get_item_attr_val_full(&self.uad, item_key, a_attr_id)
    }
    pub(in crate::sol::api) fn internal_remove_incoming_projections(&mut self, projectee_key: UadItemKey) {
        let projector_keys = self.rprojs.iter_projectors(&projectee_key).copied().collect_vec();
        for &projector_key in projector_keys.iter() {
            self.internal_remove_projection(projector_key, projectee_key).unwrap()
        }
    }
}
