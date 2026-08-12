use serde::{Deserialize, de::Error};
use serde_json::{Map, Value};

pub(crate) struct EffectMod {
    func: String,
    args: Vec<(String, rc::ed::EPrimitive)>,
}
impl EffectMod {
    pub(crate) fn into_e_effect_mod(self) -> rc::ed::EEffectMod {
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

pub(crate) fn deser_effect_mods<'de, D>(json_mods: D) -> Result<Vec<EffectMod>, D::Error>
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
        mods.push(EffectMod { func, args })
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
