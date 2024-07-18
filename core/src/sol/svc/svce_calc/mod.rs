//! Service extension methods which handle attribute calculation.
//!
//! Calculator consists of a set of registers which serve two main purposes: 1) when an attribute is
//! being calculated - they tell what modifies it and how, and 2) when an attribute's value changes,
//! they tell which attributes rely on this one, to force their recalculation too. But since
//! relations between attributes are complex, there are multiple registers which handle it.
//!
//! - Standard register - primary register which provides data on modifiers and items affected by
//! them;
//! - Buff register - collects data about buff modifiers which rely on on-item attributes to define
//! buff type;
//! - Dependency register - tracks direct dependencies between attribute values, which cannot be
//! tracked otherwise;
//! - Revision register - keeps track of custom modifiers which depend on various events not related
//! to attribute changes, and whenever significant events happen, forces recalculation of attribute
//! values it modifies. Custom ancillary armor repairer modifier uses this register.
//! - Projection register - keeps info about projection range between various projectable items.
//!
//! Next, there are a few scenarios on how those registers are used:
//!
//! - Effects with regular modifiers: those use standard register to provide info for both
//! directions: finding modifiers which affect an attr on an item, and finding items which are
//! affected by a modifier.
//! - Attribute value caps/limits: they are using dependency register, and relation between a
//! limiting attribute and a limited attribute is registered during calculation of the limited
//! attribute. Relation is removed only when item is unloaded;
//! - Custom ancillary repairer modifier: uses revision register to clear rep amount attribute
//! whenever it loads/unloads paste as its charge;
//! - Custom AB/MWD modifier: uses dependency register to establish relationship between ship speed,
//! ship mass, prop speed boost, and prop thrust during modifier calculation. This relationship is
//! removed whenever ship or prop is removed, or when effect/modifier is stopped.

use accum::{SolAttrValInfo, SolModAccumFast, SolModAccumInfo};
pub(in crate::sol::svc) use data::SolSvcCalcData;
pub use misc::SolAttrVal;
use misc::{SolAttrSpec, SolLocationKind, SolModification, SolModificationKey};
pub use mod_info::{SolAffectorInfo, SolModificationInfo, SolOpInfo};
use modifier::{
    debug, extend_with_custom_mods, SolAffecteeFilter, SolAggrMode, SolContext, SolCtxModifier, SolDomain,
    SolModifierKind, SolOp, SolRawModifier,
};

mod accum;
mod data;
mod misc;
mod mod_info;
mod modifier;
mod registers;
mod scve_attr_calc;
mod svce_maintain;
mod svce_modgen;
