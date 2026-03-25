use crate::{
    ad::{AAttrId, AEffect, AEffectBuff, AEffectId, AItem, AItemId},
    ed::EEffectId,
    misc::{Breacher, Ecm, EffectSpec},
    nd::{
        NBaseBreacherDmgGetter, NBaseMiningGetter, NBaseNormalDmgGetter, NBaseRemoteCapGetter, NEffectCharge,
        NEffectDmgKindGetter, NEffectLocalOpcSpec, NEffectProjMultGetter, NEffectProjOpcSpec, NEffectProjecteeFilter,
        NEffectSpoolAttrs,
    },
    num::PValue,
    rd::RAttrConsts,
    svc::calc::RawModifier,
    util::RMap,
};

// ADG
pub(crate) type NEffectMaker = fn() -> AEffect;
pub(crate) type NEffectAssigner = fn(&mut RMap<AItemId, AItem>) -> bool;
pub(crate) type NEffectUpdater = fn(&mut AEffect);
// General
pub(crate) type NEffectCalcCustomizer = fn(&mut Vec<RawModifier>, &RAttrConsts, EffectSpec);
// Getters
pub(crate) type NEffectModProjAttrGetter = fn(&AEffect) -> [Option<AAttrId>; 2];

pub(crate) struct NEffect {
    // EVE data effect ID. Not all effects have it, since some are added via other means
    pub(crate) eid: Option<EEffectId>,
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
    pub(crate) kills_item: bool = false,
    pub(crate) spool_attrs: Option<NEffectSpoolAttrs> = None,
    // Effect modifier customization function ran during runtime in calculator service
    pub(crate) calc_customizer: Option<NEffectCalcCustomizer> = None,
    // Getters/specs - modifier projection
    pub(crate) modifier_proj_attrs_getter: Option<NEffectModProjAttrGetter> = None,
    pub(crate) modifier_proj_mult_getter: Option<NEffectProjMultGetter> = None,
    // Getters/specs - damage output
    pub(crate) dmg_kind_getter: Option<NEffectDmgKindGetter> = None,
    pub(crate) normal_dmg_opc_spec: Option<NEffectProjOpcSpec<NBaseNormalDmgGetter>> = None,
    pub(crate) breacher_dmg_opc_spec: Option<NEffectProjOpcSpec<NBaseBreacherDmgGetter>> = None,
    // Getters/specs - mining
    pub(crate) mining_ore_opc_spec: Option<NEffectProjOpcSpec<NBaseMiningGetter>> = None,
    pub(crate) mining_ice_opc_spec: Option<NEffectProjOpcSpec<NBaseMiningGetter>> = None,
    pub(crate) mining_gas_opc_spec: Option<NEffectProjOpcSpec<NBaseMiningGetter>> = None,
    // Getters/specs - rep output
    pub(crate) outgoing_shield_rep_opc_spec: Option<NEffectProjOpcSpec<PValue>> = None,
    pub(crate) outgoing_armor_rep_opc_spec: Option<NEffectProjOpcSpec<PValue>> = None,
    pub(crate) outgoing_hull_rep_opc_spec: Option<NEffectProjOpcSpec<PValue>> = None,
    // Getters/specs - local reps
    pub(crate) local_shield_rep_opc_spec: Option<NEffectLocalOpcSpec<PValue>> = None,
    pub(crate) local_armor_rep_opc_spec: Option<NEffectLocalOpcSpec<PValue>> = None,
    pub(crate) local_hull_rep_opc_spec: Option<NEffectLocalOpcSpec<PValue>> = None,
    // Getters/specs - cap
    pub(crate) cap_consume_opc_spec: Option<NEffectLocalOpcSpec<PValue>> = None,
    pub(crate) neut_opc_spec: Option<NEffectProjOpcSpec<PValue>> = None,
    // Nosf spec is used only for purposes of cap balance/sim calcs
    pub(crate) nosf_opc_spec: Option<NEffectProjOpcSpec<PValue>> = None,
    pub(crate) outgoing_cap_opc_spec: Option<NEffectProjOpcSpec<NBaseRemoteCapGetter>> = None,
    pub(crate) cap_inject_opc_spec: Option<NEffectLocalOpcSpec<PValue>> = None,
    // Getters/specs - misc
    pub(crate) ecm_opc_spec: Option<NEffectProjOpcSpec<Ecm>> = None,
}
