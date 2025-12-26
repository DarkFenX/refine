use crate::{
    api::AttrId,
    rd::RAttrKey,
    svc::{
        Svc, SvcCtx,
        calc::{CalcAttrVals, Modification},
        err::KeyedItemLoadedError,
    },
    ud::{UData, UItemKey},
};

impl Svc {
    pub(crate) fn get_item_attr_val_full(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
        attr_key: RAttrKey,
    ) -> Result<CalcAttrVals, KeyedItemLoadedError> {
        self.calc
            .get_item_attr_rfull(SvcCtx::new(u_data, &self.eff_projs), item_key, attr_key)
    }
    pub(crate) fn iter_item_attr_vals(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<impl ExactSizeIterator<Item = (RAttrKey, CalcAttrVals)>, KeyedItemLoadedError> {
        self.calc
            .iter_item_attrs_rfull(SvcCtx::new(u_data, &self.eff_projs), item_key)
    }
    pub(crate) fn iter_item_mods(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<Modification>)>, KeyedItemLoadedError> {
        self.calc.iter_item_mods(SvcCtx::new(u_data, &self.eff_projs), item_key)
    }
}
