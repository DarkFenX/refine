#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CTgtMode {
    None,
    Item,
    Point,
}
impl From<&rc::ad::TgtMode> for CTgtMode {
    fn from(tgt_mode: &rc::ad::TgtMode) -> Self {
        match tgt_mode {
            rc::ad::TgtMode::None => Self::None,
            rc::ad::TgtMode::Item => Self::Item,
            rc::ad::TgtMode::Point => Self::Point,
        }
    }
}
impl Into<rc::ad::TgtMode> for &CTgtMode {
    fn into(self) -> rc::ad::TgtMode {
        match self {
            CTgtMode::None => rc::ad::TgtMode::None,
            CTgtMode::Item => rc::ad::TgtMode::Item,
            CTgtMode::Point => rc::ad::TgtMode::Point,
        }
    }
}
