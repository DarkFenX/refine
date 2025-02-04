use detailed::HValidInfoDetailed;
use simple::HValidInfoSimple;

mod detailed;
mod details;
mod simple;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HValidInfo {
    Simple(HValidInfoSimple),
    Detailed(Box<HValidInfoDetailed>),
}
impl From<bool> for HValidInfo {
    fn from(core_value: bool) -> Self {
        Self::Simple(core_value.into())
    }
}
impl From<&rc::SolValResult> for HValidInfo {
    fn from(core_value: &rc::SolValResult) -> Self {
        Self::Detailed(Box::new(core_value.into()))
    }
}
