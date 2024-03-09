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
//! tracked otherwise. Information here isn't added when items are added or effects are started, but
//! only when calculator realizes that there is a dependency, which usually happens during dependent
//! attribute calculation. Information is removed whenever one of items is removed, or when modifier
//! which establishes this dependency is stopped. Attribute value limits and custom propulsion
//! module modifier use this register;
//! - Revision register - keeps track of custom modifiers which depend on various events not related
//! to attribute changes, and whenever significant events happen, forces recalculation of attribute
//! values it modifies. Custom ancillary armor repairer modifier uses this register.
//!
//! Calculation service methods tie everything together, and provide a few methods to manipulate
//! relations and fetch attribute data.

pub(in crate::ss::svc) use data::CalcData;
pub use misc::{ModificationInfo, SsAttrVal};

mod data;
mod misc;
mod modifier;
mod registers;
mod svce_attr;
mod svce_buff;
mod svce_calc;
