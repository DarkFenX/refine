use std::collections::HashMap;

use crate::{
    consts::{effcats, ModAfeeFilter, ModAggrMode, ModDomain, ModOp, State, TgtMode},
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
            effcats::TARGET => (State::Active, TgtMode::Point),
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
            Vec::new(),
            Vec::new(),
        );
        for modifier_data in effect_data.mods.iter() {
            match modifier_data.func.as_str() {
                "ItemModifier" => match conv_item_modifier(modifier_data) {
                    Ok(modifier) => effect.mods.push(modifier),
                    Err(e) => {
                        let msg = format!(
                            "failed to build modifier for {} {}: {}",
                            ct::Effect::get_name(),
                            effect.id,
                            e.msg
                        );
                        log::warn!("{}", &msg);
                        warns.push(msg);
                        continue;
                    }
                },
                _ => {
                    let msg = format!(
                        "failed to build modifier for {} {}: unknown function \"{}\"",
                        ct::Effect::get_name(),
                        effect.id,
                        modifier_data.func
                    );
                    log::warn!("{}", &msg);
                    warns.push(msg);
                    continue;
                }
            };
        }
        effects.push(effect);
    }
    effects
}

fn conv_item_modifier(modifier_data: &dh::EffectMod) -> Result<ct::AttrMod> {
    Ok(ct::AttrMod::new(
        get_mod_affector_attr_id(modifier_data)?,
        ModAggrMode::Stack,
        get_mod_operation(modifier_data)?,
        ModAfeeFilter::Direct(get_mod_domain(modifier_data)?),
        get_mod_affectee_attr_id(modifier_data)?,
    ))
}

fn get_mod_affector_attr_id(modifier_data: &dh::EffectMod) -> Result<ReeInt> {
    get_arg_int(&modifier_data.args, "modifyingAttributeID")
}

fn get_mod_affectee_attr_id(modifier_data: &dh::EffectMod) -> Result<ReeInt> {
    get_arg_int(&modifier_data.args, "modifiedAttributeID")
}

fn get_mod_domain(modifier_data: &dh::EffectMod) -> Result<ModDomain> {
    let domain = get_arg_str(&modifier_data.args, "domain")?;
    match domain.as_str() {
        "itemID" => Ok(ModDomain::Item),
        "charID" => Ok(ModDomain::Char),
        "shipID" => Ok(ModDomain::Ship),
        "targetID" => Ok(ModDomain::Item),
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
