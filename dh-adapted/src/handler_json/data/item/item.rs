use std::collections::HashMap;

use crate::handler_json::data::{CItemEffectData, CItemExtras};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CItem {
    id: rc::EItemId,
    grp_id: rc::EItemGrpId,
    cat_id: rc::EItemCatId,
    attrs: HashMap<rc::EAttrId, rc::AttrVal>,
    effect_datas: HashMap<rc::EEffectId, CItemEffectData>,
    defeff_id: Option<rc::EEffectId>,
    srqs: HashMap<rc::EItemId, rc::SkillLevel>,
    extras: CItemExtras,
}
impl From<&rc::ad::AItem> for CItem {
    fn from(a_item: &rc::ad::AItem) -> Self {
        Self {
            id: a_item.id,
            grp_id: a_item.grp_id,
            cat_id: a_item.cat_id,
            attrs: (&a_item.attrs).into(),
            effect_datas: a_item.effect_datas.iter().map(|(k, v)| (*k, v.into())).collect(),
            defeff_id: a_item.defeff_id,
            srqs: (&a_item.srqs).into(),
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
            effect_datas: c_item.effect_datas.iter().map(|(k, v)| (*k, v.into())).collect(),
            defeff_id: c_item.defeff_id,
            srqs: (&c_item.srqs).into(),
            extras: (&c_item.extras).into(),
        }
    }
}
