use crate::{
    Modification, Value,
    rd::{RAttrId, REffectId},
    svc::{
        EffectSpec, Svc, SvcCtx,
        calc::{CalcAttrVals, CalcOp},
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
            .get_item_attr_rfull(&SvcCtx::new(u_data, &self.eff_projs), item_uid, attr_rid)
    }
    pub(crate) fn iter_item_attr_vals(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<impl ExactSizeIterator<Item = (RAttrId, CalcAttrVals)>, UItemLoadedError> {
        self.calc
            .iter_item_attrs_rfull(&SvcCtx::new(u_data, &self.eff_projs), item_uid)
    }
    pub(crate) fn iter_item_mods(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<impl ExactSizeIterator<Item = (RAttrId, Vec<Modification>)>, UItemLoadedError> {
        self.calc
            .iter_item_mods(&SvcCtx::new(u_data, &self.eff_projs), item_uid)
    }
    // Specific request for controllable affector switching
    pub(crate) fn iter_item_attr_mods_op_str(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
        attr_rid: RAttrId,
    ) -> Result<impl ExactSizeIterator<Item = (EffectSpec, CalcOp, Value)>, UItemLoadedError> {
        self.calc
            .iter_item_attr_mods_op_str(&SvcCtx::new(u_data, &self.eff_projs), item_uid, attr_rid)
    }
    // Specific request for side-effects
    pub(crate) fn get_effect_mods_op_str(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
        effect_rid: REffectId,
    ) -> Option<(CalcOp, Value)> {
        self.calc.get_effect_mods_op_str(
            &SvcCtx::new(u_data, &self.eff_projs),
            item_uid,
            u_data.r_data.get_effect_by_rid(effect_rid),
        )
    }
}
