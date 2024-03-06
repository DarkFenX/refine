pub(in crate::ss::svc::calc) use attr_mod::make_mod;
pub(in crate::ss::svc::calc::modifier) use mod_val_getter::get_mod_val;
pub(in crate::ss::svc::calc::modifier) use revision::revise_on_item_add_removal;
pub(in crate::ss::svc::calc::modifier) use src_attr::AAR_SRC_ATTR_ID;

mod attr_mod;
mod mod_val_getter;
mod revision;
mod src_attr;
