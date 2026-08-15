use crate::{CtlCmdResps, FitId, FleetId, ItemId, err::BackrefRenderError};

pub(crate) trait CtlCmdBackref {
    type Target;
    fn render(self, ctl_cmd_resps: &CtlCmdResps) -> Result<Self::Target, BackrefRenderError>;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fleet
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub enum FleetIdBackref {
    Id(FleetId),
    Backref(usize),
}
impl CtlCmdBackref for FleetIdBackref {
    type Target = FleetId;
    fn render(self, ctl_cmd_resps: &CtlCmdResps) -> Result<Self::Target, BackrefRenderError> {
        ctl_cmd_resps.render_fleet_id(self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fit
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub enum FitIdBackref {
    Id(FitId),
    Backref(usize),
}
impl CtlCmdBackref for FitIdBackref {
    type Target = FitId;
    fn render(self, ctl_cmd_resps: &CtlCmdResps) -> Result<Self::Target, BackrefRenderError> {
        ctl_cmd_resps.render_fit_id(self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Item
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub enum ItemIdBackref {
    Id(ItemId),
    BackrefMain(usize),
    BackrefCharge(usize),
}
impl CtlCmdBackref for ItemIdBackref {
    type Target = ItemId;
    fn render(self, ctl_cmd_resps: &CtlCmdResps) -> Result<Self::Target, BackrefRenderError> {
        ctl_cmd_resps.render_item_id(self)
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

    impl<'de> Deserialize<'de> for FleetIdBackref {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = FleetIdBackref;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("fleet ID, or #-prefixed backreference")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    if let Some(value_str) = v.strip_prefix(BACKREF_PREFIX) {
                        let index = usize::from_str(value_str).map_err(|e| Error::custom(e))?;
                        return Ok(Self::Value::Backref(index));
                    }
                    let fleet_id = FleetId::from_str(v).map_err(|e| Error::custom(e))?;
                    Ok(Self::Value::Id(fleet_id))
                }
            }
            deserializer.deserialize_string(VisitorImpl)
        }
    }

    impl<'de> Deserialize<'de> for FitIdBackref {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = FitIdBackref;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("fit ID, or #-prefixed backreference")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    if let Some(value_str) = v.strip_prefix(BACKREF_PREFIX) {
                        let index = usize::from_str(value_str).map_err(|e| Error::custom(e))?;
                        return Ok(Self::Value::Backref(index));
                    }
                    let fit_id = FitId::from_str(v).map_err(|e| Error::custom(e))?;
                    Ok(Self::Value::Id(fit_id))
                }
            }
            deserializer.deserialize_string(VisitorImpl)
        }
    }

    impl<'de> Deserialize<'de> for ItemIdBackref {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = ItemIdBackref;

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
                            return Ok(Self::Value::BackrefCharge(index));
                        }
                        let index = usize::from_str(value_str).map_err(|e| Error::custom(e))?;
                        return Ok(Self::Value::BackrefMain(index));
                    }
                    let item_id = ItemId::from_str(v).map_err(|e| Error::custom(e))?;
                    Ok(Self::Value::Id(item_id))
                }
            }
            deserializer.deserialize_string(VisitorImpl)
        }
    }
}
