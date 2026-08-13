use serde::Deserialize;
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
        let mut json_mod_map = <Map<String, Value>>::deserialize(json_mod).map_err(serde::de::Error::custom)?;
        let func = extract_string(&mut json_mod_map, func_field)?;
        let mut args = Vec::new();
        for (argname, v) in json_mod_map.into_iter() {
            let argval = primitivize::<D::Error>(v)
                .map_err(|e| serde::de::Error::custom(format!("failed to parse argument \"{argname}\" value: {e}")))?;
            args.push((argname, argval));
        }
        mods.push(EffectMod { func, args })
    }
    Ok(mods)
}

fn extract_string<E>(map: &mut Map<String, Value>, key: &'static str) -> Result<String, E>
where
    E: serde::de::Error,
{
    let Some(value) = map.remove(key) else {
        return Err(serde::de::Error::missing_field(key));
    };
    match value {
        Value::String(string) => Ok(string),
        _ => Err(serde::de::Error::custom(format!("unexpected type of {key} value"))),
    }
}

fn primitivize<E>(json: Value) -> Result<rc::ed::EPrimitive, E>
where
    E: serde::de::Error,
{
    match json {
        Value::Null => Ok(rc::ed::EPrimitive::Null),
        Value::Bool(value) => Ok(rc::ed::EPrimitive::Bool(value)),
        Value::Number(value) => match (value.as_i64(), value.as_f64()) {
            (Some(value), _) => Ok(rc::ed::EPrimitive::Int(value.saturating_cast())),
            (None, Some(value)) => Ok(rc::ed::EPrimitive::Float(value)),
            (None, None) => Err(serde::de::Error::custom("unexpected number type")),
        },
        Value::String(value) => Ok(rc::ed::EPrimitive::String(value)),
        _ => Err(serde::de::Error::custom("unexpected type")),
    }
}
