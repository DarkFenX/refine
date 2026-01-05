use crate::{
    ad::{AEffectId, AItemCatId, AItemGrpId, AItemId},
    api::MinionState,
    err::basic::ItemNotMutatedError,
    misc::{EffectMode, PValue, SkillLevel, Value},
    rd::{RAttrId, REffectId, RItemAXt, RItemEffectData, RItemListId, RState, Src},
    ud::{
        ItemId, UAttrMutationRequest, UFitId, UItemMutationRequest,
        err::ItemMutatedError,
        item::{ItemMutationData, UEffectUpdates, UItemBaseMutable, UNpcProp, UPhysics, UProjs},
    },
    util::{LibNamed, RMap, RSet},
};

#[derive(Clone)]
pub(crate) struct UDrone {
    pub(super) base: UItemBaseMutable,
    fit_uid: UFitId,
    physics: UPhysics,
    prop: UNpcProp,
    projs: UProjs,
}
impl UDrone {
    pub(crate) fn new(
        item_id: ItemId,
        type_id: AItemId,
        fit_uid: UFitId,
        drone_state: MinionState,
        mutation: Option<UItemMutationRequest>,
        physics: UPhysics,
        prop: UNpcProp,
        src: &Src,
    ) -> Self {
        Self {
            base: UItemBaseMutable::new(item_id, type_id, drone_state.into(), mutation, src),
            fit_uid,
            physics,
            prop,
            projs: UProjs::new(),
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
    pub(in crate::ud::item) fn get_effect_mode(&self, effect_rid: &REffectId) -> EffectMode {
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
    }
    // Mutation-specific methods
    pub(crate) fn get_mutation_data(&self) -> Option<&ItemMutationData> {
        self.base.get_mutation_data()
    }
    pub(crate) fn mutate(&mut self, mutation: UItemMutationRequest, src: &Src) -> Result<(), ItemNotMutatedError> {
        self.base.mutate(mutation, src)
    }
    pub(crate) fn change_mutation_attrs(
        &mut self,
        src: &Src,
        attr_mutations: Vec<UAttrMutationRequest>,
    ) -> Result<Vec<RAttrId>, ItemMutatedError> {
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
    pub(crate) fn get_fit_uid(&self) -> UFitId {
        self.fit_uid
    }
    pub(in crate::ud::item) fn get_radius(&self) -> PValue {
        match self.get_axt() {
            Some(axt) => axt.radius,
            None => PValue::new_f64_unchecked(0.0),
        }
    }
    pub(crate) fn get_physics(&self) -> &UPhysics {
        &self.physics
    }
    pub(crate) fn get_physics_mut(&mut self) -> &mut UPhysics {
        &mut self.physics
    }
    pub(crate) fn get_prop_mode(&self) -> UNpcProp {
        self.prop
    }
    pub(crate) fn set_prop_mode(&mut self, prop: UNpcProp) {
        self.prop = prop
    }
    pub(crate) fn get_projs(&self) -> &UProjs {
        &self.projs
    }
    pub(crate) fn get_projs_mut(&mut self) -> &mut UProjs {
        &mut self.projs
    }
}
impl LibNamed for UDrone {
    fn lib_get_name() -> &'static str {
        "UDrone"
    }
}
impl std::fmt::Display for UDrone {
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
