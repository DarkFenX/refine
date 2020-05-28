use crate::defines::{ReeFloat, ReeInt};

/// Represents a dogma attribute.
///
/// An attribute carries just attribute properties which govern how modified attribute values are
/// calculated. Values themselves are stored on various items as plain numbers.
#[derive(Debug)]
pub struct Attr {
    // Attribute ID.
    pub id: ReeInt,
    /// Defines if modifications applied to the attribute's values are immune to stacking penalties
    /// or not.
    pub penalizable: bool,
    /// Defines if higher value of the attribute is considered good or not.
    pub high_is_good: bool,
    /// Default value of the attribute, used if not provided by an item type.
    pub default_value: Option<ReeFloat>,
    /// Refers another attribute, whose value limits value of this attribute.
    pub max_attr_id: Option<ReeInt>,
}
impl Attr {
    /// Make a new dogma attribute out of passed data.
    pub fn new(
        id: ReeInt,
        penalizable: bool,
        high_is_good: bool,
        default_value: Option<ReeFloat>,
        max_attr_id: Option<ReeInt>,
    ) -> Attr {
        Attr {
            id,
            penalizable,
            high_is_good,
            default_value,
            max_attr_id,
        }
    }
}
