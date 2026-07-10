use super::{
    pp_fighter_count::{fighter_count_postproc_fast, fighter_count_postproc_info},
    pp_sec_status::{sec_status_postproc_fast, sec_status_postproc_info},
    pp_skill_level::{skill_level_postproc_fast, skill_level_postproc_info},
};
use crate::{
    svc::{
        SvcCtx,
        calc::{
            AttrValInfo, Calc, CalcAttrVals,
            calce_rah::{
                rah_em_resonance_postproc_fast, rah_em_resonance_postproc_info, rah_expl_resonance_postproc_fast,
                rah_expl_resonance_postproc_info, rah_kin_resonance_postproc_fast, rah_kin_resonance_postproc_info,
                rah_therm_resonance_postproc_fast, rah_therm_resonance_postproc_info,
            },
        },
    },
    ud::UItemId,
};

#[derive(Copy, Clone)]
pub(in crate::svc::calc) enum ItemAttrPostproc {
    SkillLevel,
    FighterCount,
    SecStatus,
    RahEm,
    RahThermal,
    RahKinetic,
    RahExplosive,
}
impl ItemAttrPostproc {
    pub(in crate::svc::calc) fn fast(
        &self,
        calc: &mut Calc,
        ctx: SvcCtx,
        item_uid: UItemId,
        val: CalcAttrVals,
    ) -> CalcAttrVals {
        match self {
            Self::SkillLevel => skill_level_postproc_fast(ctx, item_uid, val),
            Self::FighterCount => fighter_count_postproc_fast(ctx, item_uid, val),
            Self::SecStatus => sec_status_postproc_fast(ctx, item_uid, val),
            Self::RahEm => rah_em_resonance_postproc_fast(calc, ctx, item_uid),
            Self::RahThermal => rah_therm_resonance_postproc_fast(calc, ctx, item_uid),
            Self::RahKinetic => rah_kin_resonance_postproc_fast(calc, ctx, item_uid),
            Self::RahExplosive => rah_expl_resonance_postproc_fast(calc, ctx, item_uid),
        }
    }
    pub(in crate::svc::calc) fn info(
        &self,
        calc: &mut Calc,
        ctx: SvcCtx,
        item_uid: UItemId,
        info: AttrValInfo,
    ) -> AttrValInfo {
        match self {
            Self::SkillLevel => skill_level_postproc_info(ctx, item_uid),
            Self::FighterCount => fighter_count_postproc_info(ctx, item_uid),
            Self::SecStatus => sec_status_postproc_info(ctx, item_uid),
            Self::RahEm => rah_em_resonance_postproc_info(calc, ctx, item_uid, info),
            Self::RahThermal => rah_therm_resonance_postproc_info(calc, ctx, item_uid, info),
            Self::RahKinetic => rah_kin_resonance_postproc_info(calc, ctx, item_uid, info),
            Self::RahExplosive => rah_expl_resonance_postproc_info(calc, ctx, item_uid, info),
        }
    }
}
