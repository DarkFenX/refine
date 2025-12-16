use std::hash::Hash;

use crate::{
    ac,
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AModifierSrq, AOp,
        AState,
        generator::{GSupport, get_abil_effect},
    },
    ec,
    ed::{EData, EEffectId, EEffectMod, EFighterAbil, EItemGrpId, EItemId, EPrimitive},
    util::{RMap, RSet, StrMsgError},
};

impl EFighterAbil {
    fn get_disallow_hisec(&self) -> bool {
        self.disallow_hisec
    }
    fn get_disallow_lowsec(&self) -> bool {
        self.disallow_lowsec
    }
}

pub(in crate::ad::generator::flow::s6_conv_pre) fn conv_effects(
    e_data: &EData,
    g_supp: &GSupport,
) -> RMap<AEffectId, AEffect> {
    let mut a_effects = RMap::new();
    for e_effect in e_data.effects.data.iter() {
        let state = match e_effect.category_id {
            ec::effcats::PASSIVE => AState::Offline,
            ec::effcats::ACTIVE => AState::Active,
            ec::effcats::TARGET => AState::Active,
            ec::effcats::ONLINE => AState::Online,
            ec::effcats::OVERLOAD => AState::Overload,
            ec::effcats::SYSTEM => AState::Offline,
            _ => {
                let msg = format!("{} uses unknown effect category {}", e_effect, e_effect.category_id);
                tracing::warn!("{msg}");
                continue;
            }
        };
        let mut a_effect = AEffect {
            id: AEffectId::Dogma(e_effect.id),
            category: e_effect.category_id,
            state,
            buff: g_supp.eff_buff_map.get(&e_effect.id).cloned(),
            is_assist: e_effect.is_assistance,
            is_offense: e_effect.is_offensive,
            discharge_attr_id: e_effect.discharge_attr_id.map(AAttrId::Eve),
            duration_attr_id: e_effect.duration_attr_id.map(AAttrId::Eve),
            range_attr_id: e_effect.range_attr_id.map(AAttrId::Eve),
            falloff_attr_id: e_effect.falloff_attr_id.map(AAttrId::Eve),
            track_attr_id: e_effect.tracking_attr_id.map(AAttrId::Eve),
            chance_attr_id: e_effect.usage_chance_attr_id.map(AAttrId::Eve),
            resist_attr_id: e_effect.resist_attr_id.map(AAttrId::Eve),
            ..
        };
        for e_modifier in e_effect.mods.iter() {
            // Process effect stoppers first
            match extract_stopper(e_modifier) {
                Ok(Some(effect_id)) => {
                    if !a_effect.stopped_effect_ids.contains(&AEffectId::Dogma(effect_id)) {
                        a_effect.stopped_effect_ids.push(AEffectId::Dogma(effect_id))
                    };
                    continue;
                }
                Err(e) => {
                    let msg = format!("failed to build stopper for {a_effect}: {e}");
                    tracing::warn!("{msg}");
                    continue;
                }
                _ => (),
            }
            // Process regular attribute modifiers
            let a_mod_res = match e_modifier.func.as_str() {
                "ItemModifier" => conv_item_mod(e_modifier, &a_effect),
                "LocationModifier" => conv_loc_mod(e_modifier, &a_effect),
                "LocationGroupModifier" => conv_locgrp_mod(e_modifier, &a_effect),
                "LocationRequiredSkillModifier" => conv_locsrq_mod(e_modifier, &a_effect),
                "OwnerRequiredSkillModifier" => conv_ownsrq_mod(e_modifier, &a_effect),
                _ => Err(StrMsgError {
                    msg: format!("unknown function \"{}\"", e_modifier.func),
                }),
            };
            match a_mod_res {
                Ok(a_mod) => a_effect.modifiers.push(a_mod),
                Err(e) => {
                    let msg = format!("failed to build modifier for {a_effect}: {e}");
                    tracing::warn!("{msg}");
                    continue;
                }
            }
        }
        a_effects.insert(a_effect.id, a_effect);
    }
    // Transfer some data from abilities onto effects
    let hisec_ban_map = extract_ability_map(e_data, EFighterAbil::get_disallow_hisec);
    let lowsec_ban_map = extract_ability_map(e_data, EFighterAbil::get_disallow_lowsec);
    for a_effect in a_effects.values_mut() {
        // Hisec flag
        match hisec_ban_map.get(&a_effect.id) {
            None => (),
            Some(flags) => match flags.len() {
                1 => {
                    a_effect.banned_in_hisec = *flags.iter().next().unwrap();
                }
                _ => {
                    let msg = format!(
                        "{} has {} distinct \"disallow in hisec\" values mapped from fighter abilities",
                        a_effect,
                        flags.len()
                    );
                    tracing::warn!("{msg}");
                }
            },
        }
        // Lowsec flag
        match lowsec_ban_map.get(&a_effect.id) {
            None => (),
            Some(flags) => match flags.len() {
                1 => {
                    a_effect.banned_in_lowsec = *flags.iter().next().unwrap();
                }
                _ => {
                    let msg = format!(
                        "{} has {} distinct \"disallow in lowsec\" values mapped from fighter abilities",
                        a_effect,
                        flags.len()
                    );
                    tracing::warn!("{msg}");
                }
            },
        }
    }
    a_effects
}

