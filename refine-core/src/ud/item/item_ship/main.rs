use crate::{
    EffectMode, ItemId, ItemKind, PValue,
    ad::{AEffectId, AItemCatId, AItemId},
    rd::{RData, REffectId, RItemAttrData, RItemBase, RState},
    ud::{
        UFitId,
        item::{UEffectUpdates, UItemBase, UPhysics, UShipKind, bool_to_state_offline, state_to_bool},
    },
    util::RSet,
};

#[derive(Clone)]
pub(crate) struct UShip {
    pub(super) base: UItemBase,
    fit_uid: UFitId,
    kind: UShipKind,
    physics: UPhysics,
}
impl UShip {
    pub(crate) fn new(
        item_id: ItemId,
        type_aid: AItemId,
        fit_uid: UFitId,
        ship_state: bool,
        physics: UPhysics,
        r_data: &RData,
    ) -> Self {
        let mut ship = Self {
            base: UItemBase::new(item_id, type_aid, bool_to_state_offline(ship_state), r_data),
            fit_uid,
            kind: UShipKind::Unknown,
            physics,
        };
        ship.update_ship_kind();
        ship
    }
    pub(in crate::ud::item) fn get_item_kind() -> ItemKind {
        ItemKind::Ship
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Item base methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UShip {
    // User data
    pub(crate) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(crate) fn get_type_aid(&self) -> AItemId {
        self.base.get_type_aid()
    }
    pub(crate) fn set_type_aid(&mut self, type_aid: AItemId, r_data: &RData) {
        self.base.set_type_aid(type_aid, r_data);
        self.update_ship_kind();
    }
    pub(crate) fn get_state(&self) -> RState {
        self.base.get_state()
    }
    pub(in crate::ud::item) fn get_reffs(&self) -> Option<&RSet<REffectId>> {
        self.base.get_reffs()
    }
    pub(crate) fn update_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, r_data: &RData) {
        self.base.update_reffs(reuse_eupdates, r_data, false, false);
    }
    pub(in crate::ud::item) fn stop_all_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, r_data: &RData) {
        self.base.stop_all_reffs(reuse_eupdates, r_data, false, false)
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
        self.update_ship_kind();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Item-specific methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UShip {
    pub(crate) fn get_ship_state(&self) -> bool {
        state_to_bool(self.base.get_state())
    }
    pub(crate) fn set_ship_state(&mut self, state: bool) {
        self.base.set_state(bool_to_state_offline(state))
    }
    pub(crate) fn get_fit_uid(&self) -> UFitId {
        self.fit_uid
    }
    pub(crate) fn get_ship_kind(&self) -> UShipKind {
        self.kind
    }
    pub(crate) fn get_physics(&self) -> &UPhysics {
        &self.physics
    }
    pub(crate) fn get_radius(&self) -> PValue {
        match self.get_r_item_attr_data() {
            Some(riad) => riad.radius,
            None => PValue::ZERO,
        }
    }
    pub(crate) fn get_physics_mut(&mut self) -> &mut UPhysics {
        &mut self.physics
    }
    fn update_ship_kind(&mut self) {
        self.kind = match self.get_r_item_base().map(|v| v.cat_id) {
            Some(AItemCatId::SHIP) => UShipKind::Ship,
            Some(AItemCatId::STRUCTURE) => UShipKind::Structure,
            _ => UShipKind::Unknown,
        };
    }
}
