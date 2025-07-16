use crate::{
    ad,
    def::ItemId,
    misc::EffectMode,
    src::Src,
    uad::item::{
        base::{UadEffectUpdates, process_effects},
        misc::EffectModes,
    },
    util::{RMap, RSet},
};

// Item base stores all the data every item should have
#[derive(Clone)]
pub(in crate::uad::item) struct UadItemBase {
    // User-defined data
    item_id: ItemId,
    a_item_id: ad::AItemId,
    a_state: ad::AState,
    effect_modes: EffectModes,
    // Source-dependent data
    cache: Option<ItemBaseCache>,
}
impl UadItemBase {
    // Constructors
    pub(in crate::uad::item) fn new(
        item_id: ItemId,
        a_item_id: ad::AItemId,
        state: ad::AState,
        src: &Src,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> Self {
        UadItemBase::base_new(item_id, a_item_id, state, src, reuse_eupdates, None)
    }
    pub(in crate::uad::item::base) fn base_new(
        item_id: ItemId,
        a_item_id: ad::AItemId,
        state: ad::AState,
        src: &Src,
        reuse_eupdates: &mut UadEffectUpdates,
        a_xt_override: Option<&ad::AItemXt>,
    ) -> Self {
        let mut item = Self {
            item_id,
            a_item_id,
            a_state: state,
            effect_modes: EffectModes::new(),
            cache: src.get_a_item(&a_item_id).map(|v| ItemBaseCache {
                a_item: v.clone(),
                reffs: RSet::new(),
            }),
        };
        item.update_reffs(reuse_eupdates, src, a_xt_override);
        item
    }
    pub(in crate::uad::item::base) fn base_new_with_a_item_id_not_loaded(
        item_id: ItemId,
        a_item_id: ad::AItemId,
        a_state: ad::AState,
        reuse_eupdates: &mut UadEffectUpdates,
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
    pub(in crate::uad::item::base) fn base_new_with_a_item(
        item_id: ItemId,
        a_item: ad::ArcItemRt,
        a_state: ad::AState,
        src: &Src,
        reuse_eupdates: &mut UadEffectUpdates,
        a_xt_override: Option<&ad::AItemXt>,
    ) -> Self {
        let mut item = Self {
            item_id,
            a_item_id: a_item.ai.id,
            a_state,
            effect_modes: EffectModes::new(),
            cache: Some(ItemBaseCache {
                a_item,
                reffs: RSet::new(),
            }),
        };
        item.update_reffs(reuse_eupdates, src, a_xt_override);
        item
    }
    // Basic data access methods
    pub(in crate::uad::item) fn get_item_id(&self) -> ItemId {
        self.item_id
    }
    pub(in crate::uad::item) fn get_a_item_id(&self) -> ad::AItemId {
        self.a_item_id
    }
    pub(in crate::uad::item) fn set_a_item_id(
        &mut self,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
    ) {
        self.base_set_a_item_id_and_reload(a_item_id, reuse_eupdates, src, None)
    }
    pub(in crate::uad::item) fn base_set_a_item_id_and_reload(
        &mut self,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
        a_xt_override: Option<&ad::AItemXt>,
    ) {
        self.a_item_id = a_item_id;
        self.base_update_a_data(reuse_eupdates, src, a_xt_override);
    }
    pub(in crate::uad::item) fn get_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base_get_a_item().map(|v| v.ai.grp_id)
    }
    pub(in crate::uad::item) fn get_a_category_id(&self) -> Option<ad::AItemCatId> {
        self.base_get_a_item().map(|v| v.ai.cat_id)
    }
    pub(in crate::uad::item) fn get_a_attrs(&self) -> Option<&RMap<ad::AAttrId, ad::AAttrVal>> {
        self.base_get_a_item().map(|v| &v.ai.attrs)
    }
    pub(in crate::uad::item) fn get_a_effect_datas(&self) -> Option<&RMap<ad::AEffectId, ad::AItemEffectData>> {
        self.base_get_a_item().map(|v| &v.ai.effect_datas)
    }
    pub(in crate::uad::item) fn get_a_defeff_id(&self) -> Option<Option<ad::AEffectId>> {
        self.base_get_a_item().map(|v| v.ai.defeff_id)
    }
    pub(in crate::uad::item) fn get_a_skill_reqs(&self) -> Option<&RMap<ad::AItemId, ad::ASkillLevel>> {
        self.base_get_a_item().map(|v| &v.ai.srqs)
    }
    // Extra data access methods
    pub(in crate::uad::item) fn get_a_xt(&self) -> Option<&ad::AItemXt> {
        self.base_get_a_item().map(|v| &v.xt)
    }
    pub(in crate::uad::item) fn get_disallowed_in_wspace(&self) -> Option<bool> {
        self.base_get_a_item().map(|v| v.ai.disallowed_in_wspace)
    }
    pub(in crate::uad::item) fn get_val_fitted_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base_get_a_item().and_then(|v| v.ai.val_fitted_group_id)
    }
    pub(in crate::uad::item) fn get_val_online_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base_get_a_item().and_then(|v| v.ai.val_online_group_id)
    }
    // Misc methods
    pub(in crate::uad::item) fn get_a_state(&self) -> ad::AState {
        self.a_state
    }
    pub(in crate::uad::item) fn set_a_state(
        &mut self,
        state: ad::AState,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
    ) {
        self.base_set_a_state(state, reuse_eupdates, src, None)
    }
    pub(in crate::uad::item::base) fn base_set_a_state(
        &mut self,
        state: ad::AState,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
        a_xt_override: Option<&ad::AItemXt>,
    ) {
        self.a_state = state;
        self.update_reffs(reuse_eupdates, src, a_xt_override);
    }
    pub(in crate::uad::item) fn get_effect_mode(&self, a_effect_id: &ad::AEffectId) -> EffectMode {
        self.effect_modes.get(a_effect_id)
    }
    pub(in crate::uad::item) fn set_effect_mode(
        &mut self,
        a_effect_id: ad::AEffectId,
        effect_mode: EffectMode,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
    ) {
        self.base_set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src, None)
    }
    pub(in crate::uad::item::base) fn base_set_effect_mode(
        &mut self,
        a_effect_id: ad::AEffectId,
        effect_mode: EffectMode,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
        a_xt_override: Option<&ad::AItemXt>,
    ) {
        self.effect_modes.set(a_effect_id, effect_mode);
        self.update_reffs(reuse_eupdates, src, a_xt_override);
    }
    pub(in crate::uad::item) fn set_effect_modes(
        &mut self,
        modes: impl Iterator<Item = (ad::AEffectId, EffectMode)>,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
    ) {
        self.base_set_effect_modes(modes, reuse_eupdates, src, None)
    }
    pub(in crate::uad::item::base) fn base_set_effect_modes(
        &mut self,
        modes: impl Iterator<Item = (ad::AEffectId, EffectMode)>,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
        a_xt_override: Option<&ad::AItemXt>,
    ) {
        for (a_effect_id, effect_mode) in modes {
            self.effect_modes.set(a_effect_id, effect_mode);
        }
        self.update_reffs(reuse_eupdates, src, a_xt_override);
    }
    pub(in crate::uad::item) fn is_loaded(&self) -> bool {
        self.cache.is_some()
    }
    pub(in crate::uad::item) fn update_a_data(&mut self, reuse_eupdates: &mut UadEffectUpdates, src: &Src) {
        self.base_update_a_data(reuse_eupdates, src, None)
    }
    pub(in crate::uad::item::base) fn base_update_a_data(
        &mut self,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
        a_xt_override: Option<&ad::AItemXt>,
    ) {
        // Operations which change a_item are assumed to handle effect stopping before the call. In
        // this method, only starting effects need to be returned, so we clear running effects
        // container prior to filling effect status updates
        match (&mut self.cache, src.get_a_item(&self.a_item_id)) {
            (Some(cache), Some(a_item)) => {
                cache.a_item = a_item.clone();
                // Clear running effects as well, to receive full list of effects on new item
                cache.reffs.clear();
                self.update_reffs(reuse_eupdates, src, a_xt_override);
            }
            // No new item - no need to resolve effects, just clear results container
            (Some(_), None) => {
                self.cache = None;
                reuse_eupdates.clear();
            }
            (None, Some(a_item)) => {
                self.cache = Some(ItemBaseCache {
                    a_item: a_item.clone(),
                    reffs: RSet::new(),
                });
                self.update_reffs(reuse_eupdates, src, a_xt_override);
            }
            // Just clear so that requesting code doesn't get outdated info
            (None, None) => reuse_eupdates.clear(),
        }
    }
    // Non-public methods
    pub(in crate::uad::item::base) fn base_set_a_item_id_primitive(&mut self, a_item_id: ad::AItemId) {
        self.a_item_id = a_item_id;
    }
    pub(in crate::uad::item::base) fn base_set_a_item_id_not_loaded(
        &mut self,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        self.a_item_id = a_item_id;
        self.cache = None;
        reuse_eupdates.clear();
    }
    pub(in crate::uad::item::base) fn base_set_a_item(
        &mut self,
        a_item: ad::ArcItemRt,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
        a_xt_override: Option<&ad::AItemXt>,
    ) {
        self.a_item_id = a_item.ai.id;
        match &mut self.cache {
            Some(cache) => {
                cache.a_item = a_item;
                // Operations which change a_item are assumed to handle effect stopping before the
                // call. In this method, only starting effects need to be returned, so we clear
                // running effects container prior to filling effect status updates
                cache.reffs.clear();
            }
            None => {
                self.cache = Some(ItemBaseCache {
                    a_item,
                    reffs: RSet::new(),
                })
            }
        }
        self.update_reffs(reuse_eupdates, src, a_xt_override);
    }
    pub(in crate::uad::item::base) fn base_get_a_item(&self) -> Option<&ad::ArcItemRt> {
        self.cache.as_ref().map(|v| &v.a_item)
    }
    // Running effects-specific
    pub(in crate::uad::item) fn get_reffs(&self) -> Option<&RSet<ad::AEffectId>> {
        self.cache.as_ref().map(|v| &v.reffs)
    }
    pub(in crate::uad::item) fn start_all_reffs(&self, reuse_eupdates: &mut UadEffectUpdates, src: &Src) {
        // Fill "to start" with all running effects; does not change running effect container
        reuse_eupdates.clear();
        if let Some(cache) = &self.cache {
            reuse_eupdates
                .to_start
                .extend(cache.reffs.iter().map(|v| src.get_a_effect(v).unwrap().clone()));
        }
    }
    pub(in crate::uad::item) fn stop_all_reffs(&self, reuse_eupdates: &mut UadEffectUpdates, src: &Src) {
        // Fill "to stop" with all running effects; does not change running effect container
        reuse_eupdates.clear();
        if let Some(cache) = &self.cache {
            reuse_eupdates
                .to_stop
                .extend(cache.reffs.iter().map(|v| src.get_a_effect(v).unwrap().clone()));
        }
    }
    fn update_reffs(&mut self, reuse_eupdates: &mut UadEffectUpdates, src: &Src, a_xt_override: Option<&ad::AItemXt>) {
        // Always clear, regardless of item being loaded or not
        reuse_eupdates.clear();
        if let Some(cache) = &mut self.cache {
            process_effects(
                reuse_eupdates,
                &mut cache.reffs,
                src,
                &cache.a_item.ai.effect_datas,
                cache.a_item.ai.defeff_id,
                self.a_state,
                &self.effect_modes,
                a_xt_override.unwrap_or(&cache.a_item.xt),
            )
        }
    }
}

#[derive(Clone)]
struct ItemBaseCache {
    a_item: ad::ArcItemRt,
    // Running effects, are available only when adapted item is set
    reffs: RSet<ad::AEffectId>,
}
