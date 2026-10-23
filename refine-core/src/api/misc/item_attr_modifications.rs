use crate::{
    AttrId, ItemId, Op, PValue, UnitInterval, Value,
    svc::calc::{CalcModInfo, CalcModInfoAffector},
    ud::UData,
};

/// Single modification applied to an item attribute.
#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Clone)]
pub struct Modification {
    pub op: Op,
    pub initial_str: Value,
    pub range_mult: Option<PValue>,
    pub resist_mult: Option<UnitInterval>,
    pub stacking_mult: Option<UnitInterval>,
    pub applied_str: Value,
    pub affectors: Vec<Affector>,
}

/// Item and its attribute which contributed to a modification.
///
/// Not all affectors have attribute specified, some use hardcoded values.
#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct Affector {
    pub item_id: ItemId,
    pub attr_id: Option<AttrId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Modification {
    pub(in crate::api) fn from_calc(calc_mod_info: CalcModInfo, u_data: &UData) -> Self {
        let CalcModInfo {
            op,
            initial_str,
            range_mult,
            resist_mult,
            stacking_mult,
            applied_str,
            affectors,
            // Not exposed
            effect_rid: _,
        } = calc_mod_info;
        Self {
            op,
            initial_str,
            range_mult,
            resist_mult,
            stacking_mult,
            applied_str,
            affectors: affectors
                .into_iter()
                .map(|calc_affector| Affector::from_calc(calc_affector, u_data))
                .collect(),
        }
    }
}

impl Affector {
    pub(in crate::api) fn from_calc(calc_affector: CalcModInfoAffector, u_data: &UData) -> Self {
        let CalcModInfoAffector { item_uid, attr_rid } = calc_affector;
        Self {
            item_id: u_data.items.ext_id_by_int_id(item_uid),
            attr_id: attr_rid.map(|attr_rid| AttrId::from_aid(u_data.r_data.get_attr_by_rid(attr_rid).aid)),
        }
    }
}
