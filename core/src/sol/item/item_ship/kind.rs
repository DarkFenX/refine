#[derive(Copy, Clone)]
pub(in crate::sol) enum SolShipKind {
    Ship,
    Structure,
    Unknown,
}
impl Default for SolShipKind {
    fn default() -> Self {
        Self::Unknown
    }
}
