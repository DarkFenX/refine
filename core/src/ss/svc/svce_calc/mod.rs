//! Service extension methods which handle attribute calculation.
//!
//! Calculator consists of a set of registers which serve two main purposes: 1) when an attribute is
//! being calculated - they tell what modifies it and how, and 2) when an attribute's value changes,
//! they tell which attributes rely on this one, to force their recalculation too. But since
//! relations between attributes are complex, there are multiple registers which handle it.
//!
//! - Modifier register - primary register which provides data on which modifiers affects specific
//! attr on a specific item;
//! - Target register - primary register which provides info on which items are modified by a
//! modifier;
//! - Projection register - provides extra info about effects which apply to items which do not
//! belong to fit of modifier, or which apply to multiple targets;
//! - Dependency register - tracks direct dependencies between attribute values, which cannot be
//! tracked otherwise;
//! - Revision register - keeps track of custom modifiers which depend on various events not related
//! to attribute changes, and whenever significant events happen, forces recalculation of attribute
//! values it modifies. Custom ancillary armor repairer modifier uses this register.
//!
//! Next, there are a few scenarios on how those registers are used:
//!
//! - Fit-local dogma modifiers: those use modifier register and target register to provide info for
//! both directions: finding modifiers which affect an attr on an item, and finding items which are
//! affected by a modifier;
//! - System-wide and projected dogma modifiers: same as fit-local dogma modifier, but also use data
//! from projection register to define which fits are affected;
//! - Attribute value caps/limits: they are using dependency register, and relation between a
//! limiting attribute and a limited attribute is registered during calculation of the limited
//! attribute. Relation is removed only when item is unloaded;
//! - Custom ancillary repairer modifier: uses revision register to clear rep amount attribute
//! whenever it loads/unloads paste as its charge;
//! - Custom AB/MWD modifier: uses dependency register to establish relationship between ship speed,
//! ship mass, prop speed boost, and prop thrust during modifier calculation. This relationship is
//! removed whenever ship or prop is removed, or when effect/modifier is stopped.

pub(in crate::ss::svc) use data::CalcData;
pub use misc::SsAttrVal;
pub(in crate::ss::svc::svce_calc) use misc::SsLocType;
pub use mod_info::{ModInfo, ModOpInfo, ModSrcInfo, ModSrcValInfo};

mod data;
mod misc;
mod mod_info;
mod modifier;
mod registers;
mod svce_attr;
mod svce_buff;
mod svce_calc;
mod svce_modgen;
mod svce_modinfo;
