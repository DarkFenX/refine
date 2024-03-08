//! Service extension methods which handle attribute calculation.

pub(in crate::ss::svc) use data::CalcData;
pub use misc::SsAttrVal;

mod data;
mod misc;
mod modifier;
mod registers;
mod svce_attr;
mod svce_buff;
mod svce_calc;
