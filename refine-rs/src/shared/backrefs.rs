use crate::{CmdResps, FitId, FleetId, ItemId, err::BrResolveError};

pub(crate) trait BrResolveInfallible {
    type Target;
    fn br_resolve_infallible(self, cmd_resps: &CmdResps) -> Self::Target;
}

pub(crate) trait BrResolveFallible {
    type Target;
    fn br_resolve_fallible(self, cmd_resps: &CmdResps) -> Result<Self::Target, BrResolveError>;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fleet
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Backreference type for fleet IDs.
///
/// They allow you to refer fleets created by some previous command in the command batch you are
/// submitting.
#[derive(Copy, Clone)]
pub enum FleetIdBr {
    Id(FleetId),
    Br(usize),
}
impl From<FleetId> for FleetIdBr {
    fn from(fleet_id: FleetId) -> Self {
        FleetIdBr::Id(fleet_id)
    }
}
impl BrResolveFallible for FleetIdBr {
    type Target = FleetId;
    fn br_resolve_fallible(self, cmd_resps: &CmdResps) -> Result<Self::Target, BrResolveError> {
        cmd_resps.resolve_fleet_id(self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fit
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Backreference type for fit IDs.
///
/// They allow you to refer fits created by some previous command in the command batch you are
/// submitting.
#[derive(Copy, Clone)]
pub enum FitIdBr {
    Id(FitId),
    Br(usize),
}
impl From<FitId> for FitIdBr {
    fn from(fit_id: FitId) -> Self {
        FitIdBr::Id(fit_id)
    }
}
impl BrResolveFallible for FitIdBr {
    type Target = FitId;
    fn br_resolve_fallible(self, cmd_resps: &CmdResps) -> Result<Self::Target, BrResolveError> {
        cmd_resps.resolve_fit_id(self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Item
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Backreference type for item IDs.
///
/// They allow you to refer items created by some previous command in the command batch you are
/// submitting.
#[derive(Copy, Clone)]
pub enum ItemIdBr {
    Id(ItemId),
    BrMain(usize),
    BrCharge(usize),
}
impl From<ItemId> for ItemIdBr {
    fn from(item_id: ItemId) -> Self {
        ItemIdBr::Id(item_id)
    }
}
impl BrResolveFallible for ItemIdBr {
    type Target = ItemId;
    fn br_resolve_fallible(self, cmd_resps: &CmdResps) -> Result<Self::Target, BrResolveError> {
        cmd_resps.resolve_item_id(self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use std::str::FromStr;

    use serde::de::{Deserialize, Deserializer, Error, Visitor};

    use super::*;

    const BACKREF_PREFIX: &str = "#";
    const CHARGE_SUFFIX: &str = "c";

    impl<'de> Deserialize<'de> for FleetIdBr {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = FleetIdBr;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("fleet ID, or #-prefixed backreference")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    if let Some(value_str) = v.strip_prefix(BACKREF_PREFIX) {
                        let index = usize::from_str(value_str).map_err(|e| Error::custom(e))?;
                        return Ok(Self::Value::Br(index));
                    }
                    let fleet_id = FleetId::from_str(v).map_err(|e| Error::custom(e))?;
                    Ok(Self::Value::Id(fleet_id))
                }
            }
            deserializer.deserialize_string(VisitorImpl)
        }
    }

    impl<'de> Deserialize<'de> for FitIdBr {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = FitIdBr;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("fit ID, or #-prefixed backreference")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    if let Some(value_str) = v.strip_prefix(BACKREF_PREFIX) {
                        let index = usize::from_str(value_str).map_err(|e| Error::custom(e))?;
                        return Ok(Self::Value::Br(index));
                    }
                    let fit_id = FitId::from_str(v).map_err(|e| Error::custom(e))?;
                    Ok(Self::Value::Id(fit_id))
                }
            }
            deserializer.deserialize_string(VisitorImpl)
        }
    }

    impl<'de> Deserialize<'de> for ItemIdBr {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = ItemIdBr;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("item ID, or #-prefixed backreference")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    if let Some(value_str) = v.strip_prefix(BACKREF_PREFIX) {
                        if let Some(value_str) = value_str.strip_suffix(CHARGE_SUFFIX) {
                            let index = usize::from_str(value_str).map_err(|e| Error::custom(e))?;
                            return Ok(Self::Value::BrCharge(index));
                        }
                        let index = usize::from_str(value_str).map_err(|e| Error::custom(e))?;
                        return Ok(Self::Value::BrMain(index));
                    }
                    let item_id = ItemId::from_str(v).map_err(|e| Error::custom(e))?;
                    Ok(Self::Value::Id(item_id))
                }
            }
            deserializer.deserialize_string(VisitorImpl)
        }
    }
}
