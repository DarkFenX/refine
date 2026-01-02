use itertools::Itertools;

use crate::{
    rd::RAttrId,
    sol::SolarSystem,
    svc::{calc::CalcAttrVals, err::KeyedItemLoadedError},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_get_item_attr(
        &mut self,
        item_key: UItemId,
        attr_key: RAttrId,
    ) -> Result<CalcAttrVals, KeyedItemLoadedError> {
        self.svc.get_item_attr_val_full(&self.u_data, item_key, attr_key)
    }
    pub(in crate::api) fn internal_remove_incoming_projections(&mut self, projectee_key: UItemId) {
        let projector_keys = self.rev_projs.iter_projectors(&projectee_key).collect_vec();
        for &projector_key in projector_keys.iter() {
            self.internal_remove_projection(projector_key, projectee_key).unwrap()
        }
    }
}
