#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CTgtMode {
    None,
    Item,
    Point,
}
impl From<&rc::ad::ATgtMode> for CTgtMode {
    fn from(tgt_mode: &rc::ad::ATgtMode) -> Self {
        match tgt_mode {
            rc::ad::ATgtMode::None => Self::None,
            rc::ad::ATgtMode::Item => Self::Item,
            rc::ad::ATgtMode::Point => Self::Point,
        }
    }
}
impl Into<rc::ad::ATgtMode> for &CTgtMode {
    fn into(self) -> rc::ad::ATgtMode {
        match self {
            CTgtMode::None => rc::ad::ATgtMode::None,
            CTgtMode::Item => rc::ad::ATgtMode::Item,
            CTgtMode::Point => rc::ad::ATgtMode::Point,
        }
    }
}
