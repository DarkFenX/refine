//! Service extension methods which handle attribute calculation.
//!
//! Calculator consists of a set of registers which serve two main purposes: 1) when an attribute is
//! being calculated - they tell what modifies it and how, and 2) when an attribute's value changes,
//! they tell which attributes rely on this one, to force their recalculation too. But since
//! relations between attributes are complex, there are multiple registers which handle it.
//!
//! - Modifier register - primary register which provides data on which modifiers affects specific
//! attr on a specific item;
//! - Affectee register - primary register which provides info on which items are modified by a
//! modifier;
//! - Buff register - collects data about buff modifiers which rely on on-item attributes to define
//! buff type;
//! - Dependency register - tracks direct dependencies between attribute values, which cannot be
//! tracked otherwise;
//! - Revision register - keeps track of custom modifiers which depend on various events not related
//! to attribute changes, and whenever significant events happen, forces recalculation of attribute
//! values it modifies. Custom ancillary armor repairer modifier uses this register.
//!
//! Next, there are a few scenarios on how those registers are used:
//!
//! - Fit-local dogma modifiers: those use modifier register and affectee register to provide info
//! for both directions: finding modifiers which affect an attr on an item, and finding items which
//! are affected by a modifier;
//! - Fit-wide modifiers: work similarly to fit-local modifiers, but they can use buffs which can
//! potentially affect everything directly, e.g. abyssal weather affecting drone stats, with logic
//! for that implemented in modifier register and affectee register;
//! - System-wide, projected and targeted dogma modifiers: same as fit-wide modifiers, but span
//! outside of fit, with controlling logic spread between the two primary registers and the service
//! itself;
//! - Fleet modifiers (or buffs in general): processed like the rest of modifiers, with extra logic
//! for fleets (implemented in the two primary registers), and using buff register to make sure
//! that buff types are properly switched when on-item attribute which controls them changes;
//! - Attribute value caps/limits: they are using dependency register, and relation between a
//! limiting attribute and a limited attribute is registered during calculation of the limited
//! attribute. Relation is removed only when item is unloaded;
//! - Custom ancillary repairer modifier: uses revision register to clear rep amount attribute
//! whenever it loads/unloads paste as its charge;
//! - Custom AB/MWD modifier: uses dependency register to establish relationship between ship speed,
//! ship mass, prop speed boost, and prop thrust during modifier calculation. This relationship is
//! removed whenever ship or prop is removed, or when effect/modifier is stopped.

use attr::SolAttrValues;
pub(in crate::sol::svc) use data::SolSvcCalcData;
pub use misc::SolAttrVal;
use misc::{SolLocType, SolModKey, SolModification};
pub use mod_info::{SolModInfo, SolModOpInfo, SolModSrcInfo, SolModSrcValInfo};
use modifier::{
    debug, extend_with_custom_mods, SolAffecteeFilter, SolAttrMod, SolModAggrMode, SolModDomain, SolModOp, SolModType,
};
use registers::{SolAttrSpec, SolFleetUpdates};

mod attr;
mod data;
mod misc;
mod mod_info;
mod modifier;
mod registers;
mod svce_attr;
mod svce_calc;
mod svce_modgen;
mod svce_modinfo;
