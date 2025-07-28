use std::collections::{HashMap, HashSet};

use crate::cacher_json::data::{
    CAbilId, CAttrId, CAttrVal, CEffectId, CItemCatId, CItemEffectData, CItemGrpId, CItemId, CSkillLevel, CState,
};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json) struct CItem {
    id: CItemId,
    grp_id: CItemGrpId,
    cat_id: CItemCatId,
    attrs: HashMap<CAttrId, CAttrVal>,
    effect_datas: HashMap<CEffectId, CItemEffectData>,
    defeff_id: Option<CEffectId>,
    abil_ids: HashSet<CAbilId>,
    srqs: HashMap<CItemId, CSkillLevel>,
    max_state: CState,
    val_fitted_group_id: Option<CItemGrpId>,
    val_online_group_id: Option<CItemGrpId>,
    val_active_group_id: Option<CItemGrpId>,
    disallowed_in_wspace: bool,
}
impl From<&rc::ad::AItem> for CItem {
    fn from(a_item: &rc::ad::AItem) -> Self {
        Self {
            id: a_item.id,
            grp_id: a_item.grp_id,
            cat_id: a_item.cat_id,
            attrs: (&a_item.attrs).into(),
            effect_datas: a_item.effect_datas.iter().map(|(k, v)| (k.into(), v.into())).collect(),
            defeff_id: a_item.defeff_id.as_ref().map(|v| v.into()),
            abil_ids: a_item.abil_ids.iter().copied().collect(),
            srqs: a_item.srqs.iter().map(|(k, v)| (*k, v.get_inner())).collect(),
            max_state: (&a_item.max_state).into(),
            val_fitted_group_id: a_item.val_fitted_group_id,
            val_online_group_id: a_item.val_online_group_id,
            val_active_group_id: a_item.val_active_group_id,
            disallowed_in_wspace: a_item.disallowed_in_wspace,
        }
    }
}
impl From<&CItem> for rc::ad::AItem {
    fn from(c_item: &CItem) -> Self {
        Self {
            id: c_item.id,
            grp_id: c_item.grp_id,
            cat_id: c_item.cat_id,
            attrs: (&c_item.attrs).into(),
            effect_datas: c_item.effect_datas.iter().map(|(k, v)| (k.into(), v.into())).collect(),
            defeff_id: c_item.defeff_id.as_ref().map(|v| v.into()),
            abil_ids: c_item.abil_ids.iter().copied().collect(),
            srqs: c_item
                .srqs
                .iter()
                .map(|(k, v)| (*k, rc::ad::ASkillLevel::new(*v)))
                .collect(),
            max_state: (&c_item.max_state).into(),
            val_fitted_group_id: c_item.val_fitted_group_id,
            val_online_group_id: c_item.val_online_group_id,
            val_active_group_id: c_item.val_active_group_id,
            disallowed_in_wspace: c_item.disallowed_in_wspace,
        }
    }
}
