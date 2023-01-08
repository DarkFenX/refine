use std::collections::HashMap;

use crate::{
    consts::{effcats, ModAfeeFilter, ModAggrMode, ModBuildStatus, ModDomain, ModOp, State, TgtMode},
    ct,
    defines::ReeInt,
    dh,
    util::{Error, Named, Result},
};

use super::Data;

pub(super) fn conv_effects(data: &Data, warns: &mut Vec<String>) -> Vec<ct::Effect> {
    let mut effects = Vec::new();
    for effect_data in data.effects.iter() {
        let (state, tgt_mode) = match effect_data.category_id {
            effcats::PASSIVE => (State::Offline, TgtMode::None),
            effcats::ACTIVE => (State::Active, TgtMode::None),
            effcats::TARGET => (State::Active, TgtMode::Item),
            effcats::ONLINE => (State::Online, TgtMode::None),
            effcats::OVERLOAD => (State::Overload, TgtMode::None),
            effcats::SYSTEM => (State::Offline, TgtMode::None),
            _ => {
                let msg = format!(
                    "{} {} uses unknown effect category {}",
                    dh::Effect::get_name(),
                    effect_data.id,
                    effect_data.category_id
                );
                log::warn!("{}", &msg);
                warns.push(msg);
                continue;
            }
        };
        let mut effect = ct::Effect::new(
            effect_data.id,
            state,
            tgt_mode,
            effect_data.is_assistance,
            effect_data.is_offensive,
            None,
            None,
            effect_data.discharge_attr_id,
            effect_data.duration_attr_id,
            effect_data.range_attr_id,
            effect_data.falloff_attr_id,
            effect_data.tracking_attr_id,
            effect_data.usage_chance_attr_id,
            effect_data.resist_attr_id,
            ModBuildStatus::Unbuilt,
            Vec::new(),
            Vec::new(),
        );
        let mut mod_errs = 0;
        for modifier_data in effect_data.mods.iter() {
            // Process effect stoppers first
            match extract_stopper(modifier_data) {
                Ok(Some(eid)) => {
                    if !effect.stop_ids.contains(&eid) {
                        effect.stop_ids.push(eid)
                    };
                    continue;
                }
                Err(e) => {
                    let msg = format!(
                        "failed to build stopper for {} {}: {}",
                        ct::Effect::get_name(),
                        effect.id,
                        e.msg
                    );
                    log::warn!("{}", &msg);
                    warns.push(msg);
                    mod_errs += 1;
                    continue;
                }
                _ => (),
            }
            // Process regular attribute modifiers
            let mod_res = match modifier_data.func.as_str() {
                "ItemModifier" => conv_item_mod(modifier_data, &effect),
                "LocationModifier" => conv_loc_mod(modifier_data, &effect),
                "LocationGroupModifier" => conv_locgrp_mod(modifier_data, &effect),
                "LocationRequiredSkillModifier" => conv_locsrq_mod(modifier_data, &effect),
                "OwnerRequiredSkillModifier" => conv_ownsrq_mod(modifier_data, &effect),
                _ => Err(Error::new(format!("unknown function \"{}\"", modifier_data.func))),
            };
            match mod_res {
                Ok(m) => effect.mods.push(m),
                Err(e) => {
                    let msg = format!(
                        "failed to build modifier for {} {}: {}",
                        ct::Effect::get_name(),
                        effect.id,
                        e.msg
                    );
                    log::warn!("{}", &msg);
                    warns.push(msg);
                    mod_errs += 1;
                    continue;
                }
            }
        }
        match mod_errs {
            0 => effect.mod_build_status = ModBuildStatus::Success,
            _ if !effect.mods.is_empty() || !effect.stop_ids.is_empty() => {
                effect.mod_build_status = ModBuildStatus::SuccessPartial
            }
            _ => effect.mod_build_status = ModBuildStatus::Error,
        }
        effects.push(effect);
    }
    effects
}

fn extract_stopper(modifier_data: &dh::EffectMod) -> Result<Option<ReeInt>> {
    match modifier_data.func.as_str() {
        "EffectStopper" => {
            let domain = get_arg_str(&modifier_data.args, "domain")?;
            if domain.ne("target") {
                return Err(Error::new(format!("unexpected domain \"{}\"", domain)));
            }
            Ok(Some(get_arg_int(&modifier_data.args, "effectID")?))
        }
        _ => Ok(None),
    }
}

fn conv_item_mod(modifier_data: &dh::EffectMod, effect: &ct::Effect) -> Result<ct::AttrMod> {
    Ok(ct::AttrMod::new(
        get_mod_affector_attr_id(modifier_data)?,
        ModAggrMode::Stack,
        get_mod_operation(modifier_data)?,
        ModAfeeFilter::Direct(get_mod_domain(modifier_data, effect)?),
        get_mod_affectee_attr_id(modifier_data)?,
    ))
}

