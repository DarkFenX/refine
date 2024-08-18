use crate::{
    ad,
    defs::{EItemId, SolFitId, SolItemId},
    ec,
    err::basic::ItemLoadedError,
    sol::item::{bool_to_state, state_to_bool, SolEffectModes, SolItemBase, SolItemState, SolShipKind},
    src::Src,
    util::Named,
};

#[derive(Clone)]
pub(in crate::sol) struct SolShip {
    base: SolItemBase,
    fit_id: SolFitId,
    kind: SolShipKind,
}
impl SolShip {
    pub(in crate::sol) fn new(src: &Src, id: SolItemId, type_id: EItemId, fit_id: SolFitId, state: bool) -> Self {
        let mut ship = Self {
            base: SolItemBase::new(src, id, type_id, bool_to_state(state)),
            fit_id,
            kind: SolShipKind::Unknown,
        };
        ship.update_ship_kind();
        ship
    }
    // Item base methods
    pub(in crate::sol) fn get_id(&self) -> SolItemId {
        self.base.get_id()
    }
    pub(in crate::sol) fn get_type_id(&self) -> EItemId {
        self.base.get_type_id()
    }
    pub(in crate::sol) fn get_a_item(&self) -> Result<&ad::ArcItem, ItemLoadedError> {
        self.base.get_a_item()
    }
    pub(in crate::sol) fn get_state(&self) -> SolItemState {
        self.base.get_state()
    }
    pub(in crate::sol) fn get_effect_modes(&self) -> &SolEffectModes {
        self.base.get_effect_modes()
    }
    pub(in crate::sol) fn get_effect_modes_mut(&mut self) -> &mut SolEffectModes {
        self.base.get_effect_modes_mut()
    }
    pub(in crate::sol) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::sol::item) fn reload_a_item(&mut self, src: &Src) {
        self.base.reload_a_item(src);
        self.update_ship_kind();
    }
    // Item-specific methods
    pub(in crate::sol) fn get_fit_id(&self) -> SolFitId {
        self.fit_id
    }
    pub(in crate::sol) fn get_bool_state(&self) -> bool {
        state_to_bool(self.base.get_state())
    }
    pub(in crate::sol) fn set_bool_state(&mut self, state: bool) {
        self.base.set_state(bool_to_state(state))
    }
    pub(in crate::sol) fn get_kind(&self) -> SolShipKind {
        self.kind
    }
    fn update_ship_kind(&mut self) {
        self.kind = match self.get_a_item() {
            Ok(a_item) => match a_item.cat_id {
                ec::itemcats::SHIP => SolShipKind::Ship,
                ec::itemcats::STRUCTURE => SolShipKind::Structure,
                _ => SolShipKind::Unknown,
            },
            _ => SolShipKind::Unknown,
        };
    }
}
impl Named for SolShip {
    fn get_name() -> &'static str {
        "SolShip"
    }
}
impl std::fmt::Display for SolShip {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}(id={}, type_id={})",
            Self::get_name(),
            self.get_id(),
            self.get_type_id(),
        )
    }
}
