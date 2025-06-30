use crate::{
    ad,
    def::{FitKey, ItemId},
    misc::{AdjustableCount, FighterCountOverride, MinionState},
    src::Src,
    uad::item::{Autocharges, EffectModes, Projs, UadItemBase},
    util::{Named, RMap},
};

#[derive(Clone)]
pub(crate) struct UadFighter {
    base: UadItemBase,
    fit_key: FitKey,
    count_override: Option<FighterCountOverride>,
    autocharges: Autocharges,
    projs: Projs,
}
impl UadFighter {
    pub(crate) fn new(src: &Src, item_id: ItemId, a_item_id: ad::AItemId, fit_key: FitKey, state: MinionState) -> Self {
        Self {
            base: UadItemBase::new(src, item_id, a_item_id, state.into()),
            fit_key,
            count_override: None,
            autocharges: Autocharges::new(),
            projs: Projs::new(),
        }
    }
    // Item base methods
    pub(crate) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(crate) fn get_a_item_id(&self) -> ad::AItemId {
        self.base.get_a_item_id()
    }
    pub(crate) fn set_a_item_id(&mut self, src: &Src, a_item_id: ad::AItemId) {
        self.base.set_a_item_id_and_reload(src, a_item_id);
    }
    pub(crate) fn get_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base.get_a_group_id()
    }
    pub(crate) fn get_a_category_id(&self) -> Option<ad::AItemCatId> {
        self.base.get_a_category_id()
    }
    pub(crate) fn get_a_attrs(&self) -> Option<&RMap<ad::AAttrId, ad::AAttrVal>> {
        self.base.get_a_attrs()
    }
    pub(crate) fn get_a_effect_datas(&self) -> Option<&RMap<ad::AEffectId, ad::AItemEffectData>> {
        self.base.get_a_effect_datas()
    }
    pub(crate) fn get_a_defeff_id(&self) -> Option<Option<ad::AEffectId>> {
        self.base.get_a_defeff_id()
    }
    pub(crate) fn get_a_skill_reqs(&self) -> Option<&RMap<ad::AItemId, ad::ASkillLevel>> {
        self.base.get_a_skill_reqs()
    }
    pub(crate) fn get_a_extras(&self) -> Option<&ad::AItemExtras> {
        self.base.get_a_extras()
    }
    pub(crate) fn get_a_state(&self) -> ad::AState {
        self.base.get_a_state()
    }
    pub(crate) fn get_effect_modes(&self) -> &EffectModes {
        self.base.get_effect_modes()
    }
    pub(crate) fn get_effect_modes_mut(&mut self) -> &mut EffectModes {
        self.base.get_effect_modes_mut()
    }
    pub(crate) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::uad::item) fn update_a_data(&mut self, src: &Src) {
        self.base.update_a_data(src);
        self.autocharges.clear()
    }
    // Item-specific methods
    pub(crate) fn get_fighter_state(&self) -> MinionState {
        self.base.get_a_state().into()
    }
    pub(crate) fn set_fighter_state(&mut self, state: MinionState) {
        self.base.set_a_state(state.into())
    }
    pub(crate) fn get_fit_key(&self) -> FitKey {
        self.fit_key
    }
    pub(crate) fn get_count(&self) -> Option<AdjustableCount> {
        match self.get_a_extras() {
            Some(extras) => match self.count_override {
                Some(count_override) => Some(AdjustableCount {
                    current: count_override.get_inner(),
                    max: extras.max_fighter_count,
                }),
                None => Some(AdjustableCount {
                    current: extras.max_fighter_count,
                    max: extras.max_fighter_count,
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
    pub(crate) fn get_projs(&self) -> &Projs {
        &self.projs
    }
    pub(crate) fn get_projs_mut(&mut self) -> &mut Projs {
        &mut self.projs
    }
}
impl Named for UadFighter {
    fn get_name() -> &'static str {
        "UadFighter"
    }
}
impl std::fmt::Display for UadFighter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}(item_id={}, a_item_id={})",
            Self::get_name(),
            self.get_item_id(),
            self.get_a_item_id(),
        )
    }
}
