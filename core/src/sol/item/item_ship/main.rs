use crate::{
    ad,
    defs::{EItemId, SolFitId, SolItemId},
    ec,
    sol::item::{bool_to_state, state_to_bool, SolItemBase, SolItemState, SolShipKind},
    src::Src,
    util::Named,
};

pub(in crate::sol) struct SolShip {
    pub(in crate::sol) base: SolItemBase,
    pub(in crate::sol) fit_id: SolFitId,
    pub(in crate::sol) state: SolItemState,
    pub(in crate::sol) kind: SolShipKind,
}
impl SolShip {
    pub(in crate::sol) fn new(src: &Src, id: SolItemId, fit_id: SolFitId, a_item_id: EItemId, state: bool) -> Self {
        let a_item = src.get_a_item(&a_item_id).cloned();
        let kind = detect_fit_kind(&a_item);
        Self {
            base: SolItemBase::new(src, id, a_item_id),
            fit_id,
            state: bool_to_state(state),
            kind,
        }
    }
    pub(in crate::sol) fn get_bool_state(&self) -> bool {
        state_to_bool(self.state)
    }
    pub(in crate::sol) fn set_bool_state(&mut self, state: bool) {
        self.state = bool_to_state(state);
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
            "{}(id={}, a_item_id={})",
            Self::get_name(),
            self.base.id,
            self.base.a_item_id
        )
    }
}

fn detect_fit_kind(a_item: &Option<ad::ArcItem>) -> SolShipKind {
    match a_item {
        Some(a_item) => match a_item.cat_id {
            ec::itemcats::SHIP => SolShipKind::Ship,
            ec::itemcats::STRUCTURE => SolShipKind::Structure,
            _ => SolShipKind::Unknown,
        },
        None => SolShipKind::Unknown,
    }
}
