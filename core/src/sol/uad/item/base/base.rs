use crate::{
    ad,
    sol::{ItemId, uad::item::EffectModes},
    src::Src,
    util::HMap,
};

// Item base stores all the data every item should have
#[derive(Clone)]
pub(in crate::sol::uad::item) struct ItemBase {
    // User-defined data
    item_id: ItemId,
    a_item_id: ad::AItemId,
    a_state: ad::AState,
    effect_modes: EffectModes,
    // Source-dependent data
    cache: Option<ItemBaseCache>,
}
impl ItemBase {
    pub(in crate::sol::uad::item) fn new(
        src: &Src,
        item_id: ItemId,
        a_item_id: ad::AItemId,
        state: ad::AState,
    ) -> Self {
        Self {
            item_id,
            a_item_id,
            a_state: state,
            effect_modes: EffectModes::new(),
            cache: src.get_a_item(&a_item_id).map(|v| ItemBaseCache { a_item: v.clone() }),
        }
    }
    pub(in crate::sol::uad::item) fn get_item_id(&self) -> ItemId {
        self.item_id
    }
    pub(in crate::sol::uad::item) fn get_a_item_id(&self) -> ad::AItemId {
        self.a_item_id
    }
    pub(in crate::sol::uad::item) fn get_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.get_a_item().map(|v| v.grp_id)
    }
    pub(in crate::sol::uad::item) fn get_a_category_id(&self) -> Option<ad::AItemCatId> {
        self.get_a_item().map(|v| v.cat_id)
    }
    pub(in crate::sol::uad::item) fn get_a_attrs(&self) -> Option<&HMap<ad::AAttrId, ad::AAttrVal>> {
        self.get_a_item().map(|v| &v.attrs)
    }
    pub(in crate::sol::uad::item) fn get_a_effect_datas(&self) -> Option<&HMap<ad::AEffectId, ad::AItemEffectData>> {
        self.get_a_item().map(|v| &v.effect_datas)
    }
    pub(in crate::sol::uad::item) fn get_a_defeff_id(&self) -> Option<Option<ad::AEffectId>> {
        self.get_a_item().map(|v| v.defeff_id)
    }
    pub(in crate::sol::uad::item) fn get_a_skill_reqs(&self) -> Option<&HMap<ad::AItemId, ad::ASkillLevel>> {
        self.get_a_item().map(|v| &v.srqs)
    }
    pub(in crate::sol::uad::item) fn get_a_extras(&self) -> Option<&ad::AItemExtras> {
        self.get_a_item().map(|v| &v.extras)
    }
    pub(in crate::sol::uad::item) fn get_a_state(&self) -> ad::AState {
        self.a_state
    }
    pub(in crate::sol::uad::item) fn set_a_state(&mut self, state: ad::AState) {
        self.a_state = state
    }
    pub(in crate::sol::uad::item) fn get_effect_modes(&self) -> &EffectModes {
        &self.effect_modes
    }
    pub(in crate::sol::uad::item) fn get_effect_modes_mut(&mut self) -> &mut EffectModes {
        &mut self.effect_modes
    }
    pub(in crate::sol::uad::item) fn is_loaded(&self) -> bool {
        self.cache.is_some()
    }
    pub(in crate::sol::uad::item) fn update_a_data(&mut self, src: &Src) {
        self.cache = src
            .get_a_item(&self.a_item_id)
            .map(|v| ItemBaseCache { a_item: v.clone() });
    }
    // Non-public methods
    pub(in crate::sol::uad::item::base) fn new_with_id_not_loaded(
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
    pub(in crate::sol::uad::item::base) fn new_with_item(
        item_id: ItemId,
        a_item: ad::ArcItem,
        a_state: ad::AState,
    ) -> Self {
        Self {
            item_id,
            a_item_id: a_item.id,
            a_state,
            effect_modes: EffectModes::new(),
            cache: Some(ItemBaseCache { a_item }),
        }
    }
    pub(in crate::sol::uad::item::base) fn set_a_item_id(&mut self, a_item_id: ad::AItemId) {
        self.a_item_id = a_item_id;
    }
    pub(in crate::sol::uad::item::base) fn set_a_item_id_and_reload(&mut self, src: &Src, a_item_id: ad::AItemId) {
        self.set_a_item_id(a_item_id);
        self.update_a_data(src);
    }
    pub(in crate::sol::uad::item::base) fn set_a_item(&mut self, a_item: ad::ArcItem) {
        match &mut self.cache {
            Some(cache) => cache.a_item = a_item,
            None => self.cache = Some(ItemBaseCache { a_item }),
        }
    }
    pub(in crate::sol::uad::item::base) fn remove_a_item(&mut self) {
        self.cache = None;
    }
    pub(in crate::sol::uad::item::base) fn get_a_item(&self) -> Option<&ad::ArcItem> {
        self.cache.as_ref().map(|v| &v.a_item)
    }
}

#[derive(Clone)]
struct ItemBaseCache {
    a_item: ad::ArcItem,
}
