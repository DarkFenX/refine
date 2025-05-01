use std::collections::HashMap;

use crate::handler_json::data::{
    CAttrId, CAttrVal, CEffectId, CItemCatId, CItemEffectData, CItemExtras, CItemGrpId, CItemId, CSkillLevel,
};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CItem {
    id: CItemId,
    grp_id: CItemGrpId,
    cat_id: CItemCatId,
    attrs: HashMap<CAttrId, CAttrVal>,
    effect_datas: HashMap<CEffectId, CItemEffectData>,
    defeff_id: Option<CEffectId>,
    srqs: HashMap<CItemId, CSkillLevel>,
    extras: CItemExtras,
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
            srqs: a_item.srqs.iter().map(|(k, v)| (*k, v.get_inner())).collect(),
            extras: (&a_item.extras).into(),
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
            srqs: c_item
                .srqs
                .iter()
                .map(|(k, v)| (*k, rc::ad::ASkillLevel::new(*v)))
                .collect(),
            extras: (&c_item.extras).into(),
        }
    }
}
