use crate::{
    ac,
    ad::{AAttrId, AAttrVal, AEffectId, AItemCatId, AItemEffectData, AItemGrpId, AItemId, ASkillLevel, AState},
    def::{AttrVal, ItemId, OF},
    misc::EffectMode,
    rd::{REffectKey, RItemAXt, RShipKind},
    src::Src,
    ud::{
        UFitKey, UPosition,
        item::{UEffectUpdates, UItemBase, UShipKind, bool_to_state_offline, state_to_bool},
    },
    util::{Named, RMap, RSet},
};

#[derive(Clone)]
pub(crate) struct UShip {
    pub(super) base: UItemBase,
    fit_key: UFitKey,
    kind: UShipKind,
    position: UPosition,
}
impl UShip {
    pub(crate) fn new(
        item_id: ItemId,
        type_id: AItemId,
        fit_key: UFitKey,
        ship_state: bool,
        position: UPosition,
        src: &Src,
    ) -> Self {
        let mut ship = Self {
            base: UItemBase::new(item_id, type_id, bool_to_state_offline(ship_state), src),
            fit_key,
            kind: UShipKind::Unknown,
            position,
        };
        ship.update_ship_kind();
        ship
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
        self.update_ship_kind();
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
    pub(in crate::ud::item) fn get_reffs(&self) -> Option<&RSet<REffectKey>> {
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
        self.update_ship_kind();
    }
    // Item-specific methods
    pub(crate) fn get_ship_state(&self) -> bool {
        state_to_bool(self.base.get_state())
    }
    pub(crate) fn set_ship_state(&mut self, state: bool) {
        self.base.set_state(bool_to_state_offline(state))
    }
    pub(crate) fn get_fit_key(&self) -> UFitKey {
        self.fit_key
    }
    pub(crate) fn get_kind(&self) -> UShipKind {
        self.kind
    }
    pub(crate) fn get_position(&self) -> &UPosition {
        &self.position
    }
    pub(in crate::ud::item) fn get_radius(&self) -> AttrVal {
        match self.get_axt() {
            Some(axt) => axt.radius,
            None => OF(0.0),
        }
    }
    pub(crate) fn get_pos_mut(&mut self) -> &mut UPosition {
        &mut self.position
    }
    pub(crate) fn get_disallowed_in_wspace(&self) -> Option<bool> {
        self.base.get_disallowed_in_wspace()
    }
    pub(crate) fn get_r_kind(&self) -> Option<RShipKind> {
        self.base.get_r_ship_kind()
    }
    fn update_ship_kind(&mut self) {
        self.kind = match self.get_category_id() {
            Some(ac::itemcats::SHIP) => UShipKind::Ship,
            Some(ac::itemcats::STRUCTURE) => UShipKind::Structure,
            _ => UShipKind::Unknown,
        };
    }
}
impl Named for UShip {
    fn get_name() -> &'static str {
        "Ship"
    }
}
impl std::fmt::Display for UShip {
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