fn conv_loc_mod(modifier_data: &dh::EffectMod, effect: &ct::Effect) -> Result<ct::AttrMod> {
    Ok(ct::AttrMod::new(
        get_mod_affector_attr_id(modifier_data)?,
        ModAggrMode::Stack,
        get_mod_operation(modifier_data)?,
        ModAfeeFilter::Loc(get_mod_domain(modifier_data, effect)?),
        get_mod_affectee_attr_id(modifier_data)?,
    ))
}

fn conv_locgrp_mod(modifier_data: &dh::EffectMod, effect: &ct::Effect) -> Result<ct::AttrMod> {
    Ok(ct::AttrMod::new(
        get_mod_affector_attr_id(modifier_data)?,
        ModAggrMode::Stack,
        get_mod_operation(modifier_data)?,
        ModAfeeFilter::LocGrp(get_mod_domain(modifier_data, effect)?, get_mod_grp_id(modifier_data)?),
        get_mod_affectee_attr_id(modifier_data)?,
    ))
}

fn conv_locsrq_mod(modifier_data: &dh::EffectMod, effect: &ct::Effect) -> Result<ct::AttrMod> {
    Ok(ct::AttrMod::new(
        get_mod_affector_attr_id(modifier_data)?,
        ModAggrMode::Stack,
        get_mod_operation(modifier_data)?,
        ModAfeeFilter::LocSrq(get_mod_domain(modifier_data, effect)?, get_mod_skill_id(modifier_data)?),
        get_mod_affectee_attr_id(modifier_data)?,
    ))
}

fn conv_ownsrq_mod(modifier_data: &dh::EffectMod, effect: &ct::Effect) -> Result<ct::AttrMod> {
    Ok(ct::AttrMod::new(
        get_mod_affector_attr_id(modifier_data)?,
        ModAggrMode::Stack,
        get_mod_operation(modifier_data)?,
        ModAfeeFilter::OwnSrq(get_mod_domain(modifier_data, effect)?, get_mod_skill_id(modifier_data)?),
        get_mod_affectee_attr_id(modifier_data)?,
    ))
}

fn get_mod_affector_attr_id(modifier_data: &dh::EffectMod) -> Result<ReeInt> {
    get_arg_int(&modifier_data.args, "modifyingAttributeID")
}

fn get_mod_affectee_attr_id(modifier_data: &dh::EffectMod) -> Result<ReeInt> {
    get_arg_int(&modifier_data.args, "modifiedAttributeID")
}

fn get_mod_domain(modifier_data: &dh::EffectMod, effect: &ct::Effect) -> Result<ModDomain> {
    let domain = get_arg_str(&modifier_data.args, "domain")?;
    match domain.as_str() {
        "itemID" => Ok(ModDomain::Item),
        "charID" => Ok(ModDomain::Char),
        "shipID" => Ok(ModDomain::Ship),
        "structureID" => Ok(ModDomain::Structure),
        "targetID" => match effect.tgt_mode {
            TgtMode::Item => Ok(ModDomain::Item),
            _ => Err(Error::new(format!(
                "modifier uses {} domain on untargeted effect",
                domain
            ))),
        },
        "otherID" => Ok(ModDomain::Other),
        _ => Err(Error::new(format!("unknown domain {}", domain))),
    }
}

fn get_mod_operation(modifier_data: &dh::EffectMod) -> Result<ModOp> {
    let op = get_arg_int(&modifier_data.args, "operation")?;
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
        _ => Err(Error::new(format!("unknown operation {}", op))),
    }
}

fn get_mod_grp_id(modifier_data: &dh::EffectMod) -> Result<ReeInt> {
    get_arg_int(&modifier_data.args, "groupID")
}

fn get_mod_skill_id(modifier_data: &dh::EffectMod) -> Result<ReeInt> {
    get_arg_int(&modifier_data.args, "skillTypeID")
}

fn get_arg_int(args: &HashMap<String, dh::Primitive>, name: &str) -> Result<ReeInt> {
    let primitive = args.get(name).ok_or(Error::new(format!("no \"{}\" in args", name)))?;
    match primitive {
        dh::Primitive::Int(i) => Ok(*i),
        _ => Err(Error::new(format!("expected int in \"{}\" value", name))),
    }
}

fn get_arg_str(args: &HashMap<String, dh::Primitive>, name: &str) -> Result<String> {
    let primitive = args.get(name).ok_or(Error::new(format!("no \"{}\" in args", name)))?;
    match primitive {
        dh::Primitive::String(s) => Ok(s.into()),
        _ => Err(Error::new(format!("expected string in \"{}\" value", name))),
    }
}
