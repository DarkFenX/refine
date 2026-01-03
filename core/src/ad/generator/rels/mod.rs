pub(in crate::ad::generator) use db::KeyDb;
pub(in crate::ad::generator::rels) use funcs::attr_val_to_fk;
pub(in crate::ad::generator) use key::KeyPart;
pub(in crate::ad::generator) use traits::{Fk, Pk};

mod data;
mod db;
mod funcs;
mod key;
mod traits;
