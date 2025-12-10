use crate::{
    ad::{AAbilId, AAttrVal, AEffectId, AItemCatId, AItemGrpId, AItemId, ASkillLevel, AState},
    def::{AttrVal, ItemId, OF},
    misc::{AdjustableCount, EffectMode, FighterCountOverride, MinionState},
    rd::{RAttrKey, REffectKey, RItemAXt, RItemEffectData, RItemListKey},
    src::Src,
    ud::{
        UFitKey,
        item::{Autocharges, Projs, UEffectUpdates, UItemBase, UPhysics},
    },
    util::{Named, RMap, RSet},
};

#[derive(Clone)]
pub(crate) struct UFighter {
    pub(super) base: UItemBase,
    fit_key: UFitKey,
    count_override: Option<FighterCountOverride>,
    autocharges: Autocharges,
    physics: UPhysics,
    projs: Projs,
}
impl UFighter {
    pub(crate) fn new(
        item_id: ItemId,
        type_id: AItemId,
        fit_key: UFitKey,
        fighter_state: MinionState,
        physics: UPhysics,
        src: &Src,
    ) -> Self {
        Self {
            base: UItemBase::new(item_id, type_id, fighter_state.into(), src),
            fit_key,
            count_override: None,
            autocharges: Autocharges::new(),
            physics,
            projs: Projs::new(),
        }
    }
    // Item base methods
    pub(crate) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(crate) fn get_type_id(&self) -> AItemId {
        self.base.get_type_id()
    }
    pub(crate) fn set_type_id(&mut self, type_id: AItemId, src: &Src) {
        self.base.set_type_id(type_id, src);
    }
    pub(crate) fn get_group_id(&self) -> Option<AItemGrpId> {
        self.base.get_group_id()
    }
    pub(crate) fn get_category_id(&self) -> Option<AItemCatId> {
        self.base.get_category_id()
    }
    pub(crate) fn get_attrs(&self) -> Option<&RMap<RAttrKey, AAttrVal>> {
        self.base.get_attrs()
    }
    pub(crate) fn get_effect_datas(&self) -> Option<&RMap<REffectKey, RItemEffectData>> {
        self.base.get_effect_datas()
    }
    pub(crate) fn get_defeff_key(&self) -> Option<Option<REffectKey>> {
        self.base.get_defeff_key()
    }
    pub(crate) fn get_abils(&self) -> Option<&Vec<AAbilId>> {
        self.base.get_abils()
    }
    pub(crate) fn get_skill_reqs(&self) -> Option<&RMap<AItemId, ASkillLevel>> {
        self.base.get_skill_reqs()
    }
    pub(crate) fn get_proj_buff_item_lists(&self) -> Option<&Vec<RItemListKey>> {
        self.base.get_proj_buff_item_lists()
    }
    pub(crate) fn get_axt(&self) -> Option<&RItemAXt> {
        self.base.get_axt()
    }
    pub(crate) fn get_state(&self) -> AState {
        self.base.get_state()
    }
    pub(in crate::ud::item) fn is_ice_harvester(&self) -> bool {
        self.base.is_ice_harvester()
    }
    pub(crate) fn get_reffs(&self) -> Option<&RSet<REffectKey>> {
        self.base.get_reffs()
    }
    pub(crate) fn update_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.update_reffs(reuse_eupdates, src);
    }
    pub(in crate::ud::item) fn stop_all_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.stop_all_reffs(reuse_eupdates, src)
    }
    pub(crate) fn get_effect_key_mode(&self, effect_key: &REffectKey) -> EffectMode {
        self.base.get_effect_key_mode(effect_key)
    }
    pub(in crate::ud::item) fn set_effect_mode(&mut self, effect_id: AEffectId, effect_mode: EffectMode, src: &Src) {
        self.base.set_effect_mode(effect_id, effect_mode, src)
    }
    pub(in crate::ud::item) fn set_effect_modes(
        &mut self,
        effect_modes: impl Iterator<Item = (AEffectId, EffectMode)>,
        src: &Src,
    ) {
        self.base.set_effect_modes(effect_modes, src)
    }
    pub(crate) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::ud::item) fn src_changed(&mut self, src: &Src) {
        self.base.src_changed(src);
        self.autocharges.clear();
    }
    // Item-specific methods
    pub(crate) fn get_fighter_state(&self) -> MinionState {
        self.base.get_state().into()
    }
    pub(crate) fn set_fighter_state(&mut self, state: MinionState) {
        self.base.set_state(state.into())
    }
    pub(crate) fn get_fit_key(&self) -> UFitKey {
        self.fit_key
    }
    pub(crate) fn get_count(&self) -> Option<AdjustableCount> {
        match self.get_axt() {
            Some(axt) => match self.count_override {
                Some(count_override) => Some(AdjustableCount {
                    current: count_override.get_inner(),
                    max: axt.max_fighter_count,
                    overridden: true,
                }),
                None => Some(AdjustableCount {
                    current: axt.max_fighter_count,
                    max: axt.max_fighter_count,
                    overridden: false,
                }),
            },
            None => None,
        }
    }
    pub(crate) fn set_count_override(&mut self, count_override: Option<FighterCountOverride>) {
        self.count_override = count_override
    }
    pub(crate) fn get_autocharges(&self) -> &Autocharges {
        &self.autocharges
    }
    pub(crate) fn get_autocharges_mut(&mut self) -> &mut Autocharges {
        &mut self.autocharges
    }
    pub(crate) fn get_physics(&self) -> &UPhysics {
        &self.physics
    }
    pub(in crate::ud::item) fn get_radius(&self) -> AttrVal {
        match self.get_axt() {
            Some(axt) => axt.radius,
            None => OF(0.0),
        }
    }
    pub(crate) fn get_physics_mut(&mut self) -> &mut UPhysics {
        &mut self.physics
    }
    pub(crate) fn get_projs(&self) -> &Projs {
        &self.projs
    }
    pub(crate) fn get_projs_mut(&mut self) -> &mut Projs {
        &mut self.projs
    }
}
impl Named for UFighter {
    fn get_name() -> &'static str {
        "UFighter"
    }
}
impl std::fmt::Display for UFighter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}(item_id={}, type_id={})",
            Self::get_name(),
            self.get_item_id(),
            self.get_type_id(),
        )
    }
}
