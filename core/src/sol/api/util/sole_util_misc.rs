use itertools::Itertools;

use crate::{
    rd::RAttrKey,
    sol::SolarSystem,
    svc::{calc::CalcAttrVal, err::KeyedItemLoadedError},
    ud::UItemKey,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_get_item_attr(
        &mut self,
        item_key: UItemKey,
        attr_key: RAttrKey,
    ) -> Result<CalcAttrVal, KeyedItemLoadedError> {
        self.svc.get_item_attr_val_full(&self.u_data, item_key, attr_key)
    }
    pub(in crate::sol::api) fn internal_remove_incoming_projections(&mut self, projectee_key: UItemKey) {
        let projector_keys = self.rev_projs.iter_projectors(&projectee_key).collect_vec();
        for &projector_key in projector_keys.iter() {
            self.internal_remove_projection(projector_key, projectee_key).unwrap()
        }
    }
}
