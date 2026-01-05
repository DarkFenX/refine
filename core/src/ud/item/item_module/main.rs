use crate::{
    ad::{AEffectId, AItemCatId, AItemGrpId, AItemId},
    api::{ItemId, ModuleState},
    err::basic::ItemNotMutatedError,
    misc::{Count, EffectMode, Index, ModRack, PValue, SkillLevel, Spool, Value},
    rd::{RAttrId, REffectId, RItemAXt, RItemEffectData, RState, Src},
    ud::{
        UAttrMutationRequest, UData, UFitId, UItemId, UItemMutationRequest,
        err::ItemMutatedError,
        item::{ItemMutationData, UEffectUpdates, UItemBaseMutable, UProjs},
    },
    util::{LibNamed, RMap, RSet},
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
    reload_optionals: Option<bool>,
}
impl UModule {
    pub(crate) fn new(
        item_id: ItemId,
        type_id: AItemId,
        fit_uid: UFitId,
        module_state: ModuleState,
        rack: ModRack,
        pos: Index,
        mutation: Option<UItemMutationRequest>,
        charge_uid: Option<UItemId>,
        src: &Src,
    ) -> Self {
        Self {
            base: UItemBaseMutable::new(item_id, type_id, module_state.into(), mutation, src),
            fit_uid,
            rack,
            pos,
            charge_uid,
            projs: UProjs::new(),
            spool: None,
            reload_optionals: None,
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
    pub(crate) fn get_axt(&self) -> Option<&RItemAXt> {
        self.base.get_axt()
    }
    pub(crate) fn get_max_state(&self) -> Option<RState> {
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
    pub(crate) fn get_cap_use_attr_rids(&self) -> Option<&Vec<RAttrId>> {
        self.base.get_cap_use_attr_rids()
    }
    pub(crate) fn takes_turret_hardpoint(&self) -> bool {
        self.base.takes_turret_hardpoint()
    }
    pub(crate) fn takes_launcher_hardpoint(&self) -> bool {
        self.base.takes_launcher_hardpoint()
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
    pub(crate) fn stop_all_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
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
    pub(crate) fn get_module_state(&self) -> ModuleState {
        self.base.get_state().into()
    }
    pub(crate) fn set_module_state(&mut self, state: ModuleState) {
        self.base.set_state(state.into())
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
        let module_capacity = match self.get_axt() {
            Some(axt) => axt.capacity,
            // Module not loaded - no info
            _ => {
                return None;
            }
        };
        let charge_volume = match charge_item.get_axt() {
            Some(axt) if axt.volume != PValue::new_f64_unchecked(0.0) => axt.volume,
            // Charge not loaded or has 0 volume - no info
            _ => {
                return None;
            }
        };
        let charge_count = Count::new_f64_trunced(module_capacity.into_inner() / charge_volume.into_inner());
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
    pub(crate) fn get_reload_optionals(&self) -> Option<bool> {
        self.reload_optionals
    }
    pub(crate) fn set_reload_optionals(&mut self, reload_optionals: Option<bool>) {
        self.reload_optionals = reload_optionals
    }
}
impl LibNamed for UModule {
    fn lib_get_name() -> &'static str {
        "UModule"
    }
}
impl std::fmt::Display for UModule {
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
