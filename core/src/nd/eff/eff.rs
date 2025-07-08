use crate::{
    ad,
    def::{AttrVal, ItemKey},
    ed,
    misc::EffectSpec,
    nd::NEffectCharge,
    svc::{
        SvcCtx,
        calc::{Calc, RawModifier},
    },
    uad::UadProjRange,
};

pub(crate) type NProjMultGetter = fn(SvcCtx, &mut Calc, ItemKey, &ad::AEffect, UadProjRange) -> AttrVal;
pub(crate) type NProjAttrGetter = fn(&ad::AEffect) -> [Option<ad::AAttrId>; 2];
pub(crate) type NLocalRepGetter = fn(SvcCtx, &mut Calc, ItemKey) -> Option<AttrVal>;
pub(crate) type NRemoteRepGetter = fn(SvcCtx, &mut Calc, EffectSpec, Option<ItemKey>) -> Option<AttrVal>;

pub(crate) struct NEffect {
    // EVE data effect ID. Not all effects have it, since some are added via other means
    pub(crate) eid: Option<ed::EEffectId>,
    // Adapted data effect ID
    pub(crate) aid: ad::AEffectId,
    // Specifies if effect applies any buffs
    pub(crate) adg_buff_info: Option<ad::AEffectBuffInfo> = None,
    // Data customization function ran during cache generation time. Can change anything, but is
    // attached to effect, since it mostly operates on effects
    pub(crate) adg_custom_fn: Option<fn(&mut ad::AData)> = None,
    // Getter for attribute IDs which define projection range of effect
    pub(crate) xt_get_proj_attrs: Option<NProjAttrGetter> = None,
    // Effect data hardcoded in the library
    pub(crate) hc: NEffectHc = NEffectHc { .. },
}

#[derive(Copy, Clone, Default)]
pub(crate) struct NEffectHc {
    pub(crate) charge: Option<NEffectCharge> = None,
    // Effect modifier customization function ran during runtime in calculator service
    pub(crate) calc_custom_fn: Option<fn(&mut Vec<RawModifier>, EffectSpec)> = None,
    // Projection-related
    pub(crate) get_proj_mult: Option<NProjMultGetter> = None,
    // Functions which fetch various stats
    pub(crate) get_local_armor_rep_amount: Option<NLocalRepGetter> = None,
    pub(crate) get_local_shield_rep_amount: Option<NLocalRepGetter> = None,
    pub(crate) get_local_structure_rep_amount: Option<NLocalRepGetter> = None,
    pub(crate) get_remote_armor_rep_amount: Option<NRemoteRepGetter> = None,
    pub(crate) get_remote_shield_rep_amount: Option<NRemoteRepGetter> = None,
    pub(crate) get_remote_structure_rep_amount: Option<NRemoteRepGetter> = None,
}
