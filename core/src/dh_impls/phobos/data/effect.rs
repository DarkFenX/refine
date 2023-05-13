use std::collections::HashMap;

use itertools::Itertools;

use crate::{defs::ReeInt, dh};

use super::{super::fsd::FsdMerge, aux::into_opt};

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct Effect {
    #[serde(rename = "effectCategory")]
    pub(in super::super) category_id: ReeInt,
    #[serde(rename = "isAssistance")]
    pub(in super::super) is_assistance: ReeInt,
    #[serde(rename = "isOffensive")]
    pub(in super::super) is_offensive: ReeInt,
    #[serde(rename = "dischargeAttributeID")]
    pub(in super::super) discharge_attr_id: Option<ReeInt>,
    #[serde(rename = "durationAttributeID")]
    pub(in super::super) duration_attr_id: Option<ReeInt>,
    #[serde(rename = "rangeAttributeID")]
    pub(in super::super) range_attr_id: Option<ReeInt>,
    #[serde(rename = "falloffAttributeID")]
    pub(in super::super) falloff_attr_id: Option<ReeInt>,
    #[serde(rename = "trackingSpeedAttributeID")]
    pub(in super::super) tracking_attr_id: Option<ReeInt>,
    #[serde(rename = "fittingUsageChanceAttributeID")]
    pub(in super::super) usage_chance_attr_id: Option<ReeInt>,
    #[serde(rename = "resistanceAttributeID")]
    pub(in super::super) resist_attr_id: Option<ReeInt>,
    #[serde(rename = "modifierInfo", default, deserialize_with = "dgmmod::deserialize")]
    pub(in super::super) mods: Vec<EffectMod>,
}
impl FsdMerge<dh::Effect> for Effect {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::Effect> {
        vec![dh::Effect::new(
            id,
            self.category_id,
            self.is_assistance != 0,
            self.is_offensive != 0,
            into_opt(self.discharge_attr_id),
            into_opt(self.duration_attr_id),
            into_opt(self.range_attr_id),
            into_opt(self.falloff_attr_id),
            into_opt(self.tracking_attr_id),
            into_opt(self.usage_chance_attr_id),
            into_opt(self.resist_attr_id),
            self.mods.into_iter().map_into().collect(),
        )]
    }
}

#[derive(Debug)]
pub(in super::super) struct EffectMod {
    pub(in super::super) func: String,
    pub(in super::super) args: HashMap<String, dh::Primitive>,
}
impl Into<dh::EffectMod> for EffectMod {
    fn into(self) -> dh::EffectMod {
        dh::EffectMod::new(self.func, self.args)
    }
}

mod dgmmod {
    use std::{collections::HashMap, result::Result};

    use serde::{de::Error, Deserialize};
    use serde_json::{Map, Value};

    use crate::{
        defs::{ReeFloat, ReeInt},
        dh::{self, Primitive},
    };

    use super::EffectMod;

    pub(in super::super) fn deserialize<'de, D>(json_mods: D) -> Result<Vec<EffectMod>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let func_field = "func";
        let mut mods = Vec::new();
        for json_mod in <Vec<Value>>::deserialize(json_mods)?.into_iter() {
            let mut json_mod_map = <Map<String, Value>>::deserialize(json_mod).map_err(Error::custom)?;
            let func = extract_string(&mut json_mod_map, func_field)?;
            let mut argmap = HashMap::new();
            for (argname, v) in json_mod_map.into_iter() {
                let argval = primitivize::<D::Error>(v)
                    .map_err(|e| Error::custom(format!("failed to parse argument \"{}\" value: {}", argname, e)))?;
                argmap.insert(argname, argval);
            }
            mods.push(EffectMod { func, args: argmap })
        }
        Ok(mods)
    }

    fn extract_string<E: Error>(map: &mut Map<String, Value>, key: &'static str) -> Result<String, E> {
        let func = match map.remove(key) {
            Some(v) => v,
            None => return Err(Error::missing_field(key)),
        };
        match func {
            Value::String(s) => Ok(s.to_owned()),
            _ => return Err(Error::custom(format!("unexpected type of {} value", key))),
        }
    }

    fn primitivize<E: Error>(json: Value) -> Result<Primitive, E> {
        match json {
            Value::Null => Ok(dh::Primitive::Null),
            Value::Bool(b) => Ok(dh::Primitive::Bool(b)),
            Value::Number(n) => {
                if let Some(n) = n.as_i64() {
                    Ok(dh::Primitive::Int(n as ReeInt))
                } else if let Some(n) = n.as_f64() {
                    Ok(dh::Primitive::Float(n as ReeFloat))
                } else {
                    Err(Error::custom("unexpected number type"))
                }
            }
            Value::String(s) => Ok(dh::Primitive::String(s)),
            _ => Err(Error::custom("unexpected type")),
        }
    }
}
