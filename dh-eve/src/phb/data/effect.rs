use std::collections::HashMap;

use itertools::Itertools;

use crate::{phb::fsd::FsdMerge, util::into_opt};

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Effect {
    #[serde(rename = "effectCategory")]
    pub(crate) category_id: rc::ReeInt,
    #[serde(rename = "isAssistance")]
    pub(crate) is_assistance: rc::ReeInt,
    #[serde(rename = "isOffensive")]
    pub(crate) is_offensive: rc::ReeInt,
    #[serde(rename = "dischargeAttributeID")]
    pub(crate) discharge_attr_id: Option<rc::ReeInt>,
    #[serde(rename = "durationAttributeID")]
    pub(crate) duration_attr_id: Option<rc::ReeInt>,
    #[serde(rename = "rangeAttributeID")]
    pub(crate) range_attr_id: Option<rc::ReeInt>,
    #[serde(rename = "falloffAttributeID")]
    pub(crate) falloff_attr_id: Option<rc::ReeInt>,
    #[serde(rename = "trackingSpeedAttributeID")]
    pub(crate) tracking_attr_id: Option<rc::ReeInt>,
    #[serde(rename = "fittingUsageChanceAttributeID")]
    pub(crate) usage_chance_attr_id: Option<rc::ReeInt>,
    #[serde(rename = "resistanceAttributeID")]
    pub(crate) resist_attr_id: Option<rc::ReeInt>,
    #[serde(rename = "modifierInfo", default, deserialize_with = "dgmmod::deserialize")]
    pub(crate) mods: Vec<EffectMod>,
}
impl FsdMerge<rc::ed::EEffect> for Effect {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::ed::EEffect> {
        vec![rc::ed::EEffect::new(
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
pub(crate) struct EffectMod {
    pub(crate) func: String,
    pub(crate) args: HashMap<String, rc::ed::EPrimitive>,
}
impl Into<rc::ed::EEffectMod> for EffectMod {
    fn into(self) -> rc::ed::EEffectMod {
        rc::ed::EEffectMod::new(self.func, self.args)
    }
}

mod dgmmod {
    use std::{collections::HashMap, result::Result};

    use serde::{de::Error, Deserialize};
    use serde_json::{Map, Value};

    use super::EffectMod;

    pub(crate) fn deserialize<'de, D>(json_mods: D) -> Result<Vec<EffectMod>, D::Error>
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

    fn primitivize<E: Error>(json: Value) -> Result<rc::ed::EPrimitive, E> {
        match json {
            Value::Null => Ok(rc::ed::EPrimitive::Null),
            Value::Bool(b) => Ok(rc::ed::EPrimitive::Bool(b)),
            Value::Number(n) => {
                if let Some(n) = n.as_i64() {
                    Ok(rc::ed::EPrimitive::Int(n as rc::ReeInt))
                } else if let Some(n) = n.as_f64() {
                    Ok(rc::ed::EPrimitive::Float(n as rc::ReeFloat))
                } else {
                    Err(Error::custom("unexpected number type"))
                }
            }
            Value::String(s) => Ok(rc::ed::EPrimitive::String(s)),
            _ => Err(Error::custom("unexpected type")),
        }
    }
}
