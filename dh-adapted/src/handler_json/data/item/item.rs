use std::collections::HashMap;

use crate::handler_json::data::{CItemEffData, CItemExtras};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CItem {
    id: rc::EItemId,
    grp_id: rc::EItemGrpId,
    cat_id: rc::EItemCatId,
    attrs: HashMap<rc::EAttrId, rc::AttrVal>,
    effect_datas: HashMap<rc::EEffectId, CItemEffData>,
    defeff_id: Option<rc::EEffectId>,
    srqs: HashMap<rc::EItemId, rc::SkillLevel>,
    extras: CItemExtras,
}
impl From<&rc::ad::AItem> for CItem {
    fn from(a_item: &rc::ad::AItem) -> Self {
        CItem {
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
impl Into<rc::ad::AItem> for &CItem {
    fn into(self) -> rc::ad::AItem {
        rc::ad::AItem {
            id: self.id,
            grp_id: self.grp_id,
            cat_id: self.cat_id,
            attrs: (&self.attrs).into(),
            effect_datas: self.effect_datas.iter().map(|(k, v)| (*k, v.into())).collect(),
            defeff_id: self.defeff_id,
            srqs: (&self.srqs).into(),
            extras: (&self.extras).into(),
        }
    }
}
