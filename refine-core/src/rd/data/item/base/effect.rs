use crate::{
    Count, PValue, Value,
    ad::{AAttrId, AItemEffectData, AItemId, AItemListId},
    rd::{RAttrId, REffect, REffectProjecteeFilter, RItemListId},
    util::RMap,
};

/// Item-specific attribute-independent effect data
#[derive(Copy, Clone, Default)]
pub(crate) struct RItemBaseEffectData {
    pub(crate) ability_cooldown: Option<PValue>,
    pub(crate) ability_charge_count: Option<Count>,
    pub(crate) ability_charge_reload_duration: PValue,
    pub(crate) autocharge_aid: Option<AItemId>,
    pub(crate) projectee_filter_rid: Option<RItemListId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RItemBaseEffectData {
    pub(super) fn fill_from_a_effect_data(
        &mut self,
        r_effect: &REffect,
        a_effect_data: &AItemEffectData,
        r_item_attrs: &RMap<RAttrId, Value>,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
        item_list_aid_rid_map: &RMap<AItemListId, RItemListId>,
    ) {
        self.ability_cooldown = a_effect_data
            .ability_cooldown
            .map(PValue::from_a_value_clamped)
            .and_then(|v| match v {
                PValue::ZERO => None,
                _ => Some(v),
            });
        self.ability_charge_count = a_effect_data.ability_charge_count.map(Count::from_a_count);
        self.ability_charge_reload_duration = a_effect_data
            .ability_charge_reload_duration
            .map(PValue::from_a_value_clamped)
            .unwrap_or(PValue::ZERO);
        self.autocharge_aid = get_autocharge_aid(a_effect_data.autocharge_attr_id, r_item_attrs, attr_aid_rid_map);
        self.projectee_filter_rid = get_projectee_filter_rid(r_effect, r_item_attrs, item_list_aid_rid_map);
    }
}

fn get_autocharge_aid(
    attr_aid: Option<AAttrId>,
    r_item_attrs: &RMap<RAttrId, Value>,
    attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
) -> Option<AItemId> {
    let attr_rid = attr_aid_rid_map.get(&attr_aid?)?;
    let attr_value = *r_item_attrs.get(attr_rid)?;
    AItemId::try_from_f64_rounded(attr_value.into_f64())
}

fn get_projectee_filter_rid(
    r_effect: &REffect,
    r_item_attrs: &RMap<RAttrId, Value>,
    item_list_aid_rid_map: &RMap<AItemListId, RItemListId>,
) -> Option<RItemListId> {
    match r_effect.projectee_filter {
        Some(REffectProjecteeFilter::ItemList(item_list_rid)) => Some(item_list_rid),
        Some(REffectProjecteeFilter::ItemListAttr(attr_rid)) => {
            let attr_value = *r_item_attrs.get(&attr_rid)?;
            let item_list_aid = AItemListId::try_eve_from_f64_rounded(attr_value.into_f64())?;
            item_list_aid_rid_map.get(&item_list_aid).copied()
        }
        None => None,
    }
}
