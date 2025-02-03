#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CShipKind {
    Ship,
    CapitalShip,
    Structure,
}
impl From<&rc::ad::AShipKind> for CShipKind {
    fn from(a_ship_kind: &rc::ad::AShipKind) -> Self {
        match a_ship_kind {
            rc::ad::AShipKind::Ship => Self::Ship,
            rc::ad::AShipKind::CapitalShip => Self::CapitalShip,
            rc::ad::AShipKind::Structure => Self::Structure,
        }
    }
}
impl From<&CShipKind> for rc::ad::AShipKind {
    fn from(c_ship_kind: &CShipKind) -> Self {
        match c_ship_kind {
            CShipKind::Ship => Self::Ship,
            CShipKind::CapitalShip => Self::CapitalShip,
            CShipKind::Structure => Self::Structure,
        }
    }
}