fn extract_stopper(e_modifier: &EEffectMod) -> Result<Option<EEffectId>, StrMsgError> {
    match e_modifier.func.as_str() {
        "EffectStopper" => {
            let domain = get_arg_str(&e_modifier.args, "domain")?;
            if domain.ne("target") {
                return Err(StrMsgError {
                    msg: format!("unexpected domain \"{domain}\""),
                });
            }
            Ok(Some(get_arg_int(&e_modifier.args, "effectID")?))
        }
        _ => Ok(None),
    }
}

fn conv_item_mod(e_modifier: &EEffectMod, a_effect: &AEffect) -> Result<AEffectModifier, StrMsgError> {
    Ok(AEffectModifier {
        affector_attr_id: get_mod_src_attr_id(e_modifier)?,
        op: get_mod_operation(e_modifier)?,
        affectee_filter: AEffectAffecteeFilter::Direct(get_mod_location(e_modifier, a_effect)?),
        affectee_attr_id: get_mod_affectee_attr_id(e_modifier)?,
    })
}

fn conv_loc_mod(e_modifier: &EEffectMod, a_effect: &AEffect) -> Result<AEffectModifier, StrMsgError> {
    Ok(AEffectModifier {
        affector_attr_id: get_mod_src_attr_id(e_modifier)?,
        op: get_mod_operation(e_modifier)?,
        affectee_filter: AEffectAffecteeFilter::Loc(get_mod_location(e_modifier, a_effect)?),
        affectee_attr_id: get_mod_affectee_attr_id(e_modifier)?,
    })
}

fn conv_locgrp_mod(e_modifier: &EEffectMod, a_effect: &AEffect) -> Result<AEffectModifier, StrMsgError> {
    Ok(AEffectModifier {
        affector_attr_id: get_mod_src_attr_id(e_modifier)?,
        op: get_mod_operation(e_modifier)?,
        affectee_filter: AEffectAffecteeFilter::LocGrp(
            get_mod_location(e_modifier, a_effect)?,
            get_mod_grp_id(e_modifier)?,
        ),
        affectee_attr_id: get_mod_affectee_attr_id(e_modifier)?,
    })
}

fn conv_locsrq_mod(e_modifier: &EEffectMod, a_effect: &AEffect) -> Result<AEffectModifier, StrMsgError> {
    Ok(AEffectModifier {
        affector_attr_id: get_mod_src_attr_id(e_modifier)?,
        op: get_mod_operation(e_modifier)?,
        affectee_filter: AEffectAffecteeFilter::LocSrq(
            get_mod_location(e_modifier, a_effect)?,
            AModifierSrq::TypeId(get_mod_skill_id(e_modifier)?),
        ),
        affectee_attr_id: get_mod_affectee_attr_id(e_modifier)?,
    })
}

fn conv_ownsrq_mod(e_modifier: &EEffectMod, a_effect: &AEffect) -> Result<AEffectModifier, StrMsgError> {
    if !matches!(
        get_mod_location(e_modifier, a_effect)?,
        AEffectLocation::Char | AEffectLocation::Target
    ) {
        return Err(StrMsgError {
            msg: format!(
                "unexpected domain \"{}\" for owner-filtered modification",
                get_arg_str(&e_modifier.args, "domain")?
            ),
        });
    }
    Ok(AEffectModifier {
        affector_attr_id: get_mod_src_attr_id(e_modifier)?,
        op: get_mod_operation(e_modifier)?,
        affectee_filter: AEffectAffecteeFilter::OwnSrq(AModifierSrq::TypeId(get_mod_skill_id(e_modifier)?)),
        affectee_attr_id: get_mod_affectee_attr_id(e_modifier)?,
    })
}

