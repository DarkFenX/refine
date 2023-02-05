use crate::{ct, ReeInt};

pub trait ItemBase {
    fn get_type_id(&self) -> ReeInt;
    fn get_item(&self) -> Option<&ct::Item>;
    // fn get_parent();
    // fn get_state();
    // fn get_mod_domain();
    // fn is_owner_modifiable();
    // fn get_solsys_carrier();
}

//     running_effect_ids: Vec<ReeInt>,
//     attr_map: ReeInt, // stub
//     effect_mode_overrides: ReeInt, // stub
//     autocharges: ReeInt, // stub
