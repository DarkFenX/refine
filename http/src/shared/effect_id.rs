use std::str::FromStr;

const D_PREFIX: &str = "d";
const SC_PREFIX: &str = "sc";
const C_PREFIX: &str = "c";

#[derive(Eq, PartialEq, Hash)]
pub(crate) enum HEffectId {
    Dogma(rc::DogmaEffectId),
    SpaceComponent(rc::ItemTypeId),
    Custom(rc::CustomEffectId),
}
impl From<&rc::EffectId> for HEffectId {
    fn from(core_effect_id: &rc::EffectId) -> Self {
        match core_effect_id {
            rc::EffectId::Dogma(id) => Self::Dogma(*id),
            rc::EffectId::SpaceComponent(id) => Self::SpaceComponent(*id),
            rc::EffectId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl From<rc::EffectId> for HEffectId {
    fn from(core_effect_id: rc::EffectId) -> Self {
        match core_effect_id {
            rc::EffectId::Dogma(id) => Self::Dogma(id),
            rc::EffectId::SpaceComponent(id) => Self::SpaceComponent(id),
            rc::EffectId::Custom(id) => Self::Custom(id),
        }
    }
}
impl From<&HEffectId> for rc::EffectId {
    fn from(h_effect_id: &HEffectId) -> Self {
        match h_effect_id {
            HEffectId::Dogma(id) => Self::Dogma(*id),
            HEffectId::SpaceComponent(id) => Self::SpaceComponent(*id),
            HEffectId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl serde::Serialize for HEffectId {
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
impl<'de> serde::Deserialize<'de> for HEffectId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct CEffectIdVisitor;

        impl<'de> serde::de::Visitor<'de> for CEffectIdVisitor {
            type Value = HEffectId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Duration")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.starts_with(D_PREFIX) {
                    let id =
                        rc::DogmaEffectId::from_str(&v[D_PREFIX.len()..]).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::Dogma(id));
                }
                if v.starts_with(SC_PREFIX) {
                    let id =
                        rc::ItemTypeId::from_str(&v[SC_PREFIX.len()..]).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::SpaceComponent(id));
                }
                if v.starts_with(C_PREFIX) {
                    let id =
                        rc::CustomEffectId::from_str(&v[C_PREFIX.len()..]).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::Custom(id));
                }
                let msg =
                    format!("expected an int prefixed by \"{D_PREFIX}\", \"{SC_PREFIX}\", or \"{C_PREFIX}\", got \"\"");
                Err(serde::de::Error::custom(msg))
            }
        }
        deserializer.deserialize_enum("CEffectId", &["Dogma", "SpaceComponent", "Custom"], CEffectIdVisitor)
    }
}