fn get_mod_src_attr_id(e_modifier: &EEffectMod) -> Result<AAttrId, StrMsgError> {
    get_arg_int(&e_modifier.args, "modifyingAttributeID").map(AAttrId::Eve)
}

fn get_mod_affectee_attr_id(e_modifier: &EEffectMod) -> Result<AAttrId, StrMsgError> {
    get_arg_int(&e_modifier.args, "modifiedAttributeID").map(AAttrId::Eve)
}

fn get_mod_location(e_modifier: &EEffectMod, a_effect: &AEffect) -> Result<AEffectLocation, StrMsgError> {
    let domain = get_arg_str(&e_modifier.args, "domain")?;
    match domain.as_str() {
        "itemID" => Ok(AEffectLocation::Item),
        "charID" => Ok(AEffectLocation::Char),
        "shipID" => Ok(AEffectLocation::Ship),
        "structureID" => Ok(AEffectLocation::Structure),
        "targetID" => match a_effect.category {
            ac::effcats::TARGET => Ok(AEffectLocation::Target),
            _ => Err(StrMsgError {
                msg: format!("modifier uses {domain} domain on untargeted effect"),
            }),
        },
        "otherID" => Ok(AEffectLocation::Other),
        _ => Err(StrMsgError {
            msg: format!("unknown domain {domain}"),
        }),
    }
}

fn get_mod_operation(e_modifier: &EEffectMod) -> Result<AOp, StrMsgError> {
    let op = get_arg_int(&e_modifier.args, "operation")?;
    match op {
        -1 => Ok(AOp::PreAssign),
        0 => Ok(AOp::PreMul),
        1 => Ok(AOp::PreDiv),
        2 => Ok(AOp::Add),
        3 => Ok(AOp::Sub),
        4 => Ok(AOp::PostMul),
        5 => Ok(AOp::PostDiv),
        6 => Ok(AOp::PostPerc),
        7 => Ok(AOp::PostAssign),
        8 => Ok(AOp::PostPercImmune),
        _ => Err(StrMsgError {
            msg: format!("unknown operation {op}"),
        }),
    }
}

fn get_mod_grp_id(e_modifier: &EEffectMod) -> Result<EItemGrpId, StrMsgError> {
    get_arg_int(&e_modifier.args, "groupID")
}

fn get_mod_skill_id(e_modifier: &EEffectMod) -> Result<EItemId, StrMsgError> {
    get_arg_int(&e_modifier.args, "skillTypeID")
}

fn get_arg_int(args: &RMap<String, EPrimitive>, name: &str) -> Result<i32, StrMsgError> {
    let primitive = args.get(name).ok_or(StrMsgError {
        msg: format!("no \"{name}\" in args"),
    })?;
    match primitive {
        EPrimitive::Int(i) => Ok(*i),
        _ => Err(StrMsgError {
            msg: format!("expected int in \"{name}\" value"),
        }),
    }
}

fn get_arg_str(args: &RMap<String, EPrimitive>, name: &str) -> Result<String, StrMsgError> {
    let primitive = args.get(name).ok_or(StrMsgError {
        msg: format!("no \"{name}\" in args"),
    })?;
    match primitive {
        EPrimitive::String(s) => Ok(s.into()),
        _ => Err(StrMsgError {
            msg: format!("expected string in \"{name}\" value"),
        }),
    }
}

fn extract_ability_map<F, T>(e_data: &EData, getter: F) -> RMap<AEffectId, RSet<T>>
where
    F: Fn(&EFighterAbil) -> T,
    T: Eq + Hash,
{
    let mut map = RMap::new();
    for e_abil in e_data.abils.data.iter() {
        match get_abil_effect(e_abil.id) {
            None => continue,
            Some(effect_id) => map
                .entry(AEffectId::Dogma(effect_id))
                .or_insert_with(RSet::new)
                .insert(getter(e_abil)),
        };
    }
    map
}
