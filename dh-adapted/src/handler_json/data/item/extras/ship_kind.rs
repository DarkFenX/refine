#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CShipKind {
    Ship,
    CapitalShip,
    Structure,
}
impl From<&rc::ad::AShipKind> for CShipKind {
    fn from(ship_kind: &rc::ad::AShipKind) -> Self {
        match ship_kind {
            rc::ad::AShipKind::Ship => Self::Ship,
            rc::ad::AShipKind::CapitalShip => Self::CapitalShip,
            rc::ad::AShipKind::Structure => Self::Structure,
        }
    }
}
impl Into<rc::ad::AShipKind> for &CShipKind {
    fn into(self) -> rc::ad::AShipKind {
        match self {
            CShipKind::Ship => rc::ad::AShipKind::Ship,
            CShipKind::CapitalShip => rc::ad::AShipKind::CapitalShip,
            CShipKind::Structure => rc::ad::AShipKind::Structure,
        }
    }
}
