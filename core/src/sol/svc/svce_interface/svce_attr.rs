use crate::{
    defs::{EAttrId, SolItemId},
    sol::{svc::SolSvc, uad::SolUad},
};

impl SolSvc {
    pub(in crate::sol) fn item_base_attr_value_changed(
        &mut self,
        uad: &SolUad,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) {
        // Go through calculator, because if value wasn't calculated and cached - nobody needs to be
        // notified of the change
        self.calc.force_attr_value_recalc(uad, item_id, attr_id)
    }
    pub(in crate::sol) fn item_attr_postprocess_changed(
        &mut self,
        uad: &SolUad,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) {
        // Go through calculator, because if value wasn't calculated and cached - nobody needs to be
        // notified of the change
        self.calc.force_attr_postprocess_recalc(uad, item_id, attr_id)
    }
}
