/// Whether a register function registers or unregisters what it is given.
#[derive(std::marker::ConstParamTy, Copy, Clone, Eq, PartialEq)]
pub(in crate::svc::calc) enum RegOp {
    Reg,
    Unreg,
}
