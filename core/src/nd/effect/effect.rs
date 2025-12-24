use crate::{
    ad::{AAttrId, AEffect, AEffectBuff, AEffectId, AItem, AItemId},
    def::AttrVal,
    ed::EEffectId,
    misc::{DmgKinds, Ecm, EffectSpec, MiningAmount, Spool},
    nd::{NEffectCharge, NEffectDmgKind, NEffectLocalOpcSpec, NEffectProjOpcSpec, NEffectProjecteeFilter, NSpoolRaw},
    rd::{RAttrConsts, REffect},
    svc::{
        SvcCtx,
        calc::{Calc, RawModifier},
        output::{Output, OutputDmgBreacher},
    },
    ud::{UItem, UItemKey, UProjData},
    util::RMap,
};

// ADG
pub(crate) type NEffectMaker = fn() -> AEffect;
pub(crate) type NEffectAssigner = fn(&mut RMap<AItemId, AItem>) -> bool;
pub(crate) type NEffectUpdater = fn(&mut AEffect);
// General
pub(crate) type NSpoolGetter = fn(SvcCtx, &mut Calc, UItemKey) -> Option<NSpoolRaw>;
pub(crate) type NCalcCustomizer = fn(&mut Vec<RawModifier>, &RAttrConsts, EffectSpec);
// Getters - projection
pub(crate) type NModProjAttrGetter = fn(&AEffect) -> [Option<AAttrId>; 2];
pub(crate) type NProjMultGetter = fn(SvcCtx, &mut Calc, UItemKey, &REffect, UItemKey, UProjData) -> AttrVal;
// Getters - damage output
pub(crate) type NDmgKindGetter = fn(&UItem) -> NEffectDmgKind;
pub(crate) type NNormalDmgGetter =
    fn(SvcCtx, &mut Calc, UItemKey, &REffect, Option<Spool>, Option<UItemKey>) -> Option<Output<DmgKinds<AttrVal>>>;
pub(crate) type NBreacherDmgGetter =
    fn(SvcCtx, &mut Calc, UItemKey, &REffect, Option<UItemKey>) -> Option<OutputDmgBreacher>;
// Getters - misc
pub(crate) type NMiningGetter = fn(SvcCtx, &mut Calc, UItemKey, &REffect) -> Option<Output<MiningAmount>>;
pub(crate) type NNeutGetter = fn(SvcCtx, &mut Calc, UItemKey, &REffect, Option<UItemKey>) -> Option<Output<AttrVal>>;
pub(crate) type NCapInjectGetter = fn(SvcCtx, &mut Calc, UItemKey) -> Option<AttrVal>;
pub(crate) type NEcmGetter = fn(SvcCtx, &mut Calc, UItemKey, &REffect, Option<UItemKey>) -> Option<Ecm>;

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
    pub(crate) spool_getter: Option<NSpoolGetter> = None,
    // Effect modifier customization function ran during runtime in calculator service
    pub(crate) calc_customizer: Option<NCalcCustomizer> = None,
    // Getters - modifier projection
    pub(crate) modifier_proj_attrs_getter: Option<NModProjAttrGetter> = None,
    pub(crate) modifier_proj_mult_getter: Option<NProjMultGetter> = None,
    // Getters - damage output
    pub(crate) dmg_kind_getter: Option<NDmgKindGetter> = None,
    pub(crate) normal_dmg_opc_getter: Option<NNormalDmgGetter> = None,
    pub(crate) breacher_dmg_opc_getter: Option<NBreacherDmgGetter> = None,
    // Getters - mining
    pub(crate) mining_ore_opc_getter: Option<NMiningGetter> = None,
    pub(crate) mining_ice_opc_getter: Option<NMiningGetter> = None,
    pub(crate) mining_gas_opc_getter: Option<NMiningGetter> = None,
    // Getters - rep output
    pub(crate) outgoing_shield_rep_opc_spec: Option<NEffectProjOpcSpec<AttrVal>> = None,
    pub(crate) outgoing_armor_rep_opc_spec: Option<NEffectProjOpcSpec<AttrVal>> = None,
    pub(crate) outgoing_hull_rep_opc_spec: Option<NEffectProjOpcSpec<AttrVal>> = None,
    // Getters - local reps
    pub(crate) local_shield_rep_opc_spec: Option<NEffectLocalOpcSpec<AttrVal>> = None,
    pub(crate) local_armor_rep_opc_spec: Option<NEffectLocalOpcSpec<AttrVal>> = None,
    pub(crate) local_hull_rep_opc_spec: Option<NEffectLocalOpcSpec<AttrVal>> = None,
    // Getters - cap
    pub(crate) neut_opc_getter: Option<NNeutGetter> = None,
    pub(crate) outgoing_cap_opc_spec: Option<NEffectProjOpcSpec<AttrVal>> = None,
    pub(crate) cap_inject_getter: Option<NCapInjectGetter> = None,
    // Getters - misc
    pub(crate) ecm_opc_getter: Option<NEcmGetter> = None,
}
