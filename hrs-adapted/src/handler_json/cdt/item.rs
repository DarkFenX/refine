use std::collections::HashMap;

use super::enums::ItemType;

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json::cdt) struct Item {
    id: rc::ReeInt,
    itype: Option<ItemType>,
    grp_id: rc::ReeInt,
    cat_id: rc::ReeInt,
    attr_vals: HashMap<rc::ReeInt, rc::ReeFloat>,
    effect_datas: HashMap<rc::ReeInt, ItemEffData>,
    defeff_id: Option<rc::ReeInt>,
    srqs: HashMap<rc::ReeInt, rc::ReeInt>,
}
impl From<rc::adt::Item> for Item {
    fn from(value: rc::adt::Item) -> Self {
        Item {
            id: value.id,
            itype: value.itype.map(|v| v.into()),
            grp_id: value.grp_id,
            cat_id: value.cat_id,
            attr_vals: value.attr_vals,
            effect_datas: value.effect_datas.iter().map(|(k, v)| (*k, (*v).into())).collect(),
            defeff_id: value.defeff_id,
            srqs: value.srqs,
        }
    }
}
impl Into<rc::adt::Item> for Item {
    fn into(self) -> rc::adt::Item {
        rc::adt::Item {
            id: self.id,
            itype: self.itype.map(|v| v.into()),
            grp_id: self.grp_id,
            cat_id: self.cat_id,
            attr_vals: self.attr_vals,
            effect_datas: self.effect_datas.iter().map(|(k, v)| (*k, (*v).into())).collect(),
            defeff_id: self.defeff_id,
            srqs: self.srqs,
        }
    }
}

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
struct ItemEffData {
    cd: Option<rc::ReeFloat>,
    charge_amount: Option<rc::ReeInt>,
    charge_reload_time: Option<rc::ReeFloat>,
}
impl From<rc::adt::ItemEffData> for ItemEffData {
    fn from(value: rc::adt::ItemEffData) -> Self {
        ItemEffData {
            cd: value.cd,
            charge_amount: value.charge_amount,
            charge_reload_time: value.charge_reload_time,
        }
    }
}
impl Into<rc::adt::ItemEffData> for ItemEffData {
    fn into(self) -> rc::adt::ItemEffData {
        rc::adt::ItemEffData {
            cd: self.cd,
            charge_amount: self.charge_amount,
            charge_reload_time: self.charge_reload_time,
        }
    }
}
