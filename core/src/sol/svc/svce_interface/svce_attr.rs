use crate::{
    defs::{EAttrId, SolItemId},
    sol::{svc::SolSvcs, SolView},
};

impl SolSvcs {
    pub(in crate::sol) fn item_attr_value_changed(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) {
        self.calc_force_attr_recalc(sol_view, item_id, attr_id)
    }
}
