use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    phb::{
        fsd::{FsdId, FsdMerge},
        serde_custom::bool_from_int,
    },
    util::into_opt,
};

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PEffect {
    #[serde(rename = "effectCategory")]
    pub(in crate::phb) category_id: rc::EEffectCatId,
    #[serde(rename = "isAssistance", deserialize_with = "bool_from_int")]
    pub(in crate::phb) is_assistance: bool,
    #[serde(rename = "isOffensive", deserialize_with = "bool_from_int")]
    pub(in crate::phb) is_offensive: bool,
    #[serde(rename = "dischargeAttributeID")]
    pub(in crate::phb) discharge_attr_id: Option<rc::EAttrId>,
    #[serde(rename = "durationAttributeID")]
    pub(in crate::phb) duration_attr_id: Option<rc::EAttrId>,
    #[serde(rename = "rangeAttributeID")]
    pub(in crate::phb) range_attr_id: Option<rc::EAttrId>,
    #[serde(rename = "falloffAttributeID")]
    pub(in crate::phb) falloff_attr_id: Option<rc::EAttrId>,
    #[serde(rename = "trackingSpeedAttributeID")]
    pub(in crate::phb) tracking_attr_id: Option<rc::EAttrId>,
    #[serde(rename = "fittingUsageChanceAttributeID")]
    pub(in crate::phb) usage_chance_attr_id: Option<rc::EAttrId>,
    #[serde(rename = "resistanceAttributeID")]
    pub(in crate::phb) resist_attr_id: Option<rc::EAttrId>,
    #[serde(rename = "modifierInfo", default, deserialize_with = "dgmmod::deserialize")]
    pub(in crate::phb) mods: Vec<PEffectMod>,
}
impl FsdMerge<rc::ed::EEffect> for PEffect {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EEffect> {
        vec![rc::ed::EEffect::new(
            id,
            self.category_id,
            self.is_assistance,
            self.is_offensive,
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

pub(in crate::phb) struct PEffectMod {
    pub(in crate::phb) func: String,
    pub(in crate::phb) args: HashMap<String, rc::ed::EPrimitive>,
}
impl From<PEffectMod> for rc::ed::EEffectMod {
    fn from(p_effect_mod: PEffectMod) -> Self {
        Self::new(p_effect_mod.func, (&p_effect_mod.args).into())
    }
}

mod dgmmod {
    use std::collections::HashMap;

    use serde::{Deserialize, de::Error};
    use serde_json::{Map, Value};

    use super::PEffectMod;

    pub(in crate::phb) fn deserialize<'de, D>(json_mods: D) -> Result<Vec<PEffectMod>, D::Error>
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
                    .map_err(|e| Error::custom(format!("failed to parse argument \"{argname}\" value: {e}")))?;
                argmap.insert(argname, argval);
            }
            mods.push(PEffectMod { func, args: argmap })
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
            _ => Err(Error::custom(format!("unexpected type of {key} value"))),
        }
    }

    fn primitivize<E: Error>(json: Value) -> Result<rc::ed::EPrimitive, E> {
        match json {
            Value::Null => Ok(rc::ed::EPrimitive::Null),
            Value::Bool(b) => Ok(rc::ed::EPrimitive::Bool(b)),
            Value::Number(n) => {
                if let Some(n) = n.as_i64() {
                    Ok(rc::ed::EPrimitive::Int(n as i32))
                } else if let Some(n) = n.as_f64() {
                    Ok(rc::ed::EPrimitive::Float(n))
                } else {
                    Err(Error::custom("unexpected number type"))
                }
            }
            Value::String(s) => Ok(rc::ed::EPrimitive::String(s)),
            _ => Err(Error::custom("unexpected type")),
        }
    }
}
