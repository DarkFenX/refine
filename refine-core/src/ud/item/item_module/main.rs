use crate::{
    Count, EffectMode, Index, ItemId, ItemKind, ModRack, ModuleState, OptionalReload, PValue, Spool,
    ad::{AEffectId, AItemId},
    err::basic::ItemNotMutatedError,
    rd::{RAttrId, RData, REffectId, RItemAttrData, RItemBase, RState},
    ud::{
        UAttrMutationRequest, UData, UFitId, UItemId, UItemMutationRequest,
        err::ItemMutatedError,
        item::{ItemMutationData, UEffectModeOverrideIter, UEffectUpdates, UItemBaseMutable, UProjs},
    },
    util::RSet,
};

#[derive(Clone)]
pub(crate) struct UModule {
    pub(super) base: UItemBaseMutable,
    fit_uid: UFitId,
    rack: ModRack,
    pos: Index,
    charge_uid: Option<UItemId>,
    projs: UProjs,
    // Optional settings related to cycling
    spool: Option<Spool>,
    optional_reload_override: Option<OptionalReload>,
}
impl UModule {
    pub(crate) fn new(
        item_id: ItemId,
        type_aid: AItemId,
        fit_uid: UFitId,
        module_state: ModuleState,
        rack: ModRack,
        pos: Index,
        mutation: Option<UItemMutationRequest>,
        charge_uid: Option<UItemId>,
        r_data: &RData,
    ) -> Self {
        Self {
            base: UItemBaseMutable::new(item_id, type_aid, module_state.into_r_state(), mutation, r_data),
            fit_uid,
            rack,
            pos,
            charge_uid,
            projs: UProjs::new(),
            spool: None,
            optional_reload_override: None,
        }
    }
    pub(in crate::ud::item) fn get_item_kind() -> ItemKind {
        ItemKind::Module
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Item base methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UModule {
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
        self.base.update_reffs(reuse_eupdates, r_data, false, false);
    }
    pub(crate) fn stop_all_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, r_data: &RData) {
        self.base.stop_all_reffs(reuse_eupdates, r_data, false, false)
    }
    pub(in crate::ud::item) fn get_effect_mode_by_rid(&self, effect_rid: &REffectId) -> EffectMode {
        self.base.get_effect_mode_by_rid(effect_rid)
    }
    pub(in crate::ud::item) fn iter_effect_mode_overrides(&self) -> UEffectModeOverrideIter<'_> {
        self.base.iter_effect_mode_overrides()
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
impl UModule {
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
impl UModule {
    pub(crate) fn get_module_state(&self) -> ModuleState {
        ModuleState::from_r_state(self.base.get_state())
    }
    pub(crate) fn set_module_state(&mut self, state: ModuleState) {
        self.base.set_state(state.into_r_state())
    }
    pub(crate) fn get_fit_uid(&self) -> UFitId {
        self.fit_uid
    }
    pub(crate) fn get_rack(&self) -> ModRack {
        self.rack
    }
    pub(crate) fn get_pos(&self) -> Index {
        self.pos
    }
    pub(crate) fn set_pos(&mut self, pos: Index) {
        self.pos = pos
    }
    pub(crate) fn get_charge_uid(&self) -> Option<UItemId> {
        self.charge_uid
    }
    pub(crate) fn set_charge_uid(&mut self, charge_uid: Option<UItemId>) {
        self.charge_uid = charge_uid
    }
    pub(crate) fn get_charge_count(&self, u_data: &UData) -> Option<Count> {
        // No charge - no info
        let charge_uid = self.get_charge_uid()?;
        let charge_item = u_data.items.get(charge_uid);
        let module_capacity = match self.get_r_item_attr_data() {
            Some(riad) => riad.capacity,
            // Module not loaded - no info
            _ => {
                return None;
            }
        };
        let charge_volume = match charge_item.get_r_item_attr_data() {
            Some(riad) if riad.volume != PValue::ZERO => riad.volume,
            // Charge not loaded or has 0 volume - no info
            _ => {
                return None;
            }
        };
        let charge_count = Count::from_pvalue_trunced(module_capacity / charge_volume);
        Some(charge_count)
    }
    pub(crate) fn get_projs(&self) -> &UProjs {
        &self.projs
    }
    pub(crate) fn get_projs_mut(&mut self) -> &mut UProjs {
        &mut self.projs
    }
    pub(crate) fn get_spool(&self) -> Option<Spool> {
        self.spool
    }
    pub(crate) fn set_spool(&mut self, spool: Option<Spool>) {
        self.spool = spool
    }
    pub(crate) fn get_optional_reload_override(&self) -> Option<OptionalReload> {
        self.optional_reload_override
    }
    pub(crate) fn set_optional_reload_override(&mut self, optional_reload_override: Option<OptionalReload>) {
        self.optional_reload_override = optional_reload_override
    }
}
