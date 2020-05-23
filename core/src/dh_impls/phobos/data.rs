use std::collections::HashMap;

use crate::defines::{ReeFloat, ReeInt};
use crate::dh;

fn into_opt<T: Into<U>, U>(v: Option<T>) -> Option<U> {
    v.map(Into::into)
}
fn into_vec<T: Into<U>, U>(v: Vec<T>) -> Vec<U> {
    v.into_iter().map(|v| v.into()).collect()
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inventory
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, serde::Deserialize)]
pub(super) struct InvType {
    pub(super) id: ReeInt,
    #[serde(rename = "groupID")]
    pub(super) group_id: ReeInt,
}
impl Into<dh::InvType> for InvType {
    fn into(self) -> dh::InvType {
        dh::InvType::new(self.id, self.group_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct InvGroup {
    pub(super) id: ReeInt,
    #[serde(rename = "categoryID")]
    pub(super) category_id: ReeInt,
}
impl Into<dh::InvGroup> for InvGroup {
    fn into(self) -> dh::InvGroup {
        dh::InvGroup::new(self.id, self.category_id)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Dogma
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmAttr {
    pub(super) id: ReeInt,
    pub(super) stackable: ReeInt,
    #[serde(rename = "highIsGood")]
    pub(super) high_is_good: ReeInt,
    #[serde(rename = "defaultValue")]
    pub(super) default: ReeFloat,
}
impl Into<dh::DgmAttr> for DgmAttr {
    fn into(self) -> dh::DgmAttr {
        dh::DgmAttr::new(self.id, self.stackable != 0, self.high_is_good != 0, self.default)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmEffect {
    pub(super) id: ReeInt,
    #[serde(rename = "effectCategory")]
    pub(super) category_id: ReeInt,
    #[serde(rename = "isAssistance")]
    pub(super) is_assistance: ReeInt,
    #[serde(rename = "isOffensive")]
    pub(super) is_offensive: ReeInt,
    #[serde(rename = "isWarpSafe")]
    pub(super) is_warp_safe: ReeInt,
    #[serde(rename = "dischargeAttributeID")]
    pub(super) discharge_attr_id: Option<ReeInt>,
    #[serde(rename = "durationAttributeID")]
    pub(super) duration_attr_id: Option<ReeInt>,
    #[serde(rename = "rangeAttributeID")]
    pub(super) range_attr_id: Option<ReeInt>,
    #[serde(rename = "falloffAttributeID")]
    pub(super) falloff_attr_id: Option<ReeInt>,
    #[serde(rename = "trackingSpeedAttributeID")]
    pub(super) tracking_attr_id: Option<ReeInt>,
    #[serde(rename = "fittingUsageChanceAttributeID")]
    pub(super) usage_chance_attr_id: Option<ReeInt>,
    #[serde(rename = "resistanceAttributeID")]
    pub(super) resist_attr_id: Option<ReeInt>,
    #[serde(rename = "modifierInfo", default, deserialize_with = "dgmmod::deserialize")]
    pub(super) mods: Vec<DgmEffectMod>,
}
impl Into<dh::DgmEffect> for DgmEffect {
    fn into(self) -> dh::DgmEffect {
        dh::DgmEffect::new(
            self.id,
            self.category_id,
            self.is_assistance != 0,
            self.is_offensive != 0,
            self.is_warp_safe != 0,
            into_opt(self.discharge_attr_id),
            into_opt(self.duration_attr_id),
            into_opt(self.range_attr_id),
            into_opt(self.falloff_attr_id),
            into_opt(self.tracking_attr_id),
            into_opt(self.usage_chance_attr_id),
            into_opt(self.resist_attr_id),
            self.mods.into_iter().map(|v| v.into()).collect(),
        )
    }
}
#[derive(Debug)]
pub(super) struct DgmEffectMod {
    pub(super) func: String,
    pub(super) args: HashMap<String, dh::Primitive>,
}
impl Into<dh::DgmEffectMod> for DgmEffectMod {
    fn into(self) -> dh::DgmEffectMod {
        dh::DgmEffectMod::new(self.func, self.args)
    }
}
mod dgmmod {
    use std::collections::HashMap;
    use std::result::Result;

    use serde::{de::Error, Deserialize};
    use serde_json::{Map, Value};

    use crate::defines::ReeFloat;
    use crate::dh::Primitive;

    use super::{DgmEffectMod, dh, ReeInt};

    pub(super) fn deserialize<'de, D>(json_mods: D) -> Result<Vec<DgmEffectMod>, D::Error>
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
            mods.push(DgmEffectMod { func, args: argmap })
        }
        Ok(mods)
    }

    fn extract_string<E>(map: &mut Map<String, Value>, key: &'static str) -> Result<String, E>
    where
        E: Error,
    {
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Dogma Buffs
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmBuff {
    pub(super) id: ReeInt,
    #[serde(rename = "aggregateMode")]
    pub(super) aggregate: String,
    #[serde(rename = "operationName")]
    pub(super) operation: String,
    #[serde(rename = "itemModifiers")]
    pub(super) item_mods: Vec<DgmBuffIM>,
    #[serde(rename = "locationModifiers")]
    pub(super) loc_mods: Vec<DgmBuffLM>,
    #[serde(rename = "locationGroupModifiers")]
    pub(super) locgroup_mods: Vec<DgmBuffLGM>,
    #[serde(rename = "locationRequiredSkillModifiers")]
    pub(super) locsrq_mods: Vec<DgmBuffLRSM>,
}
impl Into<dh::DgmBuff> for DgmBuff {
    fn into(self) -> dh::DgmBuff {
        dh::DgmBuff::new(
            self.id,
            self.aggregate,
            self.operation,
            into_vec(self.item_mods),
            into_vec(self.loc_mods),
            into_vec(self.locgroup_mods),
            into_vec(self.locsrq_mods),
        )
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmBuffIM {
    #[serde(rename = "dogmaAttributeID")]
    pub(super) attr_id: ReeInt,
}
impl Into<dh::DgmBuffIM> for DgmBuffIM {
    fn into(self) -> dh::DgmBuffIM {
        dh::DgmBuffIM::new(self.attr_id)
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmBuffLM {
    #[serde(rename = "dogmaAttributeID")]
    pub(super) attr_id: ReeInt,
}
impl Into<dh::DgmBuffLM> for DgmBuffLM {
    fn into(self) -> dh::DgmBuffLM {
        dh::DgmBuffLM::new(self.attr_id)
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmBuffLGM {
    #[serde(rename = "dogmaAttributeID")]
    pub(super) attr_id: ReeInt,
    #[serde(rename = "groupID")]
    pub(super) group_id: ReeInt,
}
impl Into<dh::DgmBuffLGM> for DgmBuffLGM {
    fn into(self) -> dh::DgmBuffLGM {
        dh::DgmBuffLGM::new(self.attr_id, self.group_id)
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmBuffLRSM {
    #[serde(rename = "dogmaAttributeID")]
    pub(super) attr_id: ReeInt,
    #[serde(rename = "skillID")]
    pub(super) skill_id: ReeInt,
}
impl Into<dh::DgmBuffLRSM> for DgmBuffLRSM {
    fn into(self) -> dh::DgmBuffLRSM {
        dh::DgmBuffLRSM::new(self.attr_id, self.skill_id)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fighter abilities
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, serde::Deserialize)]
pub(super) struct FtrAbil {
    pub(super) id: ReeInt,
    #[serde(rename = "targetMode")]
    pub(super) target_mode: String,
    #[serde(rename = "disallowInHighSec")]
    pub(super) disallow_hisec: bool,
    #[serde(rename = "disallowInLowSec")]
    pub(super) disallow_lowsec: bool,
}
impl Into<dh::FtrAbil> for FtrAbil {
    fn into(self) -> dh::FtrAbil {
        dh::FtrAbil::new(self.id, &self.target_mode, self.disallow_hisec, self.disallow_lowsec)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct FtrTypeAbil {
    pub(super) type_id: ReeInt,
    #[serde(rename = "abilitySlot0")]
    pub(super) abil0: Option<FtrTypeAbilExtras>,
    #[serde(rename = "abilitySlot1")]
    pub(super) abil1: Option<FtrTypeAbilExtras>,
    #[serde(rename = "abilitySlot2")]
    pub(super) abil2: Option<FtrTypeAbilExtras>,
}
impl Into<dh::FtrTypeAbil> for FtrTypeAbil {
    fn into(self) -> dh::FtrTypeAbil {
        dh::FtrTypeAbil::new(
            self.type_id,
            into_opt(self.abil0),
            into_opt(self.abil1),
            into_opt(self.abil2),
        )
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct FtrTypeAbilExtras {
    #[serde(rename = "abilityID")]
    pub(super) ability_id: ReeInt,
    #[serde(rename = "cooldownSeconds")]
    pub(super) cooldown: Option<ReeFloat>,
    pub(super) charges: Option<FtrTypeAbilChargeExtras>,
}
impl Into<dh::FtrTypeAbilExtras> for FtrTypeAbilExtras {
    fn into(self) -> dh::FtrTypeAbilExtras {
        dh::FtrTypeAbilExtras::new(self.ability_id, self.cooldown, into_opt(self.charges))
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct FtrTypeAbilChargeExtras {
    #[serde(rename = "chargeCount")]
    pub(super) count: ReeInt,
    #[serde(rename = "rearmTimeSeconds")]
    pub(super) rearm_time: ReeFloat,
}
impl Into<dh::FtrTypeAbilChargeExtras> for FtrTypeAbilChargeExtras {
    fn into(self) -> dh::FtrTypeAbilChargeExtras {
        dh::FtrTypeAbilChargeExtras::new(self.count, self.rearm_time)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, serde::Deserialize)]
pub(super) struct Metadata {
    pub(super) field_name: String,
    pub(super) field_value: u32,
}
