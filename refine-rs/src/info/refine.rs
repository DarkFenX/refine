use crate::{SolInfo, src::SrcInfo};

#[derive(Clone)]
pub struct RefineInfo {
    pub threads_standard: usize,
    pub threads_heavy: usize,
    pub srcs: Vec<SrcInfo>,
    pub sols: Vec<SolInfo>,
}
