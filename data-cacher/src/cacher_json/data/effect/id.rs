use std::str::FromStr;

use crate::cacher_json::data::{CCustomEffectId, CDogmaEffectId, CItemId};

const D_PREFIX: &str = "d";
const SCSW_PREFIX: &str = "scsw";
const SCSE_PREFIX: &str = "scse";
const SCPE_PREFIX: &str = "scpe";
const SCPT_PREFIX: &str = "scpt";
const SCSL_PREFIX: &str = "scsl";
const C_PREFIX: &str = "c";

#[derive(Eq, PartialEq, Hash)]
pub(in crate::cacher_json) enum CEffectId {
    Dogma(CDogmaEffectId),
    ScSystemWide(CItemId),
    ScSystemEmitter(CItemId),
    ScProxyEffect(CItemId),
    ScProxyTrap(CItemId),
    ScShipLink(CItemId),
    Custom(CCustomEffectId),
}
impl From<&rc::ad::AEffectId> for CEffectId {
    fn from(a_effect_id: &rc::ad::AEffectId) -> Self {
        match a_effect_id {
            rc::ad::AEffectId::Dogma(id) => Self::Dogma(*id),
            rc::ad::AEffectId::ScSystemWide(id) => Self::ScSystemWide(*id),
            rc::ad::AEffectId::ScSystemEmitter(id) => Self::ScSystemEmitter(*id),
            rc::ad::AEffectId::ScProxyEffect(id) => Self::ScProxyEffect(*id),
            rc::ad::AEffectId::ScProxyTrap(id) => Self::ScProxyTrap(*id),
            rc::ad::AEffectId::ScShipLink(id) => Self::ScShipLink(*id),
            rc::ad::AEffectId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl From<&CEffectId> for rc::ad::AEffectId {
    fn from(c_effect_id: &CEffectId) -> Self {
        match c_effect_id {
            CEffectId::Dogma(id) => Self::Dogma(*id),
            CEffectId::ScSystemWide(id) => Self::ScSystemWide(*id),
            CEffectId::ScSystemEmitter(id) => Self::ScSystemEmitter(*id),
            CEffectId::ScProxyEffect(id) => Self::ScProxyEffect(*id),
            CEffectId::ScProxyTrap(id) => Self::ScProxyTrap(*id),
            CEffectId::ScShipLink(id) => Self::ScShipLink(*id),
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
            Self::ScSystemWide(id) => format!("{SCSW_PREFIX}{id}"),
            Self::ScSystemEmitter(id) => format!("{SCSE_PREFIX}{id}"),
            Self::ScProxyEffect(id) => format!("{SCPE_PREFIX}{id}"),
            Self::ScProxyTrap(id) => format!("{SCPT_PREFIX}{id}"),
            Self::ScShipLink(id) => format!("{SCSL_PREFIX}{id}"),
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
                if let Some(id_str) = v.strip_prefix(D_PREFIX) {
                    let id = CDogmaEffectId::from_str(id_str).map_err(|e| serde::de::Error::custom(e))?;
                    return Ok(Self::Value::Dogma(id));
                }
                if let Some(id_str) = v.strip_prefix(SCSW_PREFIX) {
                    let id = CItemId::from_str(id_str).map_err(|e| serde::de::Error::custom(e))?;
                    return Ok(Self::Value::ScSystemWide(id));
                }
                if let Some(id_str) = v.strip_prefix(SCSE_PREFIX) {
                    let id = CItemId::from_str(id_str).map_err(|e| serde::de::Error::custom(e))?;
                    return Ok(Self::Value::ScSystemEmitter(id));
                }
                if let Some(id_str) = v.strip_prefix(SCPE_PREFIX) {
                    let id = CItemId::from_str(id_str).map_err(|e| serde::de::Error::custom(e))?;
                    return Ok(Self::Value::ScProxyEffect(id));
                }
                if let Some(id_str) = v.strip_prefix(SCPT_PREFIX) {
                    let id = CItemId::from_str(id_str).map_err(|e| serde::de::Error::custom(e))?;
                    return Ok(Self::Value::ScProxyTrap(id));
                }
                if let Some(id_str) = v.strip_prefix(SCSL_PREFIX) {
                    let id = CItemId::from_str(id_str).map_err(|e| serde::de::Error::custom(e))?;
                    return Ok(Self::Value::ScShipLink(id));
                }
                if let Some(id_str) = v.strip_prefix(C_PREFIX) {
                    let id = CCustomEffectId::from_str(id_str).map_err(|e| serde::de::Error::custom(e))?;
                    return Ok(Self::Value::Custom(id));
                }
                let msg = format!(
                    "expected an int prefixed by \"{D_PREFIX}\", \"{SCSW_PREFIX}\", \"{SCSE_PREFIX}\", \
                    \"{SCPE_PREFIX}\", \"{SCPT_PREFIX}\", \"{SCSL_PREFIX}\", or \"{C_PREFIX}\", got \"{v}\""
                );
                Err(serde::de::Error::custom(msg))
            }
        }
        deserializer.deserialize_str(CEffectIdVisitor)
    }
}
