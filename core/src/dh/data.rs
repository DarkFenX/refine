use std::collections::HashMap;

use crate::defines::{ReeFloat, ReeInt};

/// Convenience type to pass data and accumulated errors to the caller.
#[derive(Debug)]
pub struct Container<T> {
    /// Vector with actual data.
    pub data: Vec<T>,
    /// Vector with strings which represent non-critical errors during data generation.
    pub errors: Vec<String>,
}
impl<T> Container<T> {
    /// Make a new empty container.
    pub fn new() -> Container<T> {
        Container {
            data: Vec::new(),
            errors: Vec::new(),
        }
    }
    /// Make a new container out of passed data.
    pub fn new_with_data(data: Vec<T>, errors: Vec<String>) -> Container<T> {
        Container { data, errors }
    }
}

/// Auxiliary entity for "primitive" data.
#[derive(Debug)]
pub enum Primitive {
    /// Represents absence of a value.
    Null,
    /// Represents a boolean value.
    Bool(bool),
    /// Represents an integer number value.
    Int(ReeInt),
    /// Represents a float number value.
    Float(ReeFloat),
    /// Represents a string value.
    String(String),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inventory
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Item type data.
#[derive(Debug)]
pub struct Item {
    /// Item type ID.
    pub id: ReeInt,
    /// Refers an item group the item type belongs to.
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
    /// Item group ID.
    pub id: ReeInt,
    /// Refers an item category the item group belongs to.
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
    /// Dogma attribute ID.
    pub id: ReeInt,
    /// Defines if modifications applied to the attribute's values stack with penalty (false) or not
    /// (true).
    pub stackable: bool,
    /// Defines if higher value of the attribute is considered good or not.
    pub high_is_good: bool,
    /// Default value of the attribute, used if not provided by an item type.
    pub default_value: Option<ReeFloat>,
    /// Refers another attribute, whose value limits value of this attribute.
    pub max_attr_id: Option<ReeInt>,
}
impl Attr {
    /// Make a new dogma attribute out of passed data.
    pub fn new(
        id: ReeInt,
        stackable: bool,
        high_is_good: bool,
        default_value: Option<ReeFloat>,
        max_attr_id: Option<ReeInt>,
    ) -> Attr {
        Attr {
            id,
            stackable,
            high_is_good,
            default_value,
            max_attr_id,
        }
    }
}

/// An item type - dogma attribute relation.
#[derive(Debug)]
pub struct ItemAttr {
    /// Refers an item type involved in the relation.
    pub item_id: ReeInt,
    /// Refers a dogma attribute involved in the relation.
    pub attr_id: ReeInt,
    /// Value of the attribute.
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
    /// Dogma effect ID.
    pub id: ReeInt,
    /// Refers an effect category the effect belongs to.
    pub category_id: ReeInt,
    /// Defines if the effect is considered as an assistance.
    pub is_assistance: bool,
    /// Defines if the effect is offensive or not.
    pub is_offensive: bool,
    /// Defines if the effect can be used while in warp.
    pub is_warp_safe: bool,
    /// Refers an attribute value which defines capacitor cost to run the effect.
    pub discharge_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines how long an effect cycle would take in milliseconds.
    pub duration_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines optimal range of the effect in meters.
    pub range_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines falloff range of the effect in meters.
    pub falloff_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines tracking speed of the effect.
    pub tracking_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines chance of the effect to run when its parent item is
    /// fitted.
    pub usage_chance_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines resistance strength to the effect.
    pub resist_attr_id: Option<ReeInt>,
    /// Modifiers of the effect.
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
    /// Function which the effect modifier calls to apply its modification.
    pub func: String,
    /// Arguments to the function call.
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
    /// Refers an item type involved in the relation.
    pub item_id: ReeInt,
    /// Refers a dogma effect involved in the relation.
    pub effect_id: ReeInt,
    /// Defines if the effect is default to the item or not.
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
    /// Mutaplasmid item type ID.
    pub muta_id: ReeInt,
    /// Refers an item type the mutaplasmid can be applied to.
    pub in_item_id: ReeInt,
    /// Refers an item type, which is the outcome of the conversion.
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
    /// Mutaplasmid item type ID.
    pub muta_id: ReeInt,
    /// Refers an attribute being modified by the mutaplasmid.
    pub attr_id: ReeInt,
    /// Lower boundary of the modification range.
    pub min_attr_mult: ReeFloat,
    /// Upper boundary of the modification range.
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
    /// Dogma buff ID.
    pub id: ReeInt,
    /// Defines how multiple buffs of the same type are aggregated.
    pub aggregate_mode: String,
    /// Name of the operation applied to attributes targeted by the buff.
    pub operation: String,
    /// Modifiers which apply some modification to some item directly.
    pub item_mods: Vec<BuffIM>,
    /// Modifiers which apply some modification to location-filtered items.
    pub loc_mods: Vec<BuffLM>,
    /// Modifiers which apply some modification to location- and group-filtered items.
    pub locgroup_mods: Vec<BuffLGM>,
    /// Modifiers which apply some modification to location- and skill requirement-filtered items.
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
    /// Refers an attribute which is the target of the modification.
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
    /// Refers an attribute which is the target of the modification.
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
    /// Refers an attribute which is the target of the modification.
    pub attr_id: ReeInt,
    /// Refers an item group for a modification filter. Only items belonging to this group are
    /// eligible for the modification.
    pub group_id: ReeInt,
}
impl BuffLGM {
    /// Make a new dogma buff auxiliary modifier out of passed data.
    pub fn new(attr_id: ReeInt, group_id: ReeInt) -> BuffLGM {
        BuffLGM { attr_id, group_id }
    }
}
/// Auxiliary data needed to apply a dogma buff modification to location- and skill
/// requirement-filtered items.
#[derive(Debug)]
pub struct BuffLRSM {
    /// Refers an attribute which is the target of the modification.
    pub attr_id: ReeInt,
    /// Refers a skill item for a modification filter. Only items having this skill requirement will
    /// be eligible for the modification.
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
    /// Fighter ability ID.
    pub id: ReeInt,
    /// Fighter ability target mode name.
    pub target_mode: String,
    /// Defines if the ability can be used in hisec.
    pub disallow_hisec: bool,
    /// Defines if the ability can be used in lowsec.
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
    /// Refers an item type involved in the relation.
    pub item_id: ReeInt,
    /// Refers a fighter ability involved in the relation.
    pub abil_id: ReeInt,
    /// Defines cooldown of the ability in seconds.
    pub cooldown: Option<ReeFloat>,
    /// Defines how many times the ability can be used before fighter has to rearm.
    pub charge_count: Option<ReeInt>,
    /// Defines how long each charge of the ability takes to rearm.
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
    /// Refers an item type for which this skill requirement is defined.
    pub item_id: ReeInt,
    /// Refers a skill item type which is needed to meet the skill requirement.
    pub skill_id: ReeInt,
    /// Defines skill level which is needed to meet the skill requirement.
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
