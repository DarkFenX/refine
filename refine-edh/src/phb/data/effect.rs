use serde::Deserialize;

use crate::phb::data::{Key, KeyMergeOne, shared::int_to_bool};

#[derive(Deserialize)]
pub(in crate::phb) struct PEffect {
    #[serde(rename = "effectCategory")]
    effect_category: i32,
    #[serde(rename = "isAssistance", deserialize_with = "bool_from_int")]
    is_assistance: bool,
    #[serde(rename = "isOffensive", deserialize_with = "bool_from_int")]
    is_offensive: bool,
    #[serde(rename = "dischargeAttributeID")]
    discharge_attribute_id: Option<i32>,
    #[serde(rename = "durationAttributeID")]
    duration_attribute_id: Option<i32>,
    #[serde(rename = "rangeAttributeID")]
    range_attribute_id: Option<i32>,
    #[serde(rename = "falloffAttributeID")]
    falloff_attribute_id: Option<i32>,
    #[serde(rename = "trackingSpeedAttributeID")]
    tracking_attribute_id: Option<i32>,
    #[serde(rename = "fittingUsageChanceAttributeID")]
    fitting_usage_chance_attribute_id: Option<i32>,
    #[serde(rename = "resistanceAttributeID")]
    resistance_attribute_id: Option<i32>,
    #[serde(rename = "modifierInfo", default, deserialize_with = "serde_custom::deserialize")]
    modifier_info: Vec<PEffectMod>,
}
impl KeyMergeOne<rc::ed::EEffect> for PEffect {
    fn key_merge(self, key: Key) -> Vec<rc::ed::EEffect> {
        vec![rc::ed::EEffect {
            id: rc::ed::EEffectId::from_i32(key),
            category_id: rc::ed::EEffectCatId::from_i32(self.effect_category),
            is_assistance: self.is_assistance,
            is_offensive: self.is_offensive,
            discharge_attr_id: self.discharge_attribute_id.map(rc::ed::EAttrId::from_i32),
            duration_attr_id: self.duration_attribute_id.map(rc::ed::EAttrId::from_i32),
            range_attr_id: self.range_attribute_id.map(rc::ed::EAttrId::from_i32),
            falloff_attr_id: self.falloff_attribute_id.map(rc::ed::EAttrId::from_i32),
            tracking_attr_id: self.tracking_attribute_id.map(rc::ed::EAttrId::from_i32),
            usage_chance_attr_id: self.fitting_usage_chance_attribute_id.map(rc::ed::EAttrId::from_i32),
            resist_attr_id: self.resistance_attribute_id.map(rc::ed::EAttrId::from_i32),
            mods: self
                .modifier_info
                .into_iter()
                .map(|p_effect_mod| p_effect_mod.into_e_effect_mod())
                .collect(),
        }]
    }
}

struct PEffectMod {
    func: String,
    args: Vec<(String, rc::ed::EPrimitive)>,
}
impl PEffectMod {
    fn into_e_effect_mod(self) -> rc::ed::EEffectMod {
        rc::ed::EEffectMod {
            func: self.func,
            args: self
                .args
                .into_iter()
                .map(|(name, value)| rc::ed::EEffectModArg { name, value })
                .collect(),
        }
    }
}

mod serde_custom {
    use serde::{Deserialize, de::Error};
    use serde_json::{Map, Value};

    use super::PEffectMod;

    pub(super) fn deserialize<'de, D>(json_mods: D) -> Result<Vec<PEffectMod>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let func_field = "func";
        let mut mods = Vec::new();
        for json_mod in <Vec<Value>>::deserialize(json_mods)?.into_iter() {
            let mut json_mod_map = <Map<String, Value>>::deserialize(json_mod).map_err(Error::custom)?;
            let func = extract_string(&mut json_mod_map, func_field)?;
            let mut args = Vec::new();
            for (argname, v) in json_mod_map.into_iter() {
                let argval = primitivize::<D::Error>(v)
                    .map_err(|e| Error::custom(format!("failed to parse argument \"{argname}\" value: {e}")))?;
                args.push((argname, argval));
            }
            mods.push(PEffectMod { func, args })
        }
        Ok(mods)
    }

    fn extract_string<E: Error>(map: &mut Map<String, Value>, key: &'static str) -> Result<String, E> {
        let Some(value) = map.remove(key) else {
            return Err(Error::missing_field(key));
        };
        match value {
            Value::String(string) => Ok(string),
            _ => Err(Error::custom(format!("unexpected type of {key} value"))),
        }
    }

    fn primitivize<E: Error>(json: Value) -> Result<rc::ed::EPrimitive, E> {
        match json {
            Value::Null => Ok(rc::ed::EPrimitive::Null),
            Value::Bool(b) => Ok(rc::ed::EPrimitive::Bool(b)),
            Value::Number(n) => {
                if let Some(n) = n.as_i64() {
                    Ok(rc::ed::EPrimitive::Int(n.saturating_cast()))
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
