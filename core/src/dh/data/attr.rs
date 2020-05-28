use crate::{
    defines::{ReeFloat, ReeInt},
    util::Named,
};

/// Dogma attribute data.
#[derive(Debug)]
pub struct Attr {
    /// Dogma attribute ID.
    pub id: ReeInt,
    /// Defines if modifications applied to the attribute's values stack with penalty (false) or not
    /// (true).
    pub stackable: bool,
    /// Defines if higher value of the attribute is considered good or not.
    pub high_is_good: bool,
    /// Default value of the attribute, used if not provided by an item type.
    pub default_value: Option<ReeFloat>,
    /// Refers another attribute, whose value limits value of this attribute.
    pub max_attr_id: Option<ReeInt>,
    /// Defines what kind of unit is used for the attribute's value. Used during cache generation
    /// process during cleanup, since this field defines if value of the attribute refers another
    /// attribute, group or something else.
    pub unit_id: Option<ReeInt>,
}
impl Attr {
    /// Make a new dogma attribute out of passed data.
    pub fn new(
        id: ReeInt,
        stackable: bool,
        high_is_good: bool,
        default_value: Option<ReeFloat>,
        max_attr_id: Option<ReeInt>,
        unit_id: Option<ReeInt>,
    ) -> Attr {
        Attr {
            id,
            stackable,
            high_is_good,
            default_value,
            max_attr_id,
            unit_id,
        }
    }
    /// Default value seems to be initialized to 0.0, which stands for "no default value" in some
    /// contexts. Here we just return None instead of it, should the context need that.
    pub(crate) fn get_nonzero_defval(&self) -> Option<ReeFloat> {
        self.default_value.and_then(|v| if v == 0.0 { None } else { Some(v) })
    }
}
impl Named for Attr {
    fn get_name() -> &'static str {
        "dh::Attr"
    }
}
