use crate::{
    ac, ad,
    sol::{
        ItemKey,
        svc::calc::{
            AggrMode, Op,
            modifier::{AffecteeFilter, Location, ModifierKind, RawModifier, affector_val::AffectorValue},
        },
    },
};

pub(in crate::sol::svc::calc) fn make_mod(affector_item_key: ItemKey, a_effect_id: ad::AEffectId) -> RawModifier {
    RawModifier {
        kind: ModifierKind::Local,
        affector_item_key,
        a_effect_id,
        affector_value: AffectorValue::AarRepAmount,
        op: Op::ExtraMul,
        aggr_mode: AggrMode::Stack,
        affectee_filter: AffecteeFilter::Direct(Location::Item),
        affectee_a_attr_id: ac::attrs::ARMOR_DMG_AMOUNT,
        buff_type_a_attr_id: None,
        resist_a_attr_id: None,
        optimal_a_attr_id: None,
        falloff_a_attr_id: None,
    }
}
