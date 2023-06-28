use crate::{
    defs::{AttrVal, EAttrId, EAttrUnitId},
    util::Named,
};

/// EVE dogma attribute data.
#[derive(Debug)]
pub struct EAttr {
    /// Attribute ID.
    pub id: EAttrId,
    /// Defines if modifications applied to the attribute's values stack with penalty (false) or not
    /// (true).
    pub stackable: bool,
    /// Defines if higher value of the attribute is considered good or not.
    pub high_is_good: bool,
    /// Default value of the attribute, used if not provided by an item type.
    pub default_value: Option<AttrVal>,
    /// Refers another attribute, whose value limits value of this attribute.
    pub max_attr_id: Option<EAttrId>,
    /// Defines what kind of unit is used for the attribute's value. Used during cache generation
    /// process during cleanup, since this field defines if value of the attribute refers another
    /// attribute, group or something else.
    pub unit_id: Option<EAttrUnitId>,
}
impl EAttr {
    /// Make a new EVE dogma attribute out of passed data.
    pub fn new(
        id: EAttrId,
        stackable: bool,
        high_is_good: bool,
        default_value: Option<AttrVal>,
        max_attr_id: Option<EAttrId>,
        unit_id: Option<EAttrUnitId>,
    ) -> Self {
        Self {
            id,
            stackable,
            high_is_good,
            default_value,
            max_attr_id,
            unit_id,
        }
    }
}
impl Named for EAttr {
    fn get_name() -> &'static str {
        "EAttr"
    }
}
