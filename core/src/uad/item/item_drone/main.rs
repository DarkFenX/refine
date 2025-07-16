use crate::{
    ad,
    def::{FitKey, ItemId},
    err::basic::ItemNotMutatedError,
    misc::{AttrMutationRequest, EffectMode, ItemMutationRequest, MinionState},
    src::Src,
    uad::{
        err::ItemMutatedError,
        item::{ItemMutationData, Projs, UadItemBaseMutable},
    },
    util::{Named, RMap},
};
#[derive(Clone)]
pub(crate) struct UadDrone {
    base: UadItemBaseMutable,
    fit_key: FitKey,
    projs: Projs,
}
impl UadDrone {
    pub(crate) fn new(
        src: &Src,
        item_id: ItemId,
        a_item_id: ad::AItemId,
        fit_key: FitKey,
        state: MinionState,
        mutation: Option<ItemMutationRequest>,
    ) -> Self {
        Self {
            base: UadItemBaseMutable::new(src, item_id, a_item_id, state.into(), mutation),
            fit_key,
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
        self.base.set_a_item_id(src, a_item_id);
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
    pub(crate) fn get_a_xt(&self) -> Option<&ad::AItemXt> {
        self.base.get_a_xt()
    }
    pub(crate) fn get_a_state(&self) -> ad::AState {
        self.base.get_a_state()
    }
    pub(in crate::uad::item) fn get_effect_mode(&self, effect_id: &ad::AEffectId) -> EffectMode {
        self.base.get_effect_mode(effect_id)
    }
    pub(in crate::uad::item) fn set_effect_mode(&mut self, a_effect_id: ad::AEffectId, effect_mode: EffectMode) {
        self.base.set_effect_mode(a_effect_id, effect_mode)
    }
    pub(in crate::uad::item) fn set_effect_modes(&mut self, modes: impl Iterator<Item = (ad::AEffectId, EffectMode)>) {
        self.base.set_effect_modes(modes)
    }
    pub(crate) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::uad::item) fn update_a_data(&mut self, src: &Src) {
        self.base.update_a_data(src);
    }
    // Mutation-specific methods
    pub(crate) fn get_mutation_data(&self) -> Option<&ItemMutationData> {
        self.base.get_mutation_data()
    }
    pub(crate) fn mutate(&mut self, src: &Src, mutation: ItemMutationRequest) -> Result<(), ItemNotMutatedError> {
        self.base.mutate(src, mutation)
    }
    pub(crate) fn change_mutation_attrs(
        &mut self,
        src: &Src,
        attr_mutations: Vec<AttrMutationRequest>,
    ) -> Result<Vec<ad::AAttrId>, ItemMutatedError> {
        self.base.change_mutation_attrs(src, attr_mutations)
    }
    pub(crate) fn set_a_mutator_id(&mut self, src: &Src, a_mutator_id: ad::AItemId) -> Result<(), ItemMutatedError> {
        self.base.set_a_mutator_id(src, a_mutator_id)
    }
    pub(crate) fn unmutate(&mut self, src: &Src) -> Result<(), ItemMutatedError> {
        self.base.unmutate(src)
    }
    // Item-specific methods
    pub(crate) fn get_drone_state(&self) -> MinionState {
        self.base.get_a_state().into()
    }
    pub(crate) fn set_drone_state(&mut self, state: MinionState) {
        self.base.set_a_state(state.into())
    }
    pub(crate) fn get_fit_key(&self) -> FitKey {
        self.fit_key
    }
    pub(crate) fn get_projs(&self) -> &Projs {
        &self.projs
    }
    pub(crate) fn get_projs_mut(&mut self) -> &mut Projs {
        &mut self.projs
    }
}
impl Named for UadDrone {
    fn get_name() -> &'static str {
        "Drone"
    }
}
impl std::fmt::Display for UadDrone {
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
