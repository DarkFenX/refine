use crate::{
    ad::AAttrId,
    misc::AttrSpec,
    num::{PValue, Value},
    rd::{RAttrId, REffectResist},
    svc::{SvcCtx, calc::Calc},
    ud::{UData, UItemId},
};

impl REffectResist {
    pub(in crate::svc) fn get_mult_by_projection(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        projector_uid: UItemId,
        projectee_uid: UItemId,
    ) -> Option<PValue> {
        let resist_rid = self.get_attr_rid(ctx.u_data, projector_uid)?;
        Self::get_mult_by_aspec(ctx, calc, &AttrSpec::new(projectee_uid, resist_rid))
    }
    pub(in crate::svc) fn get_attr_rid(&self, u_data: &UData, projector_uid: UItemId) -> Option<RAttrId> {
        match self {
            Self::Attr(resist_attr_rid) => Some(*resist_attr_rid),
            Self::AttrRef(ref_attr_rid) => {
                let ref_value = u_data.items.get(projector_uid).get_attr(*ref_attr_rid)?;
                let resist_attr_aid = AAttrId::try_eve_from_f64_rounded(ref_value.into_f64())?;
                u_data.r_data.get_attr_rid_by_aid(&resist_attr_aid)
            }
            Self::RemoteResistance => u_data
                .items
                .get(projector_uid)
                .get_axt()
                .and_then(|v| v.remote_resist_attr_rid),
        }
    }
    pub(in crate::svc) fn get_mult_by_aspec(
        ctx: SvcCtx,
        calc: &mut Calc,
        projectee_aspec: &AttrSpec,
    ) -> Option<PValue> {
        let mult = calc.get_item_attr_odogma(ctx, projectee_aspec.item_uid, projectee_aspec.attr_rid)?;
        Some(match mult <= Value::from_f64(0.0001) {
            true => PValue::ZERO,
            false => PValue::from_f64_unchecked(mult.into_f64()),
        })
    }
}
