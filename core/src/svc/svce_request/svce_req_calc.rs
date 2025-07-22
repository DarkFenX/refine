use crate::{
    ad,
    svc::{
        Svc, SvcCtx,
        calc::{CalcAttrVal, ModificationInfo},
        err::KeyedItemLoadedError,
    },
    uad::{Uad, UadItemKey},
};

impl Svc {
    pub(crate) fn get_item_attr_val_full(
        &mut self,
        uad: &Uad,
        item_key: UadItemKey,
        a_attr_id: &ad::AAttrId,
    ) -> Result<CalcAttrVal, KeyedItemLoadedError> {
        self.calc
            .get_item_attr_val_full(SvcCtx::new(uad, &self.eprojs), item_key, a_attr_id)
    }
    pub(crate) fn iter_item_attr_vals(
        &mut self,
        uad: &Uad,
        item_key: UadItemKey,
    ) -> Result<impl ExactSizeIterator<Item = (ad::AAttrId, CalcAttrVal)>, KeyedItemLoadedError> {
        self.calc.iter_item_attr_vals(SvcCtx::new(uad, &self.eprojs), item_key)
    }
    pub(crate) fn iter_item_mods(
        &mut self,
        uad: &Uad,
        item_key: UadItemKey,
    ) -> Result<impl ExactSizeIterator<Item = (ad::AAttrId, Vec<ModificationInfo>)>, KeyedItemLoadedError> {
        self.calc.iter_item_mods(SvcCtx::new(uad, &self.eprojs), item_key)
    }
}
