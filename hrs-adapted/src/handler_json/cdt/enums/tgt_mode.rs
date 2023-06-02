#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json::cdt) enum TgtMode {
    None,
    Item,
    Point,
}
impl From<rc::consts::TgtMode> for TgtMode {
    fn from(value: rc::consts::TgtMode) -> Self {
        match value {
            rc::consts::TgtMode::None => Self::None,
            rc::consts::TgtMode::Item => Self::Item,
            rc::consts::TgtMode::Point => Self::Point,
        }
    }
}
impl Into<rc::consts::TgtMode> for TgtMode {
    fn into(self) -> rc::consts::TgtMode {
        match self {
            TgtMode::None => rc::consts::TgtMode::None,
            TgtMode::Item => rc::consts::TgtMode::Item,
            TgtMode::Point => rc::consts::TgtMode::Point,
        }
    }
}
