use std::collections::HashMap;

use crate::defines::{ReeFloat, ReeInt};
use crate::dh;

use super::fsd::FsdMerge;

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
    #[serde(rename = "groupID")]
    pub(super) group_id: ReeInt,
}
impl FsdMerge<dh::InvType> for InvType {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::InvType> {
        vec![dh::InvType::new(id, self.group_id)]
    }
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct InvGroup {
    #[serde(rename = "categoryID")]
    pub(super) category_id: ReeInt,
}
impl FsdMerge<dh::InvGroup> for InvGroup {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::InvGroup> {
        vec![dh::InvGroup::new(id, self.category_id)]
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Dogma
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmAttr {
    pub(super) stackable: ReeInt,
    #[serde(rename = "highIsGood")]
    pub(super) high_is_good: ReeInt,
    #[serde(rename = "defaultValue")]
    pub(super) default: ReeFloat,
}
impl FsdMerge<dh::DgmAttr> for DgmAttr {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::DgmAttr> {
        vec![dh::DgmAttr::new(
            id,
            self.stackable != 0,
            self.high_is_good != 0,
            self.default,
        )]
    }
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmTypeAttrs {
    #[serde(rename = "dogmaAttributes", default)]
    pub(super) attrs: Vec<DgmTypeAttrData>,
}
impl FsdMerge<dh::DgmTypeAttr> for DgmTypeAttrs {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::DgmTypeAttr> {
        self.attrs
            .into_iter()
            .map(|v| dh::DgmTypeAttr::new(id, v.attr_id, v.value))
            .collect()
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmTypeAttrData {
    #[serde(rename = "attributeID")]
    pub(super) attr_id: ReeInt,
    pub(super) value: ReeFloat,
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmEffect {
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
impl FsdMerge<dh::DgmEffect> for DgmEffect {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::DgmEffect> {
        vec![dh::DgmEffect::new(
            id,
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
        )]
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

    use super::{dh, DgmEffectMod, ReeInt};

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

#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmTypeEffects {
    #[serde(rename = "dogmaEffects", default)]
    pub(super) effects: Vec<DgmTypeEffectData>,
}
impl FsdMerge<dh::DgmTypeEffect> for DgmTypeEffects {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::DgmTypeEffect> {
        self.effects
            .into_iter()
            .map(|v| dh::DgmTypeEffect::new(id, v.effect_id, v.default != 0))
            .collect()
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmTypeEffectData {
    #[serde(rename = "effectID")]
    pub(super) effect_id: ReeInt,
    #[serde(rename = "isDefault")]
    pub(super) default: ReeInt,
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmMutaTypes {
    #[serde(rename = "inputOutputMapping")]
    pub(super) type_maps: Vec<DgmMutaTypeData>,
}
impl FsdMerge<dh::DgmMutaType> for DgmMutaTypes {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::DgmMutaType> {
        let mut vec = Vec::new();
        for type_map in self.type_maps {
            for applicable_type in type_map.applicable {
                vec.push(dh::DgmMutaType::new(id, applicable_type, type_map.result))
            }
        }
        vec
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmMutaTypeData {
    #[serde(rename = "applicableTypes")]
    pub(super) applicable: Vec<ReeInt>,
    #[serde(rename = "resultingType")]
    pub(super) result: ReeInt,
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmMutaAttrs {
    #[serde(rename = "attributeIDs")]
    pub(super) attrs: HashMap<ReeInt, DgmMutaAttrsRange>,
}
impl FsdMerge<dh::DgmMutaAttr> for DgmMutaAttrs {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::DgmMutaAttr> {
        self.attrs
            .into_iter()
            .map(|(attr_id, range)| dh::DgmMutaAttr::new(id, attr_id, range.min, range.max))
            .collect()
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmMutaAttrsRange {
    pub(super) min: ReeFloat,
    pub(super) max: ReeFloat,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Dogma Buffs
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, serde::Deserialize)]
pub(super) struct DgmBuff {
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
impl FsdMerge<dh::DgmBuff> for DgmBuff {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::DgmBuff> {
        vec![dh::DgmBuff::new(
            id,
            self.aggregate,
            self.operation,
            into_vec(self.item_mods),
            into_vec(self.loc_mods),
            into_vec(self.locgroup_mods),
            into_vec(self.locsrq_mods),
        )]
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
    #[serde(rename = "targetMode")]
    pub(super) target_mode: String,
    #[serde(rename = "disallowInHighSec")]
    pub(super) disallow_hisec: bool,
    #[serde(rename = "disallowInLowSec")]
    pub(super) disallow_lowsec: bool,
}
impl FsdMerge<dh::FtrAbil> for FtrAbil {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::FtrAbil> {
        vec![dh::FtrAbil::new(
            id,
            &self.target_mode,
            self.disallow_hisec,
            self.disallow_lowsec,
        )]
    }
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct FtrTypeAbil {
    #[serde(rename = "abilitySlot0")]
    pub(super) abil0: Option<FtrTypeAbilData>,
    #[serde(rename = "abilitySlot1")]
    pub(super) abil1: Option<FtrTypeAbilData>,
    #[serde(rename = "abilitySlot2")]
    pub(super) abil2: Option<FtrTypeAbilData>,
}
impl FsdMerge<dh::FtrTypeAbil> for FtrTypeAbil {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::FtrTypeAbil> {
        vec![dh::FtrTypeAbil::new(
            id,
            into_opt(self.abil0),
            into_opt(self.abil1),
            into_opt(self.abil2),
        )]
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct FtrTypeAbilData {
    #[serde(rename = "abilityID")]
    pub(super) ability_id: ReeInt,
    #[serde(rename = "cooldownSeconds")]
    pub(super) cooldown: Option<ReeFloat>,
    pub(super) charges: Option<FtrTypeAbilChargeData>,
}
impl Into<dh::FtrTypeAbilData> for FtrTypeAbilData {
    fn into(self) -> dh::FtrTypeAbilData {
        dh::FtrTypeAbilData::new(self.ability_id, self.cooldown, into_opt(self.charges))
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct FtrTypeAbilChargeData {
    #[serde(rename = "chargeCount")]
    pub(super) count: ReeInt,
    #[serde(rename = "rearmTimeSeconds")]
    pub(super) rearm_time: ReeFloat,
}
impl Into<dh::FtrTypeAbilChargeData> for FtrTypeAbilChargeData {
    fn into(self) -> dh::FtrTypeAbilChargeData {
        dh::FtrTypeAbilChargeData::new(self.count, self.rearm_time)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) type SkillReq = HashMap<ReeInt, Vec<ReeInt>>;
impl FsdMerge<dh::SkillReq> for SkillReq {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::SkillReq> {
        let mut vec = Vec::new();
        for (skill_id, levels) in self.into_iter() {
            if let Some(level) = levels.first() {
                vec.push(dh::SkillReq::new(id, skill_id, *level))
            }
        }
        vec
    }
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct Metadata {
    pub(super) field_name: String,
    pub(super) field_value: u32,
}
