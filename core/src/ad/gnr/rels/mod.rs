pub(in crate::ad::gnr) use db::KeyDb;
pub(in crate::ad::gnr::rels) use funcs::attr_val_to_fk;
pub(in crate::ad::gnr) use traits::{Fk, Pk};

mod data;
mod db;
mod funcs;
mod traits;

// Part of primary and foreign keys
pub(in crate::ad::gnr) type KeyPart = i32;
