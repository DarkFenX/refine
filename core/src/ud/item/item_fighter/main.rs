use crate::{
    ad::{AAbilId, AEffectId, AItemCatId, AItemGrpId, AItemId},
    api::{AdjustableCount, MinionState},
    misc::{EffectMode, FighterCount, PValue, RearmMinions, SkillLevel, StOption, Value},
    rd::{RAttrId, REffectId, RItemAXt, RItemEffectData, RItemListId, RState, Src},
    ud::{
        ItemId, UEffectUpdates, UFitId, UPhysics, UProjs,
        item::{UAutocharges, UItemBase},
    },
    util::{LibNamed, RMap, RSet},
};

#[derive(Clone)]
pub(crate) struct UFighter {
    pub(super) base: UItemBase,
    fit_uid: UFitId,
    count_override: Option<FighterCount>,
    autocharges: UAutocharges,
    physics: UPhysics,
    projs: UProjs,
    // Optional settings related to cycling
    rearm_minions: StOption<RearmMinions>,
}
impl UFighter {
    pub(crate) fn new(
        item_id: ItemId,
        type_id: AItemId,
        fit_uid: UFitId,
        fighter_state: MinionState,
        physics: UPhysics,
        src: &Src,
    ) -> Self {
        Self {
            base: UItemBase::new(item_id, type_id, fighter_state.into(), src),
            fit_uid,
            count_override: None,
            autocharges: UAutocharges::new(),
            physics,
            projs: UProjs::new(),
            rearm_minions: StOption::Inherit,
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
    pub(crate) fn get_attrs(&self) -> Option<&RMap<RAttrId, Value>> {
        self.base.get_attrs()
    }
    pub(crate) fn get_effect_datas(&self) -> Option<&RMap<REffectId, RItemEffectData>> {
        self.base.get_effect_datas()
    }
    pub(crate) fn get_defeff_rid(&self) -> Option<Option<REffectId>> {
        self.base.get_defeff_rid()
    }
    pub(crate) fn get_abils(&self) -> Option<&Vec<AAbilId>> {
        self.base.get_abils()
    }
    pub(crate) fn get_skill_reqs(&self) -> Option<&RMap<AItemId, SkillLevel>> {
        self.base.get_skill_reqs()
    }
    pub(crate) fn get_proj_buff_item_lists(&self) -> Option<&Vec<RItemListId>> {
        self.base.get_proj_buff_item_lists()
    }
    pub(crate) fn get_axt(&self) -> Option<&RItemAXt> {
        self.base.get_axt()
    }
    pub(crate) fn get_state(&self) -> RState {
        self.base.get_state()
    }
    pub(in crate::ud::item) fn is_ice_harvester(&self) -> bool {
        self.base.is_ice_harvester()
    }
    pub(crate) fn get_reffs(&self) -> Option<&RSet<REffectId>> {
        self.base.get_reffs()
    }
    pub(crate) fn update_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.update_reffs(reuse_eupdates, src);
    }
    pub(in crate::ud::item) fn stop_all_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.stop_all_reffs(reuse_eupdates, src)
    }
    pub(crate) fn get_effect_mode(&self, effect_rid: &REffectId) -> EffectMode {
        self.base.get_effect_mode(effect_rid)
    }
    pub(in crate::ud::item) fn set_effect_mode(&mut self, effect_aid: AEffectId, effect_mode: EffectMode, src: &Src) {
        self.base.set_effect_mode(effect_aid, effect_mode, src)
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
    pub(crate) fn get_fit_uid(&self) -> UFitId {
        self.fit_uid
    }
    pub(crate) fn get_count(&self) -> Option<AdjustableCount> {
        match self.get_axt() {
            Some(axt) => match self.count_override {
                Some(count_override) => Some(AdjustableCount {
                    current: count_override.into_inner(),
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
    pub(crate) fn set_count_override(&mut self, count_override: Option<FighterCount>) {
        self.count_override = count_override
    }
    pub(crate) fn get_autocharges(&self) -> &UAutocharges {
        &self.autocharges
    }
    pub(crate) fn get_autocharges_mut(&mut self) -> &mut UAutocharges {
        &mut self.autocharges
    }
    pub(crate) fn get_physics(&self) -> &UPhysics {
        &self.physics
    }
    pub(in crate::ud::item) fn get_radius(&self) -> PValue {
        match self.get_axt() {
            Some(axt) => axt.radius,
            None => PValue::new_f64_unchecked(0.0),
        }
    }
    pub(crate) fn get_physics_mut(&mut self) -> &mut UPhysics {
        &mut self.physics
    }
    pub(crate) fn get_projs(&self) -> &UProjs {
        &self.projs
    }
    pub(crate) fn get_projs_mut(&mut self) -> &mut UProjs {
        &mut self.projs
    }
    pub(crate) fn get_rearm_minions(&self) -> StOption<RearmMinions> {
        self.rearm_minions
    }
    pub(crate) fn set_rearm_minions(&mut self, rearm_minions: StOption<RearmMinions>) {
        self.rearm_minions = rearm_minions
    }
}
impl LibNamed for UFighter {
    fn lib_get_name() -> &'static str {
        "UFighter"
    }
}
impl std::fmt::Display for UFighter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}(item_id={}, type_id={})",
            Self::lib_get_name(),
            self.get_item_id(),
            self.get_type_id(),
        )
    }
}
