use crate::{
    shared::{HDpsProfile, HSpool},
    util::default_quarter,
};

#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatOptionErps {
    pub(in crate::cmd) incoming_dps: Option<HDpsProfile>,
    #[serde(default = "default_quarter")]
    #[educe(Default = 0.25)]
    pub(in crate::cmd) shield_perc: rc::AttrVal,
    pub(in crate::cmd) spool: Option<HSpool>,
}
