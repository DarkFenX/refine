use crate::ed::EAttrUnitId;

impl EAttrUnitId {
    pub(crate) const GROUP_ID: Self = Self::from_i32(115);
    pub(crate) const ITEM_ID: Self = Self::from_i32(116);
    pub(crate) const ATTR_ID: Self = Self::from_i32(119);
}
