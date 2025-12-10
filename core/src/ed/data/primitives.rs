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
pub type ESkillLevel = i8;
pub type ESlot = u32;

#[derive(Clone)]
pub enum EPrimitive {
    Null,
    Bool(bool),
    Int(i32),
    Float(f64),
    String(String),
}
