pub(in crate::ad::generator) use db::KeyDb;
pub(in crate::ad::generator::rels) use funcs::attr_val_to_fk;
pub(in crate::ad::generator) use traits::{Fk, Pk};

mod data;
mod db;
mod funcs;
mod traits;

// Part of primary and foreign keys
pub(in crate::ad::generator) type KeyPart = i32;
