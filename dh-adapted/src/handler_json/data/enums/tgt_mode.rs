#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CTgtMode {
    None,
    Item,
    Point,
}
impl From<&rc::consts::TgtMode> for CTgtMode {
    fn from(tgt_mode: &rc::consts::TgtMode) -> Self {
        match tgt_mode {
            rc::consts::TgtMode::None => Self::None,
            rc::consts::TgtMode::Item => Self::Item,
            rc::consts::TgtMode::Point => Self::Point,
        }
    }
}
impl Into<rc::consts::TgtMode> for &CTgtMode {
    fn into(self) -> rc::consts::TgtMode {
        match self {
            CTgtMode::None => rc::consts::TgtMode::None,
            CTgtMode::Item => rc::consts::TgtMode::Item,
            CTgtMode::Point => rc::consts::TgtMode::Point,
        }
    }
}
