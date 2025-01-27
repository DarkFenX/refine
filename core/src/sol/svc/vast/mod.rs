use skill_req::SolVastSkillReq;
pub use validation::{SolValOptions, SolValResult};
pub(in crate::sol::svc) use vast::SolVast;
use vast::SolVastFitData;
use vaste_stats_fit::{SolStatRes, SolStatSlot};
pub use vaste_vals_fit::{
    SolMaxGroupItem, SolMaxGroupValFail, SolResUser, SolResValFail, SolRigSizeMismatch, SolRigSizeValFail,
    SolShipLimitMismatch, SolShipLimitValFail, SolSlotIndexValFail, SolSlotValFail, SolSrqSkill, SolSrqValFail,
};

mod skill_req;
mod validation;
mod vast;
mod vaste_debug;
mod vaste_maintain;
mod vaste_stats_fit;
mod vaste_validate;
mod vaste_vals_fit;
