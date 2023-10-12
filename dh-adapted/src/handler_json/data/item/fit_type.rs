#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CFitType {
    Ship,
    Structure,
}
impl From<&rc::FitType> for CFitType {
    fn from(fit_type: &rc::FitType) -> Self {
        match fit_type {
            rc::FitType::Ship => Self::Ship,
            rc::FitType::Structure => Self::Structure,
        }
    }
}
impl Into<rc::FitType> for &CFitType {
    fn into(self) -> rc::FitType {
        match self {
            CFitType::Ship => rc::FitType::Ship,
            CFitType::Structure => rc::FitType::Structure,
        }
    }
}
