use crate::{
    ad::{AEffect, AEffectBuff, AEffectId, AItem, AItemId},
    nd::{
        NEffectBreacherOutputGetter, NEffectCharge, NEffectDmgKindGetter, NEffectDmgOutputGetter, NEffectDuration,
        NEffectEcm, NEffectGeneralOutputGetter, NEffectLocalOpcSpec, NEffectMining, NEffectNeut, NEffectProjModSpec,
        NEffectProjOpcSpec, NEffectProjecteeFilter, NEffectSpoolAttrs,
    },
    svc::calc::CalcCustomModifier,
    util::RMap,
};

// ADG
pub(crate) type NEffectMaker = fn() -> AEffect;
pub(crate) type NEffectAssigner = fn(&mut RMap<AItemId, AItem>) -> bool;
pub(crate) type NEffectUpdater = fn(&mut AEffect);

pub(crate) struct NEffect {
    // Adapted data effect ID
    pub(crate) aid: AEffectId,
    // Fields related to adapted data generation - buff info and effect customization functions
    pub(crate) adg_buff: Option<AEffectBuff> = None,
    pub(crate) adg_make_effect_fn: Option<NEffectMaker> = None,
    pub(crate) adg_assign_effect_fn: Option<NEffectAssigner> = None,
    pub(crate) adg_update_effect_fn: Option<NEffectUpdater> = None,
    // General info which is not available elsewhere
    pub(crate) charge: Option<NEffectCharge> = None,
    pub(crate) projectee_filter: Option<NEffectProjecteeFilter> = None,
    pub(crate) ignore_offmod_immunity: bool = false,
    pub(crate) cloaks_carrier: bool = false,
    pub(crate) disallows_cloak: Option<NEffectDuration> = None,
    pub(crate) disallows_warp: Option<NEffectDuration> = None,
    pub(crate) disallows_jump_gate: Option<NEffectDuration> = None,
    pub(crate) disallows_jump_wh: Option<NEffectDuration> = None,
    pub(crate) disallows_jump_drive: Option<NEffectDuration> = None,
    pub(crate) disallows_dock: Option<NEffectDuration> = None,
    pub(crate) disallows_tether: Option<NEffectDuration> = None,
    pub(crate) kills_item: bool = false,
    pub(crate) spool_attrs: Option<NEffectSpoolAttrs> = None,
    // Effect modifier customization ran during runtime in calculator service
    pub(crate) calc_custom_mod: Option<CalcCustomModifier> = None,
    // Specs - modifier
    pub(crate) proj_mod: Option<NEffectProjModSpec> = None,
    // Specs - damage output
    pub(crate) dmg_kind: Option<NEffectDmgKindGetter> = None,
    pub(crate) normal_dmg: Option<NEffectProjOpcSpec<NEffectDmgOutputGetter>> = None,
    pub(crate) breacher_dmg: Option<NEffectProjOpcSpec<NEffectBreacherOutputGetter>> = None,
    // Specs - mining
    pub(crate) mining_ore: Option<NEffectMining> = None,
    pub(crate) mining_ice: Option<NEffectMining> = None,
    pub(crate) mining_gas: Option<NEffectMining> = None,
    // Specs - rep output
    pub(crate) outgoing_shield_rep: Option<NEffectProjOpcSpec<NEffectGeneralOutputGetter>> = None,
    pub(crate) outgoing_armor_rep: Option<NEffectProjOpcSpec<NEffectGeneralOutputGetter>> = None,
    pub(crate) outgoing_hull_rep: Option<NEffectProjOpcSpec<NEffectGeneralOutputGetter>> = None,
    // Specs - local reps
    pub(crate) local_shield_rep: Option<NEffectLocalOpcSpec<NEffectGeneralOutputGetter>> = None,
    pub(crate) local_armor_rep: Option<NEffectLocalOpcSpec<NEffectGeneralOutputGetter>> = None,
    pub(crate) local_hull_rep: Option<NEffectLocalOpcSpec<NEffectGeneralOutputGetter>> = None,
    // Specs - cap
    pub(crate) cap_consume: Option<NEffectLocalOpcSpec<NEffectGeneralOutputGetter>> = None,
    pub(crate) neut: Option<NEffectNeut> = None,
    // Nosf spec is used only for purposes of cap balance/sim calcs
    pub(crate) nosf: Option<NEffectProjOpcSpec<NEffectGeneralOutputGetter>> = None,
    pub(crate) outgoing_cap: Option<NEffectProjOpcSpec<NEffectGeneralOutputGetter>> = None,
    pub(crate) cap_inject: Option<NEffectLocalOpcSpec<NEffectGeneralOutputGetter>> = None,
    // Specs - misc
    pub(crate) ecm: Option<NEffectEcm> = None,
}
