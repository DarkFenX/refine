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
        // Go through calculator, because if value wasn't calculated and cached - nobody needs to be
        // notified of the change
        self.calc_force_attr_value_recalc(sol_view, item_id, attr_id)
    }
    pub(in crate::sol) fn item_attr_postprocess_changed(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) {
        // Go through calculator, because if value wasn't calculated and cached - nobody needs to be
        // notified of the change
        self.calc_force_attr_postprocess_recalc(sol_view, item_id, attr_id)
    }
}
