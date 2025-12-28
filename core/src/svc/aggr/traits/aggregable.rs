use super::instance_limit::LimitAmount;
use crate::def::AttrVal;

pub(in crate::svc) trait Aggregable:
    Default
    + std::ops::AddAssign<Self>
    + std::ops::Mul<AttrVal, Output = Self>
    + std::ops::MulAssign<AttrVal>
    + std::ops::Div<AttrVal, Output = Self>
    + LimitAmount
{
}
