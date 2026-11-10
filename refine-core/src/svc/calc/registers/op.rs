use std::hash::Hash;

use crate::util::RMapRSet;

/// Whether a register function registers or unregisters what it is given.
#[derive(std::marker::ConstParamTy, Copy, Clone, Eq, PartialEq)]
pub(in crate::svc::calc) enum RegOp {
    Reg,
    Unreg,
}

pub(in crate::svc::calc) fn process_entry<const OP: RegOp, K, V>(storage: &mut RMapRSet<K, V>, key: K, value: V)
where
    K: Eq + Hash,
    V: Eq + Hash,
{
    match OP {
        RegOp::Reg => storage.add_entry(key, value),
        RegOp::Unreg => storage.remove_entry(key, &value),
    }
}
