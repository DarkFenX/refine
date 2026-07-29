use crate::{
    Value,
    ad::{AItemId, AItemListId},
    rd::{RAttrId, REffectId, REffectProjecteeFilter, RItemEffectData, RItemListId, RcEffect},
    util::{PSlab, RMap},
};

// Item-specific attribute-derived effect data
#[derive(Copy, Clone)]
pub(crate) struct RItemAttrEffectData {
    pub(crate) autocharge_aid: Option<AItemId>,
    pub(crate) projectee_filter_rid: Option<RItemListId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RItemAttrEffectData {
    pub(in crate::rd::data::item) fn try_from_r_effect_data(
        r_effect_data: &RItemEffectData,
        item_attrs: &RMap<RAttrId, Value>,
        effect_rid: REffectId,
        item_list_aid_rid_map: &RMap<AItemListId, RItemListId>,
        r_effects: &PSlab<REffectId, RcEffect>,
    ) -> Option<Self> {
        let autocharge_aid = get_autocharge_aid(r_effect_data.autocharge_attr_rid, item_attrs);
        let projectee_filter_rid = match r_effects.get(effect_rid).unwrap().projectee_filter {
            Some(REffectProjecteeFilter::ItemList(item_list_rid)) => Some(item_list_rid),
            Some(REffectProjecteeFilter::ItemListAttr(attr_rid)) => {
                get_item_list_rid(attr_rid, item_attrs, item_list_aid_rid_map)
            }
            None => None,
        };
        match (autocharge_aid, projectee_filter_rid) {
            (None, None) => None,
            _ => Some(Self {
                autocharge_aid,
                projectee_filter_rid,
            }),
        }
    }
}

fn get_autocharge_aid(attr_rid: Option<RAttrId>, item_attrs: &RMap<RAttrId, Value>) -> Option<AItemId> {
    let attr_rid = attr_rid?;
    let attr_value = *item_attrs.get(&attr_rid)?;
    AItemId::try_from_f64_rounded(attr_value.into_f64())
}

fn get_item_list_rid(
    attr_rid: RAttrId,
    item_attrs: &RMap<RAttrId, Value>,
    item_list_aid_rid_map: &RMap<AItemListId, RItemListId>,
) -> Option<RItemListId> {
    let attr_value = *item_attrs.get(&attr_rid)?;
    let item_list_aid = AItemListId::try_eve_from_f64_rounded(attr_value.into_f64())?;
    item_list_aid_rid_map.get(&item_list_aid).copied()
}
