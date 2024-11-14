use crate::{
    ad, ec,
    sol::{
        item::SolItem,
        svc::svce_calc::{
            modifier::{
                affector_val::SolAffectorValue, get_resist_attr_id, SolAffecteeFilter, SolDomain, SolModifierKind,
                SolRawModifier,
            },
            SolAggrMode, SolOp,
        },
    },
};

pub(in crate::sol::svc::svce_calc) fn make_mod(item: &SolItem, effect: &ad::AEffect) -> SolRawModifier {
    SolRawModifier::new(
        SolModifierKind::Targeted,
        item.get_id(),
        effect.id,
        SolAffectorValue::AttrId(ec::attrs::SPEED_FACTOR),
        SolOp::PostPerc,
        SolAggrMode::Stack,
        SolAffecteeFilter::Direct(SolDomain::Target),
        ec::attrs::MAX_VELOCITY,
        None,
        get_resist_attr_id(item, effect),
        effect.range_attr_id,
        effect.falloff_attr_id,
    )
}
