use std::str::FromStr;

use crate::handler_json::data::{CCustomEffectId, CDogmaEffectId, CItemId};

const D_PREFIX: &str = "d";
const SC_PREFIX: &str = "sc";
const C_PREFIX: &str = "c";

#[derive(Eq, PartialEq, Hash)]
pub(in crate::handler_json) enum CEffectId {
    Dogma(CDogmaEffectId),
    SpaceComponent(CItemId),
    Custom(CCustomEffectId),
}
impl From<&rc::ad::AEffectId> for CEffectId {
    fn from(a_effect_id: &rc::ad::AEffectId) -> Self {
        match a_effect_id {
            rc::ad::AEffectId::Dogma(id) => Self::Dogma(*id),
            rc::ad::AEffectId::SpaceComponent(id) => Self::SpaceComponent(*id),
            rc::ad::AEffectId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl From<&CEffectId> for rc::ad::AEffectId {
    fn from(c_effect_id: &CEffectId) -> Self {
        match c_effect_id {
            CEffectId::Dogma(id) => Self::Dogma(*id),
            CEffectId::SpaceComponent(id) => Self::SpaceComponent(*id),
            CEffectId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl serde::Serialize for CEffectId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let string = match self {
            Self::Dogma(id) => format!("{D_PREFIX}{id}"),
            Self::SpaceComponent(id) => format!("{SC_PREFIX}{id}"),
            Self::Custom(id) => format!("{C_PREFIX}{id}"),
        };
        serializer.serialize_str(&string)
    }
}
impl<'de> serde::Deserialize<'de> for CEffectId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct CEffectIdVisitor;

        impl<'de> serde::de::Visitor<'de> for CEffectIdVisitor {
            type Value = CEffectId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("effect type-prefixed int CEffectId")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.starts_with(D_PREFIX) {
                    let id = CDogmaEffectId::from_str(&v[D_PREFIX.len()..]).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::Dogma(id));
                }
                if v.starts_with(SC_PREFIX) {
                    let id = CItemId::from_str(&v[SC_PREFIX.len()..]).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::SpaceComponent(id));
                }
                if v.starts_with(C_PREFIX) {
                    let id =
                        CCustomEffectId::from_str(&v[C_PREFIX.len()..]).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::Custom(id));
                }
                let msg =
                    format!("expected an int prefixed by \"{D_PREFIX}\", \"{SC_PREFIX}\", or \"{C_PREFIX}\", got \"\"");
                Err(serde::de::Error::custom(msg))
            }
        }
        deserializer.deserialize_str(CEffectIdVisitor)
    }
}
