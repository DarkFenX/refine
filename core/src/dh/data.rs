use std::collections::HashMap;

use crate::defines::{ReeFloat, ReeInt};

/// A type which is used by [`DataHandler`](super::DataHandler) to pass data and accumulated errors to the caller.
#[derive(Debug)]
pub struct Container<T> {
    pub data: Vec<T>,
    pub errors: Vec<String>,
}
impl<T> Container<T> {
    /// Make a new Container out of passed data.
    pub fn new(data: Vec<T>, errors: Vec<String>) -> Container<T> {
        Container { data, errors }
    }
}

/// An auxiliary entity for "primitive" data.
#[derive(Debug)]
pub enum Primitive {
    Null,
    Bool(bool),
    Int(ReeInt),
    Float(ReeFloat),
    String(String),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inventory
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Item type data.
#[derive(Debug)]
pub struct Item {
    pub id: ReeInt,
    pub group_id: ReeInt,
}
impl Item {
    /// Make a new item type out of passed data.
    pub fn new(id: ReeInt, group_id: ReeInt) -> Item {
        Item { id, group_id }
    }
}

/// Item group data.
#[derive(Debug)]
pub struct ItemGroup {
    pub id: ReeInt,
    pub category_id: ReeInt,
}
impl ItemGroup {
    /// Make a new item group out of passed data.
    pub fn new(id: ReeInt, category_id: ReeInt) -> ItemGroup {
        ItemGroup { id, category_id }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Dogma
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Dogma attribute data.
#[derive(Debug)]
pub struct Attr {
    pub id: ReeInt,
    pub stackable: bool,
    pub high_is_good: bool,
    pub default_value: ReeFloat,
}
impl Attr {
    /// Make a new dogma attribute out of passed data.
    pub fn new(id: ReeInt, stackable: bool, high_is_good: bool, default_value: ReeFloat) -> Attr {
        Attr {
            id,
            stackable,
            high_is_good,
            default_value,
        }
    }
}

/// An item type - dogma attribute relation.
#[derive(Debug)]
pub struct ItemAttr {
    pub item_id: ReeInt,
    pub attr_id: ReeInt,
    pub value: ReeFloat,
}
impl ItemAttr {
    /// Make a new item-attribute relation out of passed data.
    pub fn new(item_id: ReeInt, attr_id: ReeInt, value: ReeFloat) -> ItemAttr {
        ItemAttr {
            item_id,
            attr_id,
            value,
        }
    }
}

/// Dogma effect data.
#[derive(Debug)]
pub struct Effect {
    pub id: ReeInt,
    pub category_id: ReeInt,
    pub is_assistance: bool,
    pub is_offensive: bool,
    pub is_warp_safe: bool,
    pub discharge_attr_id: Option<ReeInt>,
    pub duration_attr_id: Option<ReeInt>,
    pub range_attr_id: Option<ReeInt>,
    pub falloff_attr_id: Option<ReeInt>,
    pub tracking_attr_id: Option<ReeInt>,
    pub usage_chance_attr_id: Option<ReeInt>,
    pub resist_attr_id: Option<ReeInt>,
    pub mods: Vec<EffectMod>,
}
impl Effect {
    /// Make a new dogma effect out of passed data.
    pub fn new(
        id: ReeInt,
        category_id: ReeInt,
        is_assistance: bool,
        is_offensive: bool,
        is_warp_safe: bool,
        discharge_attr_id: Option<ReeInt>,
        duration_attr_id: Option<ReeInt>,
        range_attr_id: Option<ReeInt>,
        falloff_attr_id: Option<ReeInt>,
        tracking_attr_id: Option<ReeInt>,
        usage_chance_attr_id: Option<ReeInt>,
        resist_attr_id: Option<ReeInt>,
        mods: Vec<EffectMod>,
    ) -> Effect {
        Effect {
            id,
            category_id,
            is_assistance,
            is_offensive,
            is_warp_safe,
            discharge_attr_id,
            duration_attr_id,
            range_attr_id,
            falloff_attr_id,
            tracking_attr_id,
            usage_chance_attr_id,
            resist_attr_id,
            mods,
        }
    }
}
/// Dogma effect modifier data.
#[derive(Debug)]
pub struct EffectMod {
    pub func: String,
    pub args: HashMap<String, Primitive>,
}
impl EffectMod {
    /// Make a new dogma effect modifier out of passed data.
    pub fn new<T: Into<String>>(func: T, args: HashMap<String, Primitive>) -> EffectMod {
        EffectMod {
            func: func.into(),
            args,
        }
    }
}

/// An item type - dogma effect relation.
#[derive(Debug)]
pub struct ItemEffect {
    pub item_id: ReeInt,
    pub effect_id: ReeInt,
    pub is_default: bool,
}
impl ItemEffect {
    /// Make a new item-effect relation out of passed data.
    pub fn new(item_id: ReeInt, effect_id: ReeInt, is_default: bool) -> ItemEffect {
        ItemEffect {
            item_id,
            effect_id,
            is_default,
        }
    }
}

/// Mutaplasmid item type conversion data.
#[derive(Debug)]
pub struct MutaItemConv {
    pub muta_id: ReeInt,
    pub in_item_id: ReeInt,
    pub out_item_id: ReeInt,
}
impl MutaItemConv {
    /// Make a new mutaplasmid item type conversion.
    pub fn new(muta_id: ReeInt, in_item_id: ReeInt, out_item_id: ReeInt) -> MutaItemConv {
        MutaItemConv {
            muta_id,
            in_item_id,
            out_item_id,
        }
    }
}

/// Mutaplasmid attribute modification data.
#[derive(Debug)]
pub struct MutaAttrMod {
    pub muta_id: ReeInt,
    pub attr_id: ReeInt,
    pub min_attr_mult: ReeFloat,
    pub max_attr_mult: ReeFloat,
}
impl MutaAttrMod {
    /// Make a new mutaplasmid attribute conversion.
    pub fn new(muta_id: ReeInt, attr_id: ReeInt, min_attr_mult: ReeFloat, max_attr_mult: ReeFloat) -> MutaAttrMod {
        MutaAttrMod {
            muta_id,
            attr_id,
            min_attr_mult,
            max_attr_mult,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Dogma Buffs
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Dogma buff data.
#[derive(Debug)]
pub struct Buff {
    pub id: ReeInt,
    pub aggregate_mode: String,
    pub operation: String,
    pub item_mods: Vec<BuffIM>,
    pub loc_mods: Vec<BuffLM>,
    pub locgroup_mods: Vec<BuffLGM>,
    pub locsrq_mods: Vec<BuffLRSM>,
}
impl Buff {
    /// Make a new dogma buff out of passed data.
    pub fn new<T: Into<String>, U: Into<String>>(
        id: ReeInt,
        aggregate_mode: T,
        operation: U,
        item_mods: Vec<BuffIM>,
        loc_mods: Vec<BuffLM>,
        locgroup_mods: Vec<BuffLGM>,
        locsrq_mods: Vec<BuffLRSM>,
    ) -> Buff {
        Buff {
            id,
            aggregate_mode: aggregate_mode.into(),
            operation: operation.into(),
            item_mods,
            loc_mods,
            locgroup_mods,
            locsrq_mods,
        }
    }
}
/// Auxiliary data needed to apply a dogma buff modification directly to some item.
#[derive(Debug)]
pub struct BuffIM {
    pub attr_id: ReeInt,
}
impl BuffIM {
    /// Make a new dogma buff auxiliary modifier out of passed data.
    pub fn new(attr_id: ReeInt) -> BuffIM {
        BuffIM { attr_id }
    }
}
/// Auxiliary data needed to apply a dogma buff modification to location-filtered items.
#[derive(Debug)]
pub struct BuffLM {
    pub attr_id: ReeInt,
}
impl BuffLM {
    /// Make a new dogma buff auxiliary modifier out of passed data.
    pub fn new(attr_id: ReeInt) -> BuffLM {
        BuffLM { attr_id }
    }
}
/// Auxiliary data needed to apply a dogma buff modification to location- and group-filtered items.
#[derive(Debug)]
pub struct BuffLGM {
    pub attr_id: ReeInt,
    pub group_id: ReeInt,
}
impl BuffLGM {
    /// Make a new dogma buff auxiliary modifier out of passed data.
    pub fn new(attr_id: ReeInt, group_id: ReeInt) -> BuffLGM {
        BuffLGM { attr_id, group_id }
    }
}
/// Auxiliary data needed to apply a dogma buff modification to location- and skill requirement-filtered items.
#[derive(Debug)]
pub struct BuffLRSM {
    pub attr_id: ReeInt,
    pub skill_id: ReeInt,
}
impl BuffLRSM {
    /// Make a new dogma buff auxiliary modifier out of passed data.
    pub fn new(attr_id: ReeInt, skill_id: ReeInt) -> BuffLRSM {
        BuffLRSM { attr_id, skill_id }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fighter abilities
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Fighter ability data.
#[derive(Debug)]
pub struct FighterAbil {
    pub id: ReeInt,
    pub target_mode: String,
    pub disallow_hisec: bool,
    pub disallow_lowsec: bool,
}
impl FighterAbil {
    /// Make a new fighter ability out of passed data.
    pub fn new<T: Into<String>>(
        id: ReeInt,
        target_mode: T,
        disallow_hisec: bool,
        disallow_lowsec: bool,
    ) -> FighterAbil {
        FighterAbil {
            id,
            target_mode: target_mode.into(),
            disallow_hisec,
            disallow_lowsec,
        }
    }
}

/// An item type - fighter ability relation.
#[derive(Debug)]
pub struct ItemFighterAbil {
    pub item_id: ReeInt,
    pub abil_id: ReeInt,
    pub cooldown: Option<ReeFloat>,
    pub charge_count: Option<ReeInt>,
    pub charge_rearm_time: Option<ReeFloat>,
}
impl ItemFighterAbil {
    /// Makes a new item-ability relation out of passed data.
    pub fn new(
        item_id: ReeInt,
        abil_id: ReeInt,
        cooldown: Option<ReeFloat>,
        charge_count: Option<ReeInt>,
        charge_rearm_time: Option<ReeFloat>,
    ) -> ItemFighterAbil {
        ItemFighterAbil {
            item_id,
            abil_id,
            cooldown,
            charge_count,
            charge_rearm_time,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Item type skill requirement.
#[derive(Debug)]
pub struct ItemSkillReq {
    pub item_id: ReeInt,
    pub skill_id: ReeInt,
    pub level: ReeInt,
}
impl ItemSkillReq {
    /// Make a new item type skill requirement out of passed data.
    pub fn new(item_id: ReeInt, skill_id: ReeInt, level: ReeInt) -> ItemSkillReq {
        ItemSkillReq {
            item_id,
            skill_id,
            level,
        }
    }
}
