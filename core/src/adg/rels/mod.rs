pub(in crate::adg) use db::KeyDb;
pub(in crate::adg::rels) use funcs::attr_val_to_fk;
pub(in crate::adg) use traits::{Fk, Pk};

mod data;
mod db;
mod funcs;
mod traits;

// Part of primary and foreign keys
pub(in crate::adg) type KeyPart = i32;
