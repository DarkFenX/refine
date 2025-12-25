use crate::def::AttrVal;

#[derive(Copy, Clone)]
pub(in crate::svc::vast::vaste_stats) struct EffectLocalInvarData {
    pub(super) ilimit: Option<AttrVal>,
}

#[derive(Copy, Clone)]
pub(in crate::svc::vast::vaste_stats) struct EffectProjInvarData {
    pub(super) mult_pre: Option<AttrVal>,
    pub(super) ilimit: Option<AttrVal>,
    pub(super) mult_post: Option<AttrVal>,
}
