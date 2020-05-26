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
pub(super) struct Item {
    #[serde(rename = "groupID")]
    pub(super) group_id: ReeInt,
}
impl FsdMerge<dh::Item> for Item {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::Item> {
        vec![dh::Item::new(id, self.group_id)]
    }
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct ItemGroup {
    #[serde(rename = "categoryID")]
    pub(super) category_id: ReeInt,
}
impl FsdMerge<dh::ItemGroup> for ItemGroup {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::ItemGroup> {
        vec![dh::ItemGroup::new(id, self.category_id)]
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Dogma
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, serde::Deserialize)]
pub(super) struct Attr {
    pub(super) stackable: ReeInt,
    #[serde(rename = "highIsGood")]
    pub(super) high_is_good: ReeInt,
    #[serde(rename = "defaultValue")]
    pub(super) default_value: ReeFloat,
}
impl FsdMerge<dh::Attr> for Attr {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::Attr> {
        vec![dh::Attr::new(
            id,
            self.stackable != 0,
            self.high_is_good != 0,
            self.default_value,
        )]
    }
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct ItemAttrs {
    #[serde(rename = "dogmaAttributes", default)]
    pub(super) attrs: Vec<ItemAttrData>,
}
impl FsdMerge<dh::ItemAttr> for ItemAttrs {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::ItemAttr> {
        self.attrs
            .into_iter()
            .map(|v| dh::ItemAttr::new(id, v.attr_id, v.value))
            .collect()
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct ItemAttrData {
    #[serde(rename = "attributeID")]
    pub(super) attr_id: ReeInt,
    pub(super) value: ReeFloat,
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct Effect {
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
    pub(super) mods: Vec<EffectMod>,
}
impl FsdMerge<dh::Effect> for Effect {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::Effect> {
        vec![dh::Effect::new(
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
pub(super) struct EffectMod {
    pub(super) func: String,
    pub(super) args: HashMap<String, dh::Primitive>,
}
impl Into<dh::EffectMod> for EffectMod {
    fn into(self) -> dh::EffectMod {
        dh::EffectMod::new(self.func, self.args)
    }
}
mod dgmmod {
    use std::collections::HashMap;
    use std::result::Result;

    use serde::{de::Error, Deserialize};
    use serde_json::{Map, Value};

    use crate::defines::ReeFloat;
    use crate::dh::Primitive;

    use super::{dh, EffectMod, ReeInt};

    pub(super) fn deserialize<'de, D>(json_mods: D) -> Result<Vec<EffectMod>, D::Error>
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
pub(super) struct ItemEffects {
    #[serde(rename = "dogmaEffects", default)]
    pub(super) effects: Vec<ItemEffectData>,
}
impl FsdMerge<dh::ItemEffect> for ItemEffects {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::ItemEffect> {
        self.effects
            .into_iter()
            .map(|v| dh::ItemEffect::new(id, v.effect_id, v.is_default != 0))
            .collect()
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct ItemEffectData {
    #[serde(rename = "effectID")]
    pub(super) effect_id: ReeInt,
    #[serde(rename = "isDefault")]
    pub(super) is_default: ReeInt,
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct MutaItemConvs {
    #[serde(rename = "inputOutputMapping")]
    pub(super) item_maps: Vec<MutaItemMap>,
}
impl FsdMerge<dh::MutaItemConv> for MutaItemConvs {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::MutaItemConv> {
        let mut vec = Vec::new();
        for item_map in self.item_maps {
            for applicable_type in item_map.applicable_item_ids {
                vec.push(dh::MutaItemConv::new(id, applicable_type, item_map.result_item_id))
            }
        }
        vec
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct MutaItemMap {
    #[serde(rename = "applicableTypes")]
    pub(super) applicable_item_ids: Vec<ReeInt>,
    #[serde(rename = "resultingType")]
    pub(super) result_item_id: ReeInt,
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct MutaAttrMods {
    #[serde(rename = "attributeIDs")]
    pub(super) attrs: HashMap<ReeInt, MutaAttrModRange>,
}
impl FsdMerge<dh::MutaAttrMod> for MutaAttrMods {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::MutaAttrMod> {
        self.attrs
            .into_iter()
            .map(|(attr_id, range)| dh::MutaAttrMod::new(id, attr_id, range.min, range.max))
            .collect()
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct MutaAttrModRange {
    pub(super) min: ReeFloat,
    pub(super) max: ReeFloat,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Dogma Buffs
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, serde::Deserialize)]
pub(super) struct Buff {
    #[serde(rename = "aggregateMode")]
    pub(super) aggregate_mode: String,
    #[serde(rename = "operationName")]
    pub(super) operation: String,
    #[serde(rename = "itemModifiers")]
    pub(super) item_mods: Vec<BuffIM>,
    #[serde(rename = "locationModifiers")]
    pub(super) loc_mods: Vec<BuffLM>,
    #[serde(rename = "locationGroupModifiers")]
    pub(super) locgroup_mods: Vec<BuffLGM>,
    #[serde(rename = "locationRequiredSkillModifiers")]
    pub(super) locsrq_mods: Vec<BuffLRSM>,
}
impl FsdMerge<dh::Buff> for Buff {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::Buff> {
        vec![dh::Buff::new(
            id,
            self.aggregate_mode,
            self.operation,
            into_vec(self.item_mods),
            into_vec(self.loc_mods),
            into_vec(self.locgroup_mods),
            into_vec(self.locsrq_mods),
        )]
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct BuffIM {
    #[serde(rename = "dogmaAttributeID")]
    pub(super) attr_id: ReeInt,
}
impl Into<dh::BuffIM> for BuffIM {
    fn into(self) -> dh::BuffIM {
        dh::BuffIM::new(self.attr_id)
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct BuffLM {
    #[serde(rename = "dogmaAttributeID")]
    pub(super) attr_id: ReeInt,
}
impl Into<dh::BuffLM> for BuffLM {
    fn into(self) -> dh::BuffLM {
        dh::BuffLM::new(self.attr_id)
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct BuffLGM {
    #[serde(rename = "dogmaAttributeID")]
    pub(super) attr_id: ReeInt,
    #[serde(rename = "groupID")]
    pub(super) group_id: ReeInt,
}
impl Into<dh::BuffLGM> for BuffLGM {
    fn into(self) -> dh::BuffLGM {
        dh::BuffLGM::new(self.attr_id, self.group_id)
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct BuffLRSM {
    #[serde(rename = "dogmaAttributeID")]
    pub(super) attr_id: ReeInt,
    #[serde(rename = "skillID")]
    pub(super) skill_id: ReeInt,
}
impl Into<dh::BuffLRSM> for BuffLRSM {
    fn into(self) -> dh::BuffLRSM {
        dh::BuffLRSM::new(self.attr_id, self.skill_id)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fighter abilities
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, serde::Deserialize)]
pub(super) struct FighterAbil {
    #[serde(rename = "targetMode")]
    pub(super) target_mode: String,
    #[serde(rename = "disallowInHighSec")]
    pub(super) disallow_hisec: bool,
    #[serde(rename = "disallowInLowSec")]
    pub(super) disallow_lowsec: bool,
}
impl FsdMerge<dh::FighterAbil> for FighterAbil {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::FighterAbil> {
        vec![dh::FighterAbil::new(
            id,
            &self.target_mode,
            self.disallow_hisec,
            self.disallow_lowsec,
        )]
    }
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct ItemFighterAbils {
    #[serde(rename = "abilitySlot0")]
    pub(super) abil0: Option<ItemFighterAbilData>,
    #[serde(rename = "abilitySlot1")]
    pub(super) abil1: Option<ItemFighterAbilData>,
    #[serde(rename = "abilitySlot2")]
    pub(super) abil2: Option<ItemFighterAbilData>,
}
impl FsdMerge<dh::ItemFighterAbil> for ItemFighterAbils {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::ItemFighterAbil> {
        let mut vec = Vec::new();
        for abil_data in vec![self.abil0, self.abil1, self.abil2].into_iter() {
            if let Some(abil_data) = abil_data {
                let (charge_count, charge_rearm_time) = abil_data
                    .charges
                    .map(|v| (Some(v.count), Some(v.rearm_time)))
                    .unwrap_or_default();
                vec.push(dh::ItemFighterAbil::new(
                    id,
                    abil_data.abil_id,
                    abil_data.cooldown,
                    charge_count,
                    charge_rearm_time,
                ))
            }
        }
        vec
    }
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct ItemFighterAbilData {
    #[serde(rename = "abilityID")]
    pub(super) abil_id: ReeInt,
    #[serde(rename = "cooldownSeconds")]
    pub(super) cooldown: Option<ReeFloat>,
    pub(super) charges: Option<ItemFighterAbilChargeData>,
}
#[derive(Debug, serde::Deserialize)]
pub(super) struct ItemFighterAbilChargeData {
    #[serde(rename = "chargeCount")]
    pub(super) count: ReeInt,
    #[serde(rename = "rearmTimeSeconds")]
    pub(super) rearm_time: ReeFloat,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) type ItemSkillMap = HashMap<ReeInt, Vec<ReeInt>>;
impl FsdMerge<dh::ItemSkillReq> for ItemSkillMap {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::ItemSkillReq> {
        let mut vec = Vec::new();
        for (skill_id, levels) in self.into_iter() {
            if let Some(level) = levels.first() {
                vec.push(dh::ItemSkillReq::new(id, skill_id, *level))
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
