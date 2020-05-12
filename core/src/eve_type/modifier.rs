use crate::defines::Id;
use crate::consts::{EveModDomain, EveModOperator};

pub struct ItemModifier {
    pub affectee_domain: EveModDomain,
    pub affectee_attr_id: Id,
    pub operator: EveModOperator,
    pub affector_attr_id: Id
}

impl ItemModifier {
    pub fn new(
        affectee_domain: EveModDomain,
        affectee_attr_id: Id,
        operator: EveModOperator,
        affector_attr_id: Id
    ) -> ItemModifier {
        ItemModifier {
            affectee_domain,
            affectee_attr_id,
            operator,
            affector_attr_id
        }
    }
}
