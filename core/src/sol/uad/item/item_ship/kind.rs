#[derive(Copy, Clone, Eq, PartialEq)]
pub(in crate::sol) enum ShipKind {
    Ship,
    Structure,
    Unknown,
}
