use super::getters::{
    has_effect::{has_launcher_effect, has_online_effect, has_turret_effect},
    ship_kind::get_ship_kind,
};
use crate::{
    SkillLevel,
    ad::{AAbilId, AAttrId, AEffectId, AItem, AItemCatId, AItemGrpId, AItemId, AItemListId},
    rd::{RAttrId, REffectId, RItemCapConsumer, RItemEffectData, RItemListId, RShipKind, RState, RcEffect},
    util::{PSlab, RMap},
};

// Item base data - the data which does not depend on attributes.
pub(crate) struct RItemBase {
    // Raw data
    pub(crate) aid: AItemId,
    pub(crate) grp_id: AItemGrpId,
    pub(crate) cat_id: AItemCatId,
    pub(crate) effects: RMap<REffectId, RItemEffectData>,
    pub(crate) defeff_rid: Option<REffectId>,
    pub(crate) abil_ids: Vec<AAbilId>,
    pub(crate) srqs: RMap<AItemId, SkillLevel>,
    // Derived data - item type flags
    pub(crate) is_cloak: bool,
    pub(crate) is_ice_harvester: bool,
    pub(crate) enables_portal: bool, // Used by ansiblex service
    // Derived data - effect flags
    pub(crate) has_online_effect: bool,
    pub(crate) takes_turret_hardpoint: bool,
    pub(crate) takes_launcher_hardpoint: bool,
    // Derived data - max group ID validation data
    pub(crate) val_fitted_group_id: Option<AItemGrpId>,
    pub(crate) val_online_group_id: Option<AItemGrpId>,
    pub(crate) val_active_group_id: Option<AItemGrpId>,
    // Derived data - buff item list IDs. Those are intentionally vectors: they are used for
    // iteration, and for membership checks. For cases where it matters (e.g. non-ship items) those
    // do not have more than a couple of entries, so are faster than sets.
    pub(crate) proj_buff_item_list_rids: Vec<RItemListId>,
    pub(crate) fleet_buff_item_list_rids: Vec<RItemListId>,
    // Derived data - misc
    pub(crate) max_state: RState,
    pub(crate) cap_consumers: Vec<RItemCapConsumer>,
    pub(crate) ship_kind: Option<RShipKind>,
    pub(crate) disallowed_in_wspace: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RItemBase {
    pub(in crate::rd::data::item) fn from_a_item(a_item: &AItem) -> Self {
        Self {
            aid: a_item.id,
            grp_id: a_item.grp_id,
            cat_id: a_item.cat_id,
            abil_ids: a_item.abil_ids.iter().copied().collect(),
            srqs: a_item
                .srqs
                .iter()
                .map(|a_skill_req| (a_skill_req.id, SkillLevel::from_a_skill_level(a_skill_req.level)))
                .collect(),
            is_cloak: a_item.is_cloak,
            is_ice_harvester: a_item.is_ice_harvester,
            enables_portal: a_item.enables_portal,
            val_fitted_group_id: a_item.val_fitted_group_id,
            val_online_group_id: a_item.val_online_group_id,
            val_active_group_id: a_item.val_active_group_id,
            max_state: RState::from_a_state(&a_item.max_state),
            disallowed_in_wspace: a_item.disallowed_in_wspace,
            // Fields which depend on data not available during instantiation
            effects: Default::default(),
            defeff_rid: Default::default(),
            has_online_effect: Default::default(),
            takes_turret_hardpoint: Default::default(),
            takes_launcher_hardpoint: Default::default(),
            proj_buff_item_list_rids: Default::default(),
            fleet_buff_item_list_rids: Default::default(),
            cap_consumers: Default::default(),
            ship_kind: Default::default(),
        }
    }
    pub(in crate::rd::data::item) fn fill_runtime(
        &mut self,
        a_items: &RMap<AItemId, AItem>,
        item_list_aid_rid_map: &RMap<AItemListId, RItemListId>,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
        effect_aid_rid_map: &RMap<AEffectId, REffectId>,
        r_effects: &PSlab<REffectId, RcEffect>,
    ) {
        let a_item = a_items.get(&self.aid).unwrap();
        for a_item_effect in a_item.effects.iter() {
            let Some(&effect_rid) = effect_aid_rid_map.get(&a_item_effect.id) else {
                continue;
            };
            let r_effect_data = RItemEffectData::from_a_effect_data(&a_item_effect.data, attr_aid_rid_map);
            self.effects.insert(effect_rid, r_effect_data);
        }
        self.defeff_rid = a_item
            .defeff_id
            .and_then(|defeff_aid| effect_aid_rid_map.get(&defeff_aid).copied());
        self.has_online_effect = has_online_effect(&self.effects, effect_aid_rid_map);
        self.takes_turret_hardpoint = has_turret_effect(&self.effects, effect_aid_rid_map);
        self.takes_launcher_hardpoint = has_launcher_effect(&self.effects, effect_aid_rid_map);
        self.proj_buff_item_list_rids.extend(
            a_item
                .proj_buff_item_list_ids
                .iter()
                .filter_map(|item_list_aid| item_list_aid_rid_map.get(item_list_aid).copied()),
        );
        self.fleet_buff_item_list_rids.extend(
            a_item
                .fleet_buff_item_list_ids
                .iter()
                .filter_map(|item_list_aid| item_list_aid_rid_map.get(item_list_aid).copied()),
        );

        for &effect_rid in self.effects.keys() {
            let r_effect = r_effects.get(effect_rid).unwrap();
            if let Some(opc_spec) = r_effect.cap_consume {
                self.cap_consumers.push(RItemCapConsumer { effect_rid, opc_spec })
            }
        }
        self.ship_kind = get_ship_kind(self.cat_id, &self.srqs);
    }
}
