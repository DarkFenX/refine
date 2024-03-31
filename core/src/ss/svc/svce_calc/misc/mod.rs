pub use attr_val::SsAttrVal;
pub(in crate::ss::svc::svce_calc) use attr_val_data::AttrValData;
pub(in crate::ss::svc::svce_calc) use loc_type::SsLocType;
pub(in crate::ss::svc::svce_calc) use mod_gen::a_data_to_ss_mods;
pub(in crate::ss::svc::svce_calc) use modification::Modification;
pub(in crate::ss::svc::svce_calc) use modification_key::ModKey;

mod attr_val;
mod attr_val_data;
mod item_exts;
mod loc_type;
mod mod_gen;
mod modification;
mod modification_key;
