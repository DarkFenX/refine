use crate::{
    ad,
    def::ItemId,
    misc::EffectMode,
    src::Src,
    uad::item::misc::EffectModes,
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
    pub(in crate::uad::item) fn new(src: &Src, item_id: ItemId, a_item_id: ad::AItemId, state: ad::AState) -> Self {
        Self {
            item_id,
            a_item_id,
            a_state: state,
            effect_modes: EffectModes::new(),
            cache: src.get_a_item(&a_item_id).map(|v| ItemBaseCache {
                a_item: v.clone(),
                reffs: RSet::new(),
            }),
        }
    }
    // Basic data access methods
    pub(in crate::uad::item) fn get_item_id(&self) -> ItemId {
        self.item_id
    }
    pub(in crate::uad::item) fn get_a_item_id(&self) -> ad::AItemId {
        self.a_item_id
    }
    pub(in crate::uad::item) fn set_a_item_id(&mut self, a_item_id: ad::AItemId) {
        self.base_set_a_item_id(a_item_id);
    }
    pub(in crate::uad::item) fn set_a_item_id_and_reload(&mut self, src: &Src, a_item_id: ad::AItemId) {
        self.base_set_a_item_id(a_item_id);
        self.update_a_data(src);
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
    pub(in crate::uad::item) fn set_a_state(&mut self, state: ad::AState) {
        self.a_state = state
    }
    pub(in crate::uad::item) fn get_effect_mode(&self, a_effect_id: &ad::AEffectId) -> EffectMode {
        self.effect_modes.get(a_effect_id)
    }
    pub(in crate::uad::item) fn set_effect_mode(&mut self, a_effect_id: ad::AEffectId, effect_mode: EffectMode) {
        self.effect_modes.set(a_effect_id, effect_mode);
    }
    pub(in crate::uad::item) fn set_effect_modes(&mut self, modes: impl Iterator<Item = (ad::AEffectId, EffectMode)>) {
        for (a_effect_id, effect_mode) in modes {
            self.effect_modes.set(a_effect_id, effect_mode);
        }
    }
    pub(in crate::uad::item) fn is_loaded(&self) -> bool {
        self.cache.is_some()
    }
    pub(in crate::uad::item) fn update_a_data(&mut self, src: &Src) {
        match (&mut self.cache, src.get_a_item(&self.a_item_id)) {
            (Some(cache), Some(a_item)) => {}
            (Some(cache), None) => {}
            (None, Some(a_item)) => {}
            (None, None) => {}
        }
    }
    // Non-public methods
    pub(in crate::uad::item::base) fn new_with_a_item_id_not_loaded(
        item_id: ItemId,
        a_item_id: ad::AItemId,
        a_state: ad::AState,
    ) -> Self {
        Self {
            item_id,
            a_item_id,
            a_state,
            effect_modes: EffectModes::new(),
            cache: None,
        }
    }
    pub(in crate::uad::item::base) fn new_with_a_item(
        item_id: ItemId,
        a_item: ad::ArcItemRt,
        a_state: ad::AState,
    ) -> Self {
        Self {
            item_id,
            a_item_id: a_item.ai.id,
            a_state,
            effect_modes: EffectModes::new(),
            cache: Some(ItemBaseCache {
                a_item,
                reffs: RSet::new(),
            }),
        }
    }
    pub(in crate::uad::item::base) fn base_set_a_item_id(&mut self, a_item_id: ad::AItemId) {
        self.a_item_id = a_item_id;
    }
    pub(in crate::uad::item::base) fn base_set_a_item(&mut self, a_item: ad::ArcItemRt) {
        match &mut self.cache {
            Some(cache) => {
                cache.a_item = a_item;
                cache.reffs.clear();
            }
            None => {
                self.cache = Some(ItemBaseCache {
                    a_item,
                    reffs: RSet::new(),
                })
            }
        }
    }
    pub(in crate::uad::item::base) fn base_remove_a_item(&mut self) {
        self.cache = None;
    }
    pub(in crate::uad::item::base) fn base_get_a_item(&self) -> Option<&ad::ArcItemRt> {
        self.cache.as_ref().map(|v| &v.a_item)
    }
}

#[derive(Clone)]
struct ItemBaseCache {
    a_item: ad::ArcItemRt,
    // Running effects, are available only when adapted item is set
    reffs: RSet<ad::AEffectId>,
}
