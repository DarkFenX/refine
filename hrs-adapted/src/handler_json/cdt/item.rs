use std::collections::HashMap;

use super::enums::ItemType;

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct Item {
    id: rc::ReeInt,
    itype: Option<ItemType>,
    grp_id: rc::ReeInt,
    cat_id: rc::ReeInt,
    attr_vals: HashMap<rc::ReeInt, rc::ReeFloat>,
    effect_datas: HashMap<rc::ReeInt, ItemEffData>,
    defeff_id: Option<rc::ReeInt>,
    srqs: HashMap<rc::ReeInt, rc::ReeInt>,
}
impl From<&rc::adt::AItem> for Item {
    fn from(value: &rc::adt::AItem) -> Self {
        Item {
            id: value.id,
            itype: value.itype.as_ref().map(|v| v.into()),
            grp_id: value.grp_id,
            cat_id: value.cat_id,
            attr_vals: value.attr_vals.clone(),
            effect_datas: value.effect_datas.iter().map(|(k, v)| (*k, v.into())).collect(),
            defeff_id: value.defeff_id,
            srqs: value.srqs.clone(),
        }
    }
}
impl Into<rc::adt::AItem> for &Item {
    fn into(self) -> rc::adt::AItem {
        rc::adt::AItem {
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
struct ItemEffData {
    cd: Option<rc::ReeFloat>,
    charge_amount: Option<rc::ReeInt>,
    charge_reload_time: Option<rc::ReeFloat>,
}
impl From<&rc::adt::AItemEffData> for ItemEffData {
    fn from(value: &rc::adt::AItemEffData) -> Self {
        ItemEffData {
            cd: value.cd,
            charge_amount: value.charge_amount,
            charge_reload_time: value.charge_reload_time,
        }
    }
}
impl Into<rc::adt::AItemEffData> for &ItemEffData {
    fn into(self) -> rc::adt::AItemEffData {
        rc::adt::AItemEffData {
            cd: self.cd,
            charge_amount: self.charge_amount,
            charge_reload_time: self.charge_reload_time,
        }
    }
}
