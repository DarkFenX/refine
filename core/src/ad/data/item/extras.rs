use crate::ad::AItemKind;

/// Holds extra item-specific data.
///
/// It is derived from data normally available on item and other entities, but is calculated on
/// cache generation time for optimization purposes.
pub struct AItemExtras {
    /// Item type.
    pub kind: Option<AItemKind>,
}
impl AItemExtras {
    pub fn new(kind: Option<AItemKind>) -> Self {
        Self { kind }
    }
}
