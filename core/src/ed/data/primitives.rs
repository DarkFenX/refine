// Entity IDs
pub type EAbilId = i32;
pub type EAttrId = i32;
pub type EAttrUnitId = i32;
pub type EBuffId = i32;
pub type EEffectId = i32;
pub type EEffectCatId = i32;
pub type EItemId = i32;
pub type EItemGrpId = i32;
pub type EItemCatId = i32;
pub type EItemListId = i32;
// Misc
pub type EAttrVal = f64;
pub type ECount = u32;
pub type ESkillLevel = u8;

/// Auxiliary entity for "primitive" data.
#[derive(Clone)]
pub enum EPrimitive {
    /// Represents absence of a value.
    Null,
    /// Represents a boolean value.
    Bool(bool),
    /// Represents an integer number value.
    Int(i32),
    /// Represents a float number value.
    Float(f64),
    /// Represents a string value.
    String(String),
}
