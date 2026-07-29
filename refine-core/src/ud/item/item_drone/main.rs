use crate::{
    EffectMode, ItemId, ItemKind, NpcProp, PValue,
    ad::{AEffectId, AItemId},
    api::MinionState,
    err::basic::ItemNotMutatedError,
    rd::{RAttrId, RData, REffectId, RItemAttrData, RItemBase, RState},
    ud::{
        UAttrMutationRequest, UFitId, UItemMutationRequest,
        err::ItemMutatedError,
        item::{ItemMutationData, UEffectUpdates, UItemBaseMutable, UPhysics, UProjs},
    },
    util::RSet,
};

#[derive(Clone)]
pub(crate) struct UDrone {
    pub(super) base: UItemBaseMutable,
    fit_uid: UFitId,
    physics: UPhysics,
    npc_prop: Option<NpcProp>,
    projs: UProjs,
}
impl UDrone {
    pub(crate) fn new(
        item_id: ItemId,
        type_aid: AItemId,
        fit_uid: UFitId,
        drone_state: MinionState,
        mutation: Option<UItemMutationRequest>,
        physics: UPhysics,
        r_data: &RData,
    ) -> Self {
        Self {
            base: UItemBaseMutable::new(item_id, type_aid, drone_state.into_r_state(), mutation, r_data),
            fit_uid,
            physics,
            npc_prop: None,
            projs: UProjs::new(),
        }
    }
    pub(in crate::ud::item) fn get_item_kind() -> ItemKind {
        ItemKind::Drone
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Item base methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UDrone {
    // User data
    pub(crate) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(crate) fn get_type_aid(&self) -> AItemId {
        self.base.get_type_aid()
    }
    pub(crate) fn set_type_aid(&mut self, type_aid: AItemId, r_data: &RData) {
        self.base.set_type_aid(type_aid, r_data);
    }
    pub(crate) fn get_state(&self) -> RState {
        self.base.get_state()
    }
    pub(crate) fn get_reffs(&self) -> Option<&RSet<REffectId>> {
        self.base.get_reffs()
    }
    pub(crate) fn update_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, r_data: &RData) {
        self.base.update_reffs(reuse_eupdates, r_data, false, true);
    }
    pub(in crate::ud::item) fn stop_all_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, r_data: &RData) {
        self.base.stop_all_reffs(reuse_eupdates, r_data, false, true)
    }
    pub(in crate::ud::item) fn get_effect_mode(&self, effect_rid: &REffectId) -> EffectMode {
        self.base.get_effect_mode(effect_rid)
    }
    pub(in crate::ud::item) fn set_effect_mode(
        &mut self,
        effect_aid: AEffectId,
        effect_mode: EffectMode,
        r_data: &RData,
    ) {
        self.base.set_effect_mode(effect_aid, effect_mode, r_data)
    }
    pub(in crate::ud::item) fn set_effect_modes(
        &mut self,
        effect_modes: impl Iterator<Item = (AEffectId, EffectMode)>,
        r_data: &RData,
    ) {
        self.base.set_effect_modes(effect_modes, r_data)
    }
    // Runtime data
    pub(crate) fn get_r_item_base(&self) -> Option<&RItemBase> {
        self.base.get_r_item_base()
    }
    pub(crate) fn get_r_item_attr_data(&self) -> Option<&RItemAttrData> {
        self.base.get_r_item_attr_data()
    }
    pub(crate) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::ud::item) fn r_data_changed(&mut self, r_data: &RData) {
        self.base.r_data_changed(r_data);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Mutation-specific methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UDrone {
    pub(crate) fn get_mutation_data(&self) -> Option<&ItemMutationData> {
        self.base.get_mutation_data()
    }
    pub(crate) fn mutate(&mut self, mutation: UItemMutationRequest, r_data: &RData) -> Result<(), ItemNotMutatedError> {
        self.base.mutate(mutation, r_data)
    }
    pub(crate) fn change_mutation_attrs(
        &mut self,
        r_data: &RData,
        attr_mutations: Vec<UAttrMutationRequest>,
    ) -> Result<Vec<RAttrId>, ItemMutatedError> {
        self.base.change_mutation_attrs(r_data, attr_mutations)
    }
    pub(crate) fn set_mutator_type_aid(
        &mut self,
        mutator_type_aid: AItemId,
        r_data: &RData,
    ) -> Result<(), ItemMutatedError> {
        self.base.set_mutator_type_aid(mutator_type_aid, r_data)
    }
    pub(crate) fn unmutate(&mut self, r_data: &RData) -> Result<(), ItemMutatedError> {
        self.base.unmutate(r_data)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Item-specific methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UDrone {
    pub(crate) fn get_drone_state(&self) -> MinionState {
        MinionState::from_r_state(self.base.get_state())
    }
    pub(crate) fn set_drone_state(&mut self, state: MinionState) {
        self.base.set_state(state.into_r_state())
    }
    pub(crate) fn get_fit_uid(&self) -> UFitId {
        self.fit_uid
    }
    pub(in crate::ud::item) fn get_radius(&self) -> PValue {
        match self.get_r_item_attr_data() {
            Some(axt) => axt.radius,
            None => PValue::ZERO,
        }
    }
    pub(crate) fn get_physics(&self) -> &UPhysics {
        &self.physics
    }
    pub(crate) fn get_physics_mut(&mut self) -> &mut UPhysics {
        &mut self.physics
    }
    pub(crate) fn get_npc_prop(&self) -> Option<NpcProp> {
        self.npc_prop
    }
    pub(crate) fn set_npc_prop(&mut self, npc_prop: Option<NpcProp>) {
        self.npc_prop = npc_prop
    }
    pub(crate) fn get_projs(&self) -> &UProjs {
        &self.projs
    }
    pub(crate) fn get_projs_mut(&mut self) -> &mut UProjs {
        &mut self.projs
    }
}
