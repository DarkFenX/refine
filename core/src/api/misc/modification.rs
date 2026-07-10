use crate::{
    api::{AttrId, Op},
    num::{PValue, Value},
    svc::calc::{CalcModInfo, CalcModInfoAffector},
    ud::{ItemId, UData},
};

pub struct Modification {
    pub op: Op,
    pub initial_str: Value,
    pub range_mult: Option<PValue>,
    pub resist_mult: Option<PValue>,
    pub stacking_mult: Option<PValue>,
    pub applied_str: Value,
    pub affectors: Vec<Affector>,
}

pub struct Affector {
    pub item_id: ItemId,
    pub attr_id: Option<AttrId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Modification {
    pub(crate) fn from_calc(calc_mod_info: CalcModInfo, u_data: &UData) -> Self {
        Self {
            op: calc_mod_info.op,
            initial_str: calc_mod_info.initial_str,
            range_mult: calc_mod_info.range_mult,
            resist_mult: calc_mod_info.resist_mult,
            stacking_mult: calc_mod_info.stacking_mult,
            applied_str: calc_mod_info.applied_str,
            affectors: calc_mod_info
                .affectors
                .into_iter()
                .map(|calc_affector| Affector::from_calc(calc_affector, u_data))
                .collect(),
        }
    }
}

impl Affector {
    pub(crate) fn from_calc(calc_affector: CalcModInfoAffector, u_data: &UData) -> Self {
        Self {
            item_id: u_data.items.ext_id_by_int_id(calc_affector.item_uid),
            attr_id: calc_affector
                .attr_rid
                .map(|attr_rid| AttrId::from_aid(u_data.r_data.get_attr_by_rid(attr_rid).aid)),
        }
    }
}
