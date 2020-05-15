use crate::consts::{EveModDomain, EveModOperator};
use crate::defines::ReeInt;

pub struct ItemModifier {
    pub affectee_domain: EveModDomain,
    pub affectee_attr_id: ReeInt,
    pub operator: EveModOperator,
    pub affector_attr_id: ReeInt,
}

impl ItemModifier {
    pub fn new(
        affectee_domain: EveModDomain,
        affectee_attr_id: ReeInt,
        operator: EveModOperator,
        affector_attr_id: ReeInt,
    ) -> ItemModifier {
        ItemModifier {
            affectee_domain,
            affectee_attr_id,
            operator,
            affector_attr_id,
        }
    }
}
