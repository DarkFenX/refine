use crate::{def::AttrVal, util::Limit};

pub(in crate::svc::aggr) trait Aggregable:
    Default
    + std::ops::AddAssign<Self>
    + std::ops::Mul<AttrVal, Output = Self>
    + std::ops::MulAssign<AttrVal>
    + std::ops::Div<AttrVal, Output = Self>
    + Limit
{
}
