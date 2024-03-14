use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::{
    ad,
    adg::GData,
    defs::{EAttrId, EEffectId, EItemGrpId, EItemId},
    ec, ed,
    shr::{ModOp, State},
    util::{IntError, IntResult},
};

impl ed::EFighterAbil {
    fn get_target_mode(&self) -> String {
        self.target_mode.clone()
    }
    fn get_disallow_hisec(&self) -> bool {
        self.disallow_hisec
    }
    fn get_disallow_lowsec(&self) -> bool {
        self.disallow_lowsec
    }
}

pub(in crate::adg::conv) fn conv_effects(g_data: &GData) -> Vec<ad::AEffect> {
    let mut a_effects = Vec::new();
    for e_effect in g_data.effects.iter() {
        let (state, tgt_mode, is_system_wide) = match e_effect.category_id {
            ec::effcats::PASSIVE => (State::Offline, None, false),
            ec::effcats::ACTIVE => (State::Active, None, false),
            ec::effcats::TARGET => (State::Active, Some(ad::ATgtMode::Item), false),
            ec::effcats::ONLINE => (State::Online, None, false),
            ec::effcats::OVERLOAD => (State::Overload, None, false),
            ec::effcats::SYSTEM => (State::Offline, None, true),
            _ => {
                let msg = format!("{} uses unknown effect category {}", e_effect, e_effect.category_id);
                tracing::warn!("{msg}");
                continue;
            }
        };
        let buff_info = if ec::effects::FLEET_BUFF_EFFECT_IDS.contains(&e_effect.id) {
            Some(ad::AEffectBuffInfo::new(
                ad::AEffectBuffDataSrc::DefaultAttrs,
                ad::AEffectBuffScope::FleetShips,
            ))
        } else if ec::effects::EVERYTHING_BUFF_EFFECT_IDS.contains(&e_effect.id) {
            Some(ad::AEffectBuffInfo::new(
                ad::AEffectBuffDataSrc::DefaultAttrs,
                ad::AEffectBuffScope::Everything,
            ))
        } else {
            None
        };
        let mut a_effect = ad::AEffect::new(
            e_effect.id,
            state,
            tgt_mode,
            is_system_wide,
            e_effect.is_assistance,
            e_effect.is_offensive,
            None,
            None,
            e_effect.discharge_attr_id,
            e_effect.duration_attr_id,
            e_effect.range_attr_id,
            e_effect.falloff_attr_id,
            e_effect.tracking_attr_id,
            e_effect.usage_chance_attr_id,
            e_effect.resist_attr_id,
            ad::AModBuildStatus::Unbuilt,
            Vec::new(),
            Vec::new(),
            buff_info,
        );
        let mut mod_errs = 0;
        for e_modifier in e_effect.mods.iter() {
            // Process effect stoppers first
            match extract_stopper(e_modifier) {
                Ok(Some(effect_id)) => {
                    if !a_effect.stop_ids.contains(&effect_id) {
                        a_effect.stop_ids.push(effect_id)
                    };
                    continue;
                }
                Err(e) => {
                    let msg = format!("failed to build stopper for {}: {}", a_effect, e.msg);
                    tracing::warn!("{msg}");
                    mod_errs += 1;
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
                _ => Err(IntError::new(format!("unknown function \"{}\"", e_modifier.func))),
            };
            match a_mod_res {
                Ok(a_mod) => a_effect.mods.push(a_mod),
                Err(e) => {
                    let msg = format!("failed to build modifier for {}: {}", a_effect, e.msg);
                    tracing::warn!("{msg}");
                    mod_errs += 1;
                    continue;
                }
            }
        }
        match mod_errs {
            0 => a_effect.mod_build_status = ad::AModBuildStatus::Success,
            _ if !a_effect.mods.is_empty() || !a_effect.stop_ids.is_empty() => {
                a_effect.mod_build_status = ad::AModBuildStatus::SuccessPartial(mod_errs)
            }
            _ => a_effect.mod_build_status = ad::AModBuildStatus::Error(mod_errs),
        }
        a_effects.push(a_effect);
    }
    // Transfer some data from abilities onto effects
    let hisec_ban_map = extract_ability_map(g_data, ed::EFighterAbil::get_disallow_hisec);
    let lowsec_ban_map = extract_ability_map(g_data, ed::EFighterAbil::get_disallow_lowsec);
    let tgt_mode_map = extract_ability_map(g_data, ed::EFighterAbil::get_target_mode);
    for a_effect in a_effects.iter_mut() {
        // Hisec flag
        match hisec_ban_map.get(&a_effect.id) {
            None => (),
            Some(flags) => match flags.len() {
                1 => {
                    a_effect.hisec = Some(!*flags.iter().next().unwrap());
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
                    a_effect.lowsec = Some(!*flags.iter().next().unwrap());
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
        // Target mode
        match tgt_mode_map.get(&a_effect.id) {
            None => (),
            Some(modes) => match modes.len() {
                1 => match get_abil_tgt_mode(modes.iter().next().unwrap()) {
                    Ok(mode) => a_effect.tgt_mode = mode,
                    Err(e) => {
                        let msg = format!("failed to update target mode for {}: {}", a_effect, e.msg);
                        tracing::warn!("{msg}");
                    }
                },
                _ => {
                    let msg = format!(
                        "{} has {} distinct \"target mode\" values mapped from fighter abilities",
                        a_effect,
                        modes.len()
                    );
                    tracing::warn!("{msg}");
                }
            },
        }
    }
    a_effects
}

fn extract_stopper(e_modifier: &ed::EEffectMod) -> IntResult<Option<EEffectId>> {
    match e_modifier.func.as_str() {
        "EffectStopper" => {
            let domain = get_arg_str(&e_modifier.args, "domain")?;
            if domain.ne("target") {
                return Err(IntError::new(format!("unexpected domain \"{domain}\"")));
            }
            Ok(Some(get_arg_int(&e_modifier.args, "effectID")?))
        }
        _ => Ok(None),
    }
}

fn conv_item_mod(e_modifier: &ed::EEffectMod, a_effect: &ad::AEffect) -> IntResult<ad::AEffectAttrMod> {
    Ok(ad::AEffectAttrMod::new(
        get_mod_src_attr_id(e_modifier)?,
        get_mod_operation(e_modifier)?,
        ad::AModTgtFilter::Direct(get_mod_domain(e_modifier, a_effect)?),
        get_mod_tgt_attr_id(e_modifier)?,
    ))
}

fn conv_loc_mod(e_modifier: &ed::EEffectMod, a_effect: &ad::AEffect) -> IntResult<ad::AEffectAttrMod> {
    Ok(ad::AEffectAttrMod::new(
        get_mod_src_attr_id(e_modifier)?,
        get_mod_operation(e_modifier)?,
        ad::AModTgtFilter::Loc(get_mod_domain(e_modifier, a_effect)?),
        get_mod_tgt_attr_id(e_modifier)?,
    ))
}

fn conv_locgrp_mod(e_modifier: &ed::EEffectMod, a_effect: &ad::AEffect) -> IntResult<ad::AEffectAttrMod> {
    Ok(ad::AEffectAttrMod::new(
        get_mod_src_attr_id(e_modifier)?,
        get_mod_operation(e_modifier)?,
        ad::AModTgtFilter::LocGrp(get_mod_domain(e_modifier, a_effect)?, get_mod_grp_id(e_modifier)?),
        get_mod_tgt_attr_id(e_modifier)?,
    ))
}

fn conv_locsrq_mod(e_modifier: &ed::EEffectMod, a_effect: &ad::AEffect) -> IntResult<ad::AEffectAttrMod> {
    Ok(ad::AEffectAttrMod::new(
        get_mod_src_attr_id(e_modifier)?,
        get_mod_operation(e_modifier)?,
        ad::AModTgtFilter::LocSrq(
            get_mod_domain(e_modifier, a_effect)?,
            ad::AModSrq::ItemId(get_mod_skill_id(e_modifier)?),
        ),
        get_mod_tgt_attr_id(e_modifier)?,
    ))
}

fn conv_ownsrq_mod(e_modifier: &ed::EEffectMod, a_effect: &ad::AEffect) -> IntResult<ad::AEffectAttrMod> {
    if get_mod_domain(e_modifier, a_effect)? != ad::AModDomain::Char {
        return Err(IntError::new(format!(
            "unexpected domain \"{}\" for owner-filtered modification",
            get_arg_str(&e_modifier.args, "domain")?
        )));
    }
    Ok(ad::AEffectAttrMod::new(
        get_mod_src_attr_id(e_modifier)?,
        get_mod_operation(e_modifier)?,
        ad::AModTgtFilter::OwnSrq(ad::AModSrq::ItemId(get_mod_skill_id(e_modifier)?)),
        get_mod_tgt_attr_id(e_modifier)?,
    ))
}

fn get_mod_src_attr_id(e_modifier: &ed::EEffectMod) -> IntResult<EAttrId> {
    get_arg_int(&e_modifier.args, "modifyingAttributeID")
}

fn get_mod_tgt_attr_id(e_modifier: &ed::EEffectMod) -> IntResult<EAttrId> {
    get_arg_int(&e_modifier.args, "modifiedAttributeID")
}

fn get_mod_domain(e_modifier: &ed::EEffectMod, a_effect: &ad::AEffect) -> IntResult<ad::AModDomain> {
    let domain = get_arg_str(&e_modifier.args, "domain")?;
    match domain.as_str() {
        "itemID" => Ok(ad::AModDomain::Item),
        "charID" => Ok(ad::AModDomain::Char),
        "shipID" => Ok(ad::AModDomain::Ship),
        "structureID" => Ok(ad::AModDomain::Structure),
        "targetID" => match a_effect.tgt_mode {
            Some(ad::ATgtMode::Item) => Ok(ad::AModDomain::Item),
            _ => Err(IntError::new(format!(
                "modifier uses {} domain on untargeted effect",
                domain
            ))),
        },
        "otherID" => Ok(ad::AModDomain::Other),
        _ => Err(IntError::new(format!("unknown domain {domain}"))),
    }
}

fn get_mod_operation(e_modifier: &ed::EEffectMod) -> IntResult<ModOp> {
    let op = get_arg_int(&e_modifier.args, "operation")?;
    match op {
        -1 => Ok(ModOp::PreAssign),
        0 => Ok(ModOp::PreMul),
        1 => Ok(ModOp::PreDiv),
        2 => Ok(ModOp::Add),
        3 => Ok(ModOp::Sub),
        4 => Ok(ModOp::PostMul),
        5 => Ok(ModOp::PostDiv),
        6 => Ok(ModOp::PostPerc),
        7 => Ok(ModOp::PostAssign),
        _ => Err(IntError::new(format!("unknown operation {op}"))),
    }
}

fn get_mod_grp_id(e_modifier: &ed::EEffectMod) -> IntResult<EItemGrpId> {
    get_arg_int(&e_modifier.args, "groupID")
}

fn get_mod_skill_id(e_modifier: &ed::EEffectMod) -> IntResult<EItemId> {
    get_arg_int(&e_modifier.args, "skillTypeID")
}

fn get_arg_int(args: &HashMap<String, ed::EPrimitive>, name: &str) -> IntResult<i32> {
    let primitive = args.get(name).ok_or(IntError::new(format!("no \"{name}\" in args")))?;
    match primitive {
        ed::EPrimitive::Int(i) => Ok(*i),
        _ => Err(IntError::new(format!("expected int in \"{name}\" value"))),
    }
}

fn get_arg_str(args: &HashMap<String, ed::EPrimitive>, name: &str) -> IntResult<String> {
    let primitive = args.get(name).ok_or(IntError::new(format!("no \"{name}\" in args")))?;
    match primitive {
        ed::EPrimitive::String(s) => Ok(s.into()),
        _ => Err(IntError::new(format!("expected string in \"{name}\" value"))),
    }
}

fn extract_ability_map<F, T>(g_data: &GData, getter: F) -> HashMap<EEffectId, HashSet<T>>
where
    F: Fn(&ed::EFighterAbil) -> T,
    T: Eq + Hash,
{
    let mut map = HashMap::new();
    for e_abil in g_data.abils.iter() {
        match ec::abils::get_abil_effect(e_abil.id) {
            None => continue,
            Some(effect_id) => map
                .entry(effect_id)
                .or_insert_with(|| HashSet::new())
                .insert(getter(&e_abil)),
        };
    }
    map
}

fn get_abil_tgt_mode(tgt_mode: &str) -> IntResult<Option<ad::ATgtMode>> {
    match tgt_mode {
        "untargeted" => Ok(None),
        "itemTargeted" => Ok(Some(ad::ATgtMode::Item)),
        "pointTargeted" => Ok(Some(ad::ATgtMode::Point)),
        _ => Err(IntError::new(format!("unknown ability target mode \"{tgt_mode}\""))),
    }
}
