use crate::{
    ad::{AAttrId, AAttrVal, AEffectId, AItemCatId, AItemEffectData, AItemGrpId, AItemId, ASkillLevel, AState},
    def::ItemId,
    err::basic::ItemNotMutatedError,
    misc::{AttrMutationRequest, EffectMode, ItemMutationRequest, MinionState},
    rd::{REffectKey, RItemAXt},
    src::Src,
    ud::{
        UFitKey,
        err::ItemMutatedError,
        item::{ItemMutationData, Projs, UEffectUpdates, UItemBaseMutable},
    },
    util::{Named, RMap, RSet},
};
#[derive(Clone)]
pub(crate) struct UDrone {
    pub(super) base: UItemBaseMutable,
    fit_key: UFitKey,
    projs: Projs,
}
impl UDrone {
    pub(crate) fn new(
        item_id: ItemId,
        type_id: AItemId,
        fit_key: UFitKey,
        drone_state: MinionState,
        mutation: Option<ItemMutationRequest>,
        src: &Src,
    ) -> Self {
        Self {
            base: UItemBaseMutable::new(item_id, type_id, drone_state.into(), mutation, src),
            fit_key,
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
    pub(crate) fn get_attrs(&self) -> Option<&RMap<AAttrId, AAttrVal>> {
        self.base.get_attrs()
    }
    pub(crate) fn get_effect_datas(&self) -> Option<&RMap<REffectKey, AItemEffectData>> {
        self.base.get_effect_datas()
    }
    pub(crate) fn get_defeff_key(&self) -> Option<Option<REffectKey>> {
        self.base.get_defeff_key()
    }
    pub(crate) fn get_skill_reqs(&self) -> Option<&RMap<AItemId, ASkillLevel>> {
        self.base.get_skill_reqs()
    }
    pub(crate) fn get_axt(&self) -> Option<&RItemAXt> {
        self.base.get_axt()
    }
    pub(crate) fn get_state(&self) -> AState {
        self.base.get_state()
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
    pub(in crate::ud::item) fn get_effect_key_mode(&self, effect_key: &REffectKey) -> EffectMode {
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
    }
    // Mutation-specific methods
    pub(crate) fn get_mutation_data(&self) -> Option<&ItemMutationData> {
        self.base.get_mutation_data()
    }
    pub(crate) fn mutate(&mut self, mutation: ItemMutationRequest, src: &Src) -> Result<(), ItemNotMutatedError> {
        self.base.mutate(mutation, src)
    }
    pub(crate) fn change_mutation_attrs(
        &mut self,
        src: &Src,
        attr_mutations: Vec<AttrMutationRequest>,
    ) -> Result<Vec<AAttrId>, ItemMutatedError> {
        self.base.change_mutation_attrs(src, attr_mutations)
    }
    pub(crate) fn set_mutator_id(&mut self, mutator_id: AItemId, src: &Src) -> Result<(), ItemMutatedError> {
        self.base.set_mutator_id(mutator_id, src)
    }
    pub(crate) fn unmutate(&mut self, src: &Src) -> Result<(), ItemMutatedError> {
        self.base.unmutate(src)
    }
    // Item-specific methods
    pub(crate) fn get_drone_state(&self) -> MinionState {
        self.base.get_state().into()
    }
    pub(crate) fn set_drone_state(&mut self, state: MinionState) {
        self.base.set_state(state.into())
    }
    pub(crate) fn get_fit_key(&self) -> UFitKey {
        self.fit_key
    }
    pub(crate) fn get_projs(&self) -> &Projs {
        &self.projs
    }
    pub(crate) fn get_projs_mut(&mut self) -> &mut Projs {
        &mut self.projs
    }
}
impl Named for UDrone {
    fn get_name() -> &'static str {
        "Drone"
    }
}
impl std::fmt::Display for UDrone {
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
