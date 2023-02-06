use crate::{ct, Fit, ReeInt};
use std::sync::Arc;

pub trait ItemBase {
    fn get_type_id(&self) -> ReeInt;
    // fn get_parent();
    // fn get_state();
    // fn get_mod_domain();
    // fn is_owner_modifiable();
    // fn get_solsys_carrier();
}

pub(crate) trait IntItemBase {
    fn get_item(&self) -> Option<&ct::Item>;
    fn load_item(&mut self);
}

pub(crate) trait FitChild {
    fn get_fit(&self) -> Option<Arc<Fit>>;
    fn set_fit(&mut self, fit: Option<Arc<Fit>>);
}

//     running_effect_ids: Vec<ReeInt>,
//     attr_map: ReeInt, // stub
//     effect_mode_overrides: ReeInt, // stub
//     autocharges: ReeInt, // stub
