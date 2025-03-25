use crate::{
    ad,
    sol::{
        AdjustableCount, Count, FitId, ItemId,
        uad::item::{Autocharges, EffectModes, ItemBase, MinionState, Projs},
    },
    src::Src,
    util::{Named, StMap},
};

#[derive(Clone)]
pub(in crate::sol) struct Fighter {
    base: ItemBase,
    fit_id: FitId,
    count_override: Option<Count>,
    autocharges: Autocharges,
    projs: Projs,
}
impl Fighter {
    pub(in crate::sol) fn new(
        src: &Src,
        item_id: ItemId,
        a_item_id: ad::AItemId,
        fit_id: FitId,
        state: MinionState,
    ) -> Self {
        Self {
            base: ItemBase::new(src, item_id, a_item_id, state.into()),
            fit_id,
            count_override: None,
            autocharges: Autocharges::new(),
            projs: Projs::new(),
        }
    }
    // Item base methods
    pub(in crate::sol) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(in crate::sol) fn get_a_item_id(&self) -> ad::AItemId {
        self.base.get_a_item_id()
    }
    pub(in crate::sol) fn get_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base.get_a_group_id()
    }
    pub(in crate::sol) fn get_a_category_id(&self) -> Option<ad::AItemCatId> {
        self.base.get_a_category_id()
    }
    pub(in crate::sol) fn get_a_attrs(&self) -> Option<&StMap<ad::AAttrId, ad::AAttrVal>> {
        self.base.get_a_attrs()
    }
    pub(in crate::sol) fn get_a_effect_datas(&self) -> Option<&StMap<ad::AEffectId, ad::AItemEffectData>> {
        self.base.get_a_effect_datas()
    }
    pub(in crate::sol) fn get_a_defeff_id(&self) -> Option<Option<ad::AEffectId>> {
        self.base.get_a_defeff_id()
    }
    pub(in crate::sol) fn get_a_skill_reqs(&self) -> Option<&StMap<ad::AItemId, ad::ASkillLevel>> {
        self.base.get_a_skill_reqs()
    }
    pub(in crate::sol) fn get_a_extras(&self) -> Option<&ad::AItemExtras> {
        self.base.get_a_extras()
    }
    pub(in crate::sol) fn get_a_state(&self) -> ad::AState {
        self.base.get_a_state()
    }
    pub(in crate::sol) fn get_effect_modes(&self) -> &EffectModes {
        self.base.get_effect_modes()
    }
    pub(in crate::sol) fn get_effect_modes_mut(&mut self) -> &mut EffectModes {
        self.base.get_effect_modes_mut()
    }
    pub(in crate::sol) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::sol::uad::item) fn update_a_data(&mut self, src: &Src) {
        self.base.update_a_data(src);
        self.autocharges.clear()
    }
    // Item-specific methods
    pub(in crate::sol) fn get_fighter_state(&self) -> MinionState {
        self.base.get_a_state().into()
    }
    pub(in crate::sol) fn set_fighter_state(&mut self, state: MinionState) {
        self.base.set_a_state(state.into())
    }
    pub(in crate::sol) fn get_fit_id(&self) -> FitId {
        self.fit_id
    }
    pub(in crate::sol) fn get_count(&self) -> Option<AdjustableCount> {
        match self.get_a_extras() {
            Some(extras) => match self.count_override {
                Some(count_override) => Some(AdjustableCount {
                    current: count_override,
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
    pub(in crate::sol) fn set_count_override(&mut self, count_override: Option<Count>) {
        self.count_override = count_override
    }
    pub(in crate::sol) fn get_autocharges(&self) -> &Autocharges {
        &self.autocharges
    }
    pub(in crate::sol) fn get_autocharges_mut(&mut self) -> &mut Autocharges {
        &mut self.autocharges
    }
    pub(in crate::sol) fn get_projs(&self) -> &Projs {
        &self.projs
    }
    pub(in crate::sol) fn get_projs_mut(&mut self) -> &mut Projs {
        &mut self.projs
    }
}
impl Named for Fighter {
    fn get_name() -> &'static str {
        "Fighter"
    }
}
impl std::fmt::Display for Fighter {
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
