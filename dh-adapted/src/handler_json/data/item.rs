use std::collections::HashMap;

use super::enums::CItemType;

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CItem {
    id: rc::ReeInt,
    itype: Option<CItemType>,
    grp_id: rc::ReeInt,
    cat_id: rc::ReeInt,
    attr_vals: HashMap<rc::ReeInt, rc::ReeFloat>,
    effect_datas: HashMap<rc::ReeInt, CItemEffData>,
    defeff_id: Option<rc::ReeInt>,
    srqs: HashMap<rc::ReeInt, rc::ReeInt>,
}
impl From<&rc::ad::AItem> for CItem {
    fn from(a_item: &rc::ad::AItem) -> Self {
        CItem {
            id: a_item.id,
            itype: a_item.itype.as_ref().map(|v| v.into()),
            grp_id: a_item.grp_id,
            cat_id: a_item.cat_id,
            attr_vals: a_item.attr_vals.clone(),
            effect_datas: a_item.effect_datas.iter().map(|(k, v)| (*k, v.into())).collect(),
            defeff_id: a_item.defeff_id,
            srqs: a_item.srqs.clone(),
        }
    }
}
impl Into<rc::ad::AItem> for &CItem {
    fn into(self) -> rc::ad::AItem {
        rc::ad::AItem {
            id: self.id,
            itype: self.itype.as_ref().map(|v| v.into()),
            grp_id: self.grp_id,
            cat_id: self.cat_id,
            attr_vals: self.attr_vals.clone(),
            effect_datas: self.effect_datas.iter().map(|(k, v)| (*k, v.into())).collect(),
            defeff_id: self.defeff_id,
            srqs: self.srqs.clone(),
        }
    }
}

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
struct CItemEffData {
    cd: Option<rc::ReeFloat>,
    charge_amount: Option<rc::ReeInt>,
    charge_reload_time: Option<rc::ReeFloat>,
}
impl From<&rc::ad::AItemEffData> for CItemEffData {
    fn from(a_item_eff_data: &rc::ad::AItemEffData) -> Self {
        CItemEffData {
            cd: a_item_eff_data.cd,
            charge_amount: a_item_eff_data.charge_amount,
            charge_reload_time: a_item_eff_data.charge_reload_time,
        }
    }
}
impl Into<rc::ad::AItemEffData> for &CItemEffData {
    fn into(self) -> rc::ad::AItemEffData {
        rc::ad::AItemEffData {
            cd: self.cd,
            charge_amount: self.charge_amount,
            charge_reload_time: self.charge_reload_time,
        }
    }
}
