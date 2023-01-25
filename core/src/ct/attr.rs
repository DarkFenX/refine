use crate::{
    defines::{ReeFloat, ReeInt},
    util::Named,
};

/// Represents a dogma attribute.
///
/// An attribute carries just attribute properties which govern how modified attribute values are
/// calculated. Values themselves are stored on various items as plain numbers.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Attr {
    /// Attribute ID.
    pub id: ReeInt,
    /// Defines if modifications applied to the attribute's values are immune to stacking penalties
    /// or not.
    pub penalizable: bool,
    /// "High is good" defines if higher value of the attribute is considered good or not.
    pub hig: bool,
    /// Default value of the attribute, used if not provided by an item type.
    pub def_val: Option<ReeFloat>,
    /// Refers another attribute, whose value limits value of this attribute.
    pub max_attr_id: Option<ReeInt>,
}
impl Attr {
    /// Make a new dogma attribute out of passed data.
    pub fn new(
        id: ReeInt,
        penalizable: bool,
        hig: bool,
        def_val: Option<ReeFloat>,
        max_attr_id: Option<ReeInt>,
    ) -> Attr {
        Attr {
            id,
            penalizable,
            hig,
            def_val,
            max_attr_id,
        }
    }
}
impl Named for Attr {
    fn get_name() -> &'static str {
        "ct::Attr"
    }
}
