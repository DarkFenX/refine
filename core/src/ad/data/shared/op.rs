/// Modifier operators, which are used in effect modification info and buff info.
#[derive(Copy, Clone)]
pub enum AOp {
    PreAssign,
    PreMul,
    PreDiv,
    Add,
    Sub,
    PostMul,
    PostMulImmune,
    PostDiv,
    PostPerc,
    PostAssign,
}
