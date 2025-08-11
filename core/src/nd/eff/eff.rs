use crate::{
    ad,
    def::AttrVal,
    ed,
    misc::{DmgKinds, EffectSpec, ResolvedSpool, Spool},
    nd::{NEffectCharge, NEffectDmgKind},
    rd,
    svc::{
        SvcCtx,
        calc::{Calc, RawModifier},
        output::{Output, OutputDmgBreacher},
    },
    ud::{UItemKey, UProjRange},
    util::RMap,
};

pub(crate) type NEffectMaker = fn() -> ad::AEffect;
pub(crate) type NEffectAssigner = fn(&mut RMap<ad::AItemId, ad::AItem>) -> bool;
pub(crate) type NEffectUpdater = fn(&mut ad::AEffect);
pub(crate) type NCalcCustomizer = fn(&mut Vec<RawModifier>, EffectSpec);
pub(crate) type NProjMultGetter = fn(SvcCtx, &mut Calc, UItemKey, &rd::REffect, UProjRange) -> AttrVal;
pub(crate) type NSpoolResolver = fn(SvcCtx, &mut Calc, UItemKey, &rd::REffect, Option<Spool>) -> Option<ResolvedSpool>;
pub(crate) type NProjAttrGetter = fn(&ad::AEffect) -> [Option<ad::AAttrId>; 2];
pub(crate) type NNormalDmgGetter =
    fn(SvcCtx, &mut Calc, UItemKey, &rd::REffect, Option<Spool>, Option<UItemKey>) -> Option<Output<DmgKinds<AttrVal>>>;
pub(crate) type NBreacherDmgGetter =
    fn(SvcCtx, &mut Calc, UItemKey, &rd::REffect, Option<UItemKey>) -> Option<OutputDmgBreacher>;
pub(crate) type NLocalRepGetter = fn(SvcCtx, &mut Calc, UItemKey, &rd::REffect) -> Option<Output<AttrVal>>;
pub(crate) type NRemoteRepGetter =
    fn(SvcCtx, &mut Calc, UItemKey, &rd::REffect, Option<Spool>, Option<UItemKey>) -> Option<Output<AttrVal>>;

pub(crate) struct NEffect {
    // EVE data effect ID. Not all effects have it, since some are added via other means
    pub(crate) eid: Option<ed::EEffectId>,
    // Adapted data effect ID
    pub(crate) aid: ad::AEffectId,
    // Specifies if effect applies any buffs
    pub(crate) adg_buff_info: Option<ad::AEffectBuffInfo> = None,
    // Data customization function ran during cache generation time
    pub(crate) adg_make_effect_fn: Option<NEffectMaker> = None,
    pub(crate) adg_assign_effect_fn: Option<NEffectAssigner> = None,
    pub(crate) adg_update_effect_fn: Option<NEffectUpdater> = None,
    // Getter for attribute IDs which define projection range of effect
    pub(crate) xt_get_proj_attrs: Option<NProjAttrGetter> = None,
    // Effect data hardcoded in the library
    pub(crate) hc: NEffectHc = NEffectHc { .. },
}

#[derive(Copy, Clone, Default)]
pub(crate) struct NEffectHc {
    pub(crate) dmg_kind: Option<NEffectDmgKind> = None,
    pub(crate) charge: Option<NEffectCharge> = None,
    pub(crate) kills_item: bool = false,
    // Effect modifier customization function ran during runtime in calculator service
    pub(crate) calc_customizer: Option<NCalcCustomizer> = None,
    // Effect strength-related
    pub(crate) proj_mult_getter: Option<NProjMultGetter> = None,
    pub(crate) spool_resolver: Option<NSpoolResolver> = None,
    // Functions which fetch output per cycle
    pub(crate) normal_dmg_opc_getter: Option<NNormalDmgGetter> = None,
    pub(crate) breacher_dmg_opc_getter: Option<NBreacherDmgGetter> = None,
    pub(crate) local_shield_rep_opc_getter: Option<NLocalRepGetter> = None,
    pub(crate) local_armor_rep_opc_getter: Option<NLocalRepGetter> = None,
    pub(crate) local_hull_rep_opc_getter: Option<NLocalRepGetter> = None,
    pub(crate) remote_shield_rep_opc_getter: Option<NRemoteRepGetter> = None,
    pub(crate) remote_armor_rep_opc_getter: Option<NRemoteRepGetter> = None,
    pub(crate) remote_hull_rep_opc_getter: Option<NRemoteRepGetter> = None,
    pub(crate) remote_cap_rep_opc_getter: Option<NRemoteRepGetter> = None,
}
