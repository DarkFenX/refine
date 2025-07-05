use crate::{
    ad,
    def::{AttrVal, ItemKey},
    ed,
    misc::EffectSpec,
    svc::{
        SvcCtx,
        calc::{Calc, RawModifier},
    },
    uad::UadProjRange,
};

pub(crate) type ProjMultGetter = fn(SvcCtx, &mut Calc, ItemKey, &ad::AEffect, UadProjRange) -> AttrVal;
pub(crate) type LocalRepGetter = fn(SvcCtx, &mut Calc, ItemKey) -> Option<AttrVal>;
pub(crate) type RemoteRepGetter = fn(SvcCtx, &mut Calc, &EffectSpec, Option<ItemKey>) -> Option<AttrVal>;

pub(crate) struct NttEffect {
    // EVE data effect ID. Not all effects have it, since some are added via other means
    pub(crate) eid: Option<ed::EEffectId>,
    // Adapted data effect ID
    pub(crate) aid: ad::AEffectId,
    // Specifies if effect applies any buffs
    pub(crate) adg_buff_info: Option<ad::AEffectBuffInfo> = None,
    // Specifies if effect uses charges
    pub(crate) adg_charge_info: Option<ad::AEffectChargeInfo> = None,
    // Data customization function ran during cache generation time. Can change anything, but is
    // attached to effect, since it mostly operates on effects
    pub(crate) adg_custom_fn: Option<fn(&mut ad::AData)> = None,
    // Effect data
    pub(crate) rt: NttEffectRt = NttEffectRt { .. },
}

#[derive(Copy, Clone, Default)]
pub struct NttEffectRt {
    // Effect modifier customization function ran during runtime in calculator service
    pub(crate) calc_custom_fn: Option<fn(&mut Vec<RawModifier>, EffectSpec)> = None,
    pub(crate) get_proj_mult: Option<ProjMultGetter> = None,
    // Functions which fetch various stats
    pub(crate) get_local_armor_rep_amount: Option<LocalRepGetter> = None,
    pub(crate) get_local_shield_rep_amount: Option<LocalRepGetter> = None,
    pub(crate) get_local_structure_rep_amount: Option<LocalRepGetter> = None,
    pub(crate) get_remote_armor_rep_amount: Option<RemoteRepGetter> = None,
    pub(crate) get_remote_shield_rep_amount: Option<RemoteRepGetter> = None,
    pub(crate) get_remote_structure_rep_amount: Option<RemoteRepGetter> = None,
}
