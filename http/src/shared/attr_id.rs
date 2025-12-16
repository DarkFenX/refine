use std::str::FromStr;

const C_PREFIX: &str = "c";

#[derive(Eq, PartialEq, Hash)]
pub(crate) enum HAttrId {
    Eve(rc::EveAttrId),
    Custom(rc::CustomAttrId),
}
impl From<&rc::AttrId> for HAttrId {
    fn from(core_attr_id: &rc::AttrId) -> Self {
        match core_attr_id {
            rc::AttrId::Eve(id) => Self::Eve(*id),
            rc::AttrId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl From<rc::AttrId> for HAttrId {
    fn from(core_attr_id: rc::AttrId) -> Self {
        match core_attr_id {
            rc::AttrId::Eve(id) => Self::Eve(id),
            rc::AttrId::Custom(id) => Self::Custom(id),
        }
    }
}
impl From<&HAttrId> for rc::AttrId {
    fn from(h_attr_id: &HAttrId) -> Self {
        match h_attr_id {
            HAttrId::Eve(id) => Self::Eve(*id),
            HAttrId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl From<HAttrId> for rc::AttrId {
    fn from(h_attr_id: HAttrId) -> Self {
        match h_attr_id {
            HAttrId::Eve(id) => Self::Eve(id),
            HAttrId::Custom(id) => Self::Custom(id),
        }
    }
}
impl serde::Serialize for HAttrId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let string = match self {
            Self::Eve(id) => format!("{id}"),
            Self::Custom(id) => format!("{C_PREFIX}{id}"),
        };
        serializer.serialize_str(&string)
    }
}
impl<'de> serde::Deserialize<'de> for HAttrId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct HAttrIdVisitor;

        impl<'de> serde::de::Visitor<'de> for HAttrIdVisitor {
            type Value = HAttrId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("attr type-prefixed int HAttrId")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(id_str) = v.strip_prefix(C_PREFIX) {
                    let id = rc::CustomAttrId::from_str(id_str).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::Custom(id));
                }

                let msg = format!("expected an int, or int prefixed by \"{C_PREFIX}\", got \"{v}\"");
                Err(serde::de::Error::custom(msg))
            }
        }
        deserializer.deserialize_str(HAttrIdVisitor)
    }
}
