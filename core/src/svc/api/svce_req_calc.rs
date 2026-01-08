use crate::{
    api::AttrId,
    rd::RAttrId,
    svc::{
        Svc, SvcCtx,
        calc::{CalcAttrVals, Modification},
        err::UItemLoadedError,
    },
    ud::{UData, UItemId},
};

impl Svc {
    pub(crate) fn get_item_attr_val_full(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
        attr_rid: RAttrId,
    ) -> Result<CalcAttrVals, UItemLoadedError> {
        self.calc
            .get_item_attr_rfull(SvcCtx::new(u_data, &self.eff_projs), item_uid, attr_rid)
    }
    pub(crate) fn iter_item_attr_vals(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<impl ExactSizeIterator<Item = (RAttrId, CalcAttrVals)>, UItemLoadedError> {
        self.calc
            .iter_item_attrs_rfull(SvcCtx::new(u_data, &self.eff_projs), item_uid)
    }
    pub(crate) fn iter_item_mods(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<Modification>)>, UItemLoadedError> {
        self.calc.iter_item_mods(SvcCtx::new(u_data, &self.eff_projs), item_uid)
    }
}
