use crate::{
    ad::{AAttrVal, AEffectId, AItemCatId, AItemGrpId, AItemId, ASkillLevel, AState},
    def::{Count, Idx, ItemId, OF},
    err::basic::ItemNotMutatedError,
    misc::{AttrMutationRequest, EffectMode, ItemMutationRequest, ModRack, ModuleState, Spool},
    rd::{RAttrKey, REffectKey, RItemAXt, RItemEffectData},
    src::Src,
    ud::{
        UData, UFitKey, UItemKey,
        err::ItemMutatedError,
        item::{ItemMutationData, Projs, UEffectUpdates, UItemBaseMutable},
    },
    util::{Named, RMap, RSet, trunc_unerr},
};

#[derive(Clone)]
pub(crate) struct UModule {
    pub(super) base: UItemBaseMutable,
    fit_key: UFitKey,
    rack: ModRack,
    pos: Idx,
    charge_key: Option<UItemKey>,
    spool: Option<Spool>,
    projs: Projs,
}
impl UModule {
    pub(crate) fn new(
        item_id: ItemId,
        type_id: AItemId,
        fit_key: UFitKey,
        module_state: ModuleState,
        rack: ModRack,
        pos: Idx,
        mutation: Option<ItemMutationRequest>,
        charge_key: Option<UItemKey>,
        src: &Src,
    ) -> Self {
        Self {
            base: UItemBaseMutable::new(item_id, type_id, module_state.into(), mutation, src),
            fit_key,
            rack,
            pos,
            charge_key,
            spool: None,
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
    pub(crate) fn get_skill_reqs(&self) -> Option<&RMap<AItemId, ASkillLevel>> {
        self.base.get_skill_reqs()
    }
    pub(crate) fn get_axt(&self) -> Option<&RItemAXt> {
        self.base.get_axt()
    }
    pub(crate) fn get_max_state(&self) -> Option<AState> {
        self.base.get_max_state()
    }
    pub(crate) fn get_val_fitted_group_id(&self) -> Option<AItemGrpId> {
        self.base.get_val_fitted_group_id()
    }
    pub(crate) fn get_val_online_group_id(&self) -> Option<AItemGrpId> {
        self.base.get_val_online_group_id()
    }
    pub(crate) fn get_val_active_group_id(&self) -> Option<AItemGrpId> {
        self.base.get_val_active_group_id()
    }
    pub(crate) fn takes_turret_hardpoint(&self) -> bool {
        self.base.takes_turret_hardpoint()
    }
    pub(crate) fn takes_launcher_hardpoint(&self) -> bool {
        self.base.takes_launcher_hardpoint()
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
    pub(crate) fn stop_all_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
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
    ) -> Result<Vec<RAttrKey>, ItemMutatedError> {
        self.base.change_mutation_attrs(src, attr_mutations)
    }
    pub(crate) fn set_mutator_id(&mut self, mutator_id: AItemId, src: &Src) -> Result<(), ItemMutatedError> {
        self.base.set_mutator_id(mutator_id, src)
    }
    pub(crate) fn unmutate(&mut self, src: &Src) -> Result<(), ItemMutatedError> {
        self.base.unmutate(src)
    }
    // Item-specific methods
    pub(crate) fn get_module_state(&self) -> ModuleState {
        self.base.get_state().into()
    }
    pub(crate) fn set_module_state(&mut self, state: ModuleState) {
        self.base.set_state(state.into())
    }
    pub(crate) fn get_fit_key(&self) -> UFitKey {
        self.fit_key
    }
    pub(crate) fn get_rack(&self) -> ModRack {
        self.rack
    }
    pub(crate) fn get_pos(&self) -> Idx {
        self.pos
    }
    pub(crate) fn set_pos(&mut self, pos: Idx) {
        self.pos = pos
    }
    pub(crate) fn get_charge_key(&self) -> Option<UItemKey> {
        self.charge_key
    }
    pub(crate) fn set_charge_key(&mut self, charge_key: Option<UItemKey>) {
        self.charge_key = charge_key
    }
    pub(crate) fn get_charge_count(&self, u_data: &UData) -> Option<Count> {
        // No charge - no info
        let charge_key = self.get_charge_key()?;
        let charge_item = u_data.items.get(charge_key);
        let module_capacity = match self.get_axt() {
            Some(axt) => axt.capacity,
            // Module not loaded - no info
            _ => {
                return None;
            }
        };
        let charge_volume = match charge_item.get_axt() {
            Some(axt) if axt.volume != OF(0.0) => axt.volume,
            // Charge not loaded or has 0 volume - no info
            _ => {
                return None;
            }
        };
        let charge_count = trunc_unerr(module_capacity / charge_volume).into_inner() as Count;
        Some(charge_count)
    }
    pub(crate) fn get_spool(&self) -> Option<Spool> {
        self.spool
    }
    pub(crate) fn set_spool(&mut self, spool: Option<Spool>) {
        self.spool = spool
    }
    pub(crate) fn get_projs(&self) -> &Projs {
        &self.projs
    }
    pub(crate) fn get_projs_mut(&mut self) -> &mut Projs {
        &mut self.projs
    }
}
impl Named for UModule {
    fn get_name() -> &'static str {
        "Module"
    }
}
impl std::fmt::Display for UModule {
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
