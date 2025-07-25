use crate::{
    ad,
    def::ItemId,
    misc::EffectMode,
    rd,
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
    a_item_id: ad::AItemId,
    a_state: ad::AState,
    effect_modes: EffectModes,
    // Source-dependent data
    cache: Option<ItemBaseCache>,
}
impl UItemBase {
    // Constructors
    pub(in crate::ud::item) fn new(
        item_id: ItemId,
        a_item_id: ad::AItemId,
        state: ad::AState,
        src: &Src,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Self {
        let mut item = Self {
            item_id,
            a_item_id,
            a_state: state,
            effect_modes: EffectModes::new(),
            cache: src.get_r_item(&a_item_id).map(|r_item| ItemBaseCache {
                r_item: r_item.clone(),
                reffs: RSet::new(),
            }),
        };
        item.update_reffs(reuse_eupdates, src);
        item
    }
    pub(in crate::ud::item::base) fn base_new_with_a_item_id_not_loaded(
        item_id: ItemId,
        a_item_id: ad::AItemId,
        a_state: ad::AState,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Self {
        // When item is not loaded, just clear effect updates, we are not going to resolve any
        // effects
        reuse_eupdates.clear();
        Self {
            item_id,
            a_item_id,
            a_state,
            effect_modes: EffectModes::new(),
            cache: None,
        }
    }
    pub(in crate::ud::item::base) fn base_new_with_r_item(
        item_id: ItemId,
        r_item: rd::RcItem,
        a_state: ad::AState,
        src: &Src,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Self {
        let mut item = Self {
            item_id,
            a_item_id: r_item.get_id(),
            a_state,
            effect_modes: EffectModes::new(),
            cache: Some(ItemBaseCache {
                r_item,
                reffs: RSet::new(),
            }),
        };
        item.update_reffs(reuse_eupdates, src);
        item
    }
    // Basic data access methods
    pub(in crate::ud::item) fn get_item_id(&self) -> ItemId {
        self.item_id
    }
    pub(in crate::ud::item) fn get_a_item_id(&self) -> ad::AItemId {
        self.a_item_id
    }
    pub(in crate::ud::item) fn set_a_item_id(
        &mut self,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UEffectUpdates,
        src: &Src,
    ) {
        self.a_item_id = a_item_id;
        self.update_r_data(reuse_eupdates, src);
    }
    pub(in crate::ud::item) fn get_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base_get_r_item().map(|v| v.get_group_id())
    }
    pub(in crate::ud::item) fn get_a_category_id(&self) -> Option<ad::AItemCatId> {
        self.base_get_r_item().map(|v| v.get_category_id())
    }
    pub(in crate::ud::item) fn get_a_attrs(&self) -> Option<&RMap<ad::AAttrId, ad::AAttrVal>> {
        self.base_get_r_item().map(|v| v.get_attrs())
    }
    pub(in crate::ud::item) fn get_a_effect_datas(&self) -> Option<&RMap<ad::AEffectId, ad::AItemEffectData>> {
        self.base_get_r_item().map(|v| v.get_effect_datas_ids())
    }
    pub(in crate::ud::item) fn get_a_defeff_id(&self) -> Option<Option<ad::AEffectId>> {
        self.base_get_r_item().map(|v| v.get_defeff_id())
    }
    pub(in crate::ud::item) fn get_a_skill_reqs(&self) -> Option<&RMap<ad::AItemId, ad::ASkillLevel>> {
        self.base_get_r_item().map(|v| v.get_srqs())
    }
    // Extra data access methods
    pub(in crate::ud::item) fn get_r_axt(&self) -> Option<&rd::RItemAXt> {
        self.base_get_r_item().map(|v| v.get_axt())
    }
    pub(in crate::ud::item) fn get_disallowed_in_wspace(&self) -> Option<bool> {
        self.base_get_r_item().map(|v| v.is_disallowed_in_wspace())
    }
    pub(in crate::ud::item) fn get_val_fitted_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base_get_r_item().and_then(|v| v.get_val_fitted_group_id())
    }
    pub(in crate::ud::item) fn get_val_online_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base_get_r_item().and_then(|v| v.get_val_online_group_id())
    }
    pub(in crate::ud::item) fn get_r_ship_kind(&self) -> Option<rd::RShipKind> {
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
    pub(in crate::ud::item) fn get_a_state(&self) -> ad::AState {
        self.a_state
    }
    pub(in crate::ud::item) fn set_a_state(
        &mut self,
        state: ad::AState,
        reuse_eupdates: &mut UEffectUpdates,
        src: &Src,
    ) {
        self.a_state = state;
        self.update_reffs(reuse_eupdates, src);
    }
    pub(in crate::ud::item) fn get_effect_mode(&self, a_effect_id: &ad::AEffectId) -> EffectMode {
        self.effect_modes.get(a_effect_id)
    }
    pub(in crate::ud::item) fn set_effect_mode(
        &mut self,
        a_effect_id: ad::AEffectId,
        effect_mode: EffectMode,
        reuse_eupdates: &mut UEffectUpdates,
        src: &Src,
    ) {
        self.effect_modes.set(a_effect_id, effect_mode);
        self.update_reffs(reuse_eupdates, src);
    }
    pub(in crate::ud::item) fn set_effect_modes(
        &mut self,
        modes: impl Iterator<Item = (ad::AEffectId, EffectMode)>,
        reuse_eupdates: &mut UEffectUpdates,
        src: &Src,
    ) {
        for (a_effect_id, effect_mode) in modes {
            self.effect_modes.set(a_effect_id, effect_mode);
        }
        self.update_reffs(reuse_eupdates, src);
    }
    pub(in crate::ud::item) fn is_loaded(&self) -> bool {
        self.cache.is_some()
    }
    pub(in crate::ud::item) fn update_r_data(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        // Operations which replace r_item are assumed to handle effect stopping before the call. In
        // this method, only starting effects need to be returned, so we clear running effects
        // container prior to filling effect status updates
        match (&mut self.cache, src.get_r_item(&self.a_item_id)) {
            (Some(cache), Some(r_item)) => {
                cache.r_item = r_item.clone();
                // Clear running effects as well, to receive full list of effects on new item
                cache.reffs.clear();
                self.update_reffs(reuse_eupdates, src);
            }
            // No new item - no need to resolve effects, just clear results container
            (Some(_), None) => {
                self.cache = None;
                reuse_eupdates.clear();
            }
            (None, Some(r_item)) => {
                self.cache = Some(ItemBaseCache {
                    r_item: r_item.clone(),
                    reffs: RSet::new(),
                });
                self.update_reffs(reuse_eupdates, src);
            }
            // Just clear so that requesting code doesn't get outdated info
            (None, None) => reuse_eupdates.clear(),
        }
    }
    // Non-public methods
    pub(in crate::ud::item::base) fn base_set_a_item_id_primitive(&mut self, a_item_id: ad::AItemId) {
        self.a_item_id = a_item_id;
    }
    pub(in crate::ud::item::base) fn base_set_a_item_id_not_loaded(
        &mut self,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        self.a_item_id = a_item_id;
        self.cache = None;
        reuse_eupdates.clear();
    }
    pub(in crate::ud::item::base) fn base_set_r_item(
        &mut self,
        r_item: rd::RcItem,
        reuse_eupdates: &mut UEffectUpdates,
        src: &Src,
    ) {
        self.a_item_id = r_item.get_id();
        match &mut self.cache {
            Some(cache) => {
                cache.r_item = r_item;
                // Operations which change a_item are assumed to handle effect stopping before the
                // call. In this method, only starting effects need to be returned, so we clear
                // running effects container prior to filling effect status updates
                cache.reffs.clear();
            }
            None => {
                self.cache = Some(ItemBaseCache {
                    r_item,
                    reffs: RSet::new(),
                })
            }
        }
        self.update_reffs(reuse_eupdates, src);
    }
    pub(in crate::ud::item::base) fn base_get_r_item(&self) -> Option<&rd::RcItem> {
        self.cache.as_ref().map(|v| &v.r_item)
    }
    // Running effects-specific
    pub(in crate::ud::item) fn get_reffs(&self) -> Option<&RSet<ad::AEffectId>> {
        self.cache.as_ref().map(|v| &v.reffs)
    }
    pub(in crate::ud::item) fn start_all_reffs(&self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        // Fill "to start" with all running effects; does not change running effect container
        reuse_eupdates.clear();
        if let Some(cache) = &self.cache {
            reuse_eupdates
                .to_start
                .extend(cache.reffs.iter().map(|v| src.get_r_effect(v).unwrap().clone()));
        }
    }
    pub(in crate::ud::item) fn stop_all_reffs(&self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        // Fill "to stop" with all running effects; does not change running effect container
        reuse_eupdates.clear();
        if let Some(cache) = &self.cache {
            reuse_eupdates
                .to_stop
                .extend(cache.reffs.iter().map(|v| src.get_r_effect(v).unwrap().clone()));
        }
    }
    fn update_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        // Always clear, regardless of item being loaded or not
        reuse_eupdates.clear();
        if let Some(cache) = &mut self.cache {
            process_effects(
                reuse_eupdates,
                &mut cache.reffs,
                src,
                &cache.r_item,
                self.a_state,
                &self.effect_modes,
            )
        }
    }
}

#[derive(Clone)]
struct ItemBaseCache {
    r_item: rd::RcItem,
    // Running effects, are available only when adapted item is set
    reffs: RSet<ad::AEffectId>,
}
