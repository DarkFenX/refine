use crate::{
    ad::{
        AAbilId, AAttrId, AAttrVal, AEffectId, AItemCatId, AItemEffectData, AItemGrpId, AItemId, ASkillLevel, AState,
    },
    def::ItemId,
    misc::EffectMode,
    rd::{REffectKey, RItemAXt, RShipKind, RcItem},
    src::Src,
    ud::item::{
        base::{UEffectUpdates, process_effects},
        misc::EffectModes,
    },
    util::{GetId, RMap, RSet},
};

// Item base stores all the data every item should have
#[derive(Clone)]
pub(in crate::ud::item) struct UItemBase {
    // User-defined data
    item_id: ItemId,
    type_id: AItemId,
    state: AState,
    pub(super) effect_modes: EffectModes,
    // Source-dependent data
    cache: Option<ItemBaseCache>,
}
impl UItemBase {
    // Constructors
    pub(in crate::ud::item) fn new(item_id: ItemId, type_id: AItemId, state: AState, src: &Src) -> Self {
        Self {
            item_id,
            type_id,
            state,
            effect_modes: EffectModes::new(),
            cache: src.get_item(&type_id).map(|r_item| ItemBaseCache {
                r_item: r_item.clone(),
                reffs: RSet::new(),
            }),
        }
    }
    pub(in crate::ud::item::base) fn base_new_with_type_id_not_loaded(
        item_id: ItemId,
        type_id: AItemId,
        state: AState,
    ) -> Self {
        Self {
            item_id,
            type_id,
            state,
            effect_modes: EffectModes::new(),
            cache: None,
        }
    }
    pub(in crate::ud::item::base) fn base_new_with_r_item(item_id: ItemId, r_item: RcItem, state: AState) -> Self {
        Self {
            item_id,
            type_id: r_item.get_id(),
            state,
            effect_modes: EffectModes::new(),
            cache: Some(ItemBaseCache {
                r_item,
                reffs: RSet::new(),
            }),
        }
    }
    // Basic data access methods
    pub(in crate::ud::item) fn get_item_id(&self) -> ItemId {
        self.item_id
    }
    pub(in crate::ud::item) fn get_type_id(&self) -> AItemId {
        self.type_id
    }
    pub(in crate::ud::item) fn set_type_id(&mut self, type_id: AItemId, src: &Src) {
        self.type_id = type_id;
        self.base_update_r_data(src);
    }
    pub(in crate::ud::item) fn get_group_id(&self) -> Option<AItemGrpId> {
        self.base_get_r_item().map(|v| v.get_group_id())
    }
    pub(in crate::ud::item) fn get_category_id(&self) -> Option<AItemCatId> {
        self.base_get_r_item().map(|v| v.get_category_id())
    }
    pub(in crate::ud::item) fn get_attrs(&self) -> Option<&RMap<AAttrId, AAttrVal>> {
        self.base_get_r_item().map(|v| v.get_attrs())
    }
    pub(in crate::ud::item) fn get_effect_datas(&self) -> Option<&RMap<REffectKey, AItemEffectData>> {
        self.base_get_r_item().map(|v| v.get_effect_datas())
    }
    pub(in crate::ud::item) fn get_defeff_key(&self) -> Option<Option<REffectKey>> {
        self.base_get_r_item().map(|v| v.get_defeff_key())
    }
    pub(in crate::ud::item) fn get_abils(&self) -> Option<&Vec<AAbilId>> {
        self.base_get_r_item().map(|v| v.get_abils())
    }
    pub(in crate::ud::item) fn get_skill_reqs(&self) -> Option<&RMap<AItemId, ASkillLevel>> {
        self.base_get_r_item().map(|v| v.get_srqs())
    }
    // Extra data access methods
    pub(in crate::ud::item) fn get_axt(&self) -> Option<&RItemAXt> {
        self.base_get_r_item().map(|v| v.get_axt())
    }
    pub(in crate::ud::item) fn get_disallowed_in_wspace(&self) -> Option<bool> {
        self.base_get_r_item().map(|v| v.is_disallowed_in_wspace())
    }
    pub(in crate::ud::item) fn get_val_fitted_group_id(&self) -> Option<AItemGrpId> {
        self.base_get_r_item().and_then(|v| v.get_val_fitted_group_id())
    }
    pub(in crate::ud::item) fn get_val_online_group_id(&self) -> Option<AItemGrpId> {
        self.base_get_r_item().and_then(|v| v.get_val_online_group_id())
    }
    pub(in crate::ud::item) fn get_r_ship_kind(&self) -> Option<RShipKind> {
        self.base_get_r_item().and_then(|v| v.get_ship_kind())
    }
    pub(in crate::ud::item) fn takes_turret_hardpoint(&self) -> bool {
        match self.base_get_r_item() {
            Some(r_item) => r_item.takes_turret_hardpoint(),
            None => false,
        }
    }
    pub(in crate::ud::item) fn takes_launcher_hardpoint(&self) -> bool {
        match self.base_get_r_item() {
            Some(r_item) => r_item.takes_launcher_hardpoint(),
            None => false,
        }
    }
    // Misc methods
    pub(in crate::ud::item) fn get_state(&self) -> AState {
        self.state
    }
    pub(in crate::ud::item) fn set_state(&mut self, state: AState) {
        self.state = state;
    }
    pub(in crate::ud::item) fn get_effect_key_mode(&self, effect_key: &REffectKey) -> EffectMode {
        self.effect_modes.get_by_key(effect_key)
    }
    pub(in crate::ud::item) fn get_effect_id_mode(&self, effect_id: &AEffectId) -> EffectMode {
        self.effect_modes.get_by_id(effect_id)
    }
    pub(in crate::ud::item) fn set_effect_mode(&mut self, effect_id: AEffectId, effect_mode: EffectMode, src: &Src) {
        self.effect_modes.set_by_id(effect_id, effect_mode, src);
    }
    pub(in crate::ud::item) fn set_effect_modes(
        &mut self,
        effect_modes: impl Iterator<Item = (AEffectId, EffectMode)>,
        src: &Src,
    ) {
        for (effect_id, effect_mode) in effect_modes {
            self.effect_modes.set_by_id(effect_id, effect_mode, src);
        }
    }
    pub(in crate::ud::item::base) fn base_update_effect_modes(&mut self, src: &Src) {
        self.effect_modes.update_keys(src);
    }
    pub(in crate::ud::item) fn is_loaded(&self) -> bool {
        self.cache.is_some()
    }
    pub(in crate::ud::item) fn src_changed(&mut self, src: &Src) {
        self.base_update_effect_modes(src);
        self.base_update_r_data(src);
    }
    pub(in crate::ud::item::base) fn base_update_r_data(&mut self, src: &Src) {
        match src.get_item(&self.type_id) {
            Some(r_item) => self.base_set_r_item(r_item.clone()),
            None => self.cache = None,
        }
    }
    // Non-public methods
    pub(in crate::ud::item::base) fn base_set_type_id_primitive(&mut self, type_id: AItemId) {
        self.type_id = type_id;
    }
    pub(in crate::ud::item::base) fn base_set_type_id_not_loaded(&mut self, type_id: AItemId) {
        self.type_id = type_id;
        self.cache = None;
    }
    pub(in crate::ud::item::base) fn base_set_r_item(&mut self, r_item: RcItem) {
        self.type_id = r_item.get_id();
        match &mut self.cache {
            Some(cache) => {
                cache.r_item = r_item;
            }
            None => {
                self.cache = Some(ItemBaseCache {
                    r_item,
                    reffs: RSet::new(),
                })
            }
        }
    }
    pub(in crate::ud::item::base) fn base_get_r_item(&self) -> Option<&RcItem> {
        self.cache.as_ref().map(|v| &v.r_item)
    }
    // Running effects-specific
    pub(in crate::ud::item) fn get_reffs(&self) -> Option<&RSet<REffectKey>> {
        self.cache.as_ref().map(|v| &v.reffs)
    }
    pub(in crate::ud::item) fn update_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        // Always clear, regardless of item being loaded or not
        reuse_eupdates.clear();
        if let Some(cache) = &mut self.cache {
            process_effects(
                reuse_eupdates,
                &mut cache.reffs,
                src,
                &cache.r_item,
                self.state,
                &self.effect_modes,
            )
        }
    }
    pub(in crate::ud::item) fn stop_all_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        reuse_eupdates.clear();
        if let Some(cache) = &mut self.cache {
            process_effects(
                reuse_eupdates,
                &mut cache.reffs,
                src,
                &cache.r_item,
                AState::Ghost,
                &self.effect_modes,
            )
        }
    }
}

#[derive(Clone)]
struct ItemBaseCache {
    r_item: RcItem,
    // Running effects, are available only when adapted item is set
    reffs: RSet<REffectKey>,
}
