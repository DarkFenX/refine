#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json::cdt) enum ModDomain {
    Ship,
    Structure,
    Char,
    Item,
    Other,
}
impl From<rc::consts::ModDomain> for ModDomain {
    fn from(value: rc::consts::ModDomain) -> Self {
        match value {
            rc::consts::ModDomain::Ship => Self::Ship,
            rc::consts::ModDomain::Structure => Self::Structure,
            rc::consts::ModDomain::Char => Self::Char,
            rc::consts::ModDomain::Item => Self::Item,
            rc::consts::ModDomain::Other => Self::Other,
        }
    }
}
impl Into<rc::consts::ModDomain> for ModDomain {
    fn into(self) -> rc::consts::ModDomain {
        match self {
            ModDomain::Ship => rc::consts::ModDomain::Ship,
            ModDomain::Structure => rc::consts::ModDomain::Structure,
            ModDomain::Char => rc::consts::ModDomain::Char,
            ModDomain::Item => rc::consts::ModDomain::Item,
            ModDomain::Other => rc::consts::ModDomain::Other,
        }
    }
}
