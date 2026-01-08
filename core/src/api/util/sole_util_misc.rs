use itertools::Itertools;

use crate::{
    rd::RAttrId,
    sol::SolarSystem,
    svc::{calc::CalcAttrVals, err::UItemLoadedError},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_get_item_attr(
        &mut self,
        item_uid: UItemId,
        attr_rid: RAttrId,
    ) -> Result<CalcAttrVals, UItemLoadedError> {
        self.svc.get_item_attr_val_full(&self.u_data, item_uid, attr_rid)
    }
    pub(in crate::api) fn internal_remove_incoming_projections(&mut self, projectee_uid: UItemId) {
        let projector_uids = self.rev_projs.iter_projectors(&projectee_uid).collect_vec();
        for &projector_uid in projector_uids.iter() {
            self.internal_remove_projection(projector_uid, projectee_uid).unwrap()
        }
    }
}
