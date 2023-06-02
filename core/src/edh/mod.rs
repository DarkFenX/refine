//! EVE data handling interface.
//!
//! EVE data handlers provide a way for the library to fetch the EVE data it needs from external
//! sources.
//!
//! # Assumptions about data
//! Reefast verifies data integrity and makes several assumptions about it. If those assumptions are
//! broken, offending entries will be adjusted or removed during conversion of the data into
//! [EVE reefast types](crate::adt).
//!
//! ### Primary keys
//! Almost every data entry provided by a [`edh::EveDataHandler`](crate::edh::EveDataHandler)
//! implementation has a private PK getter defined. For every vector there can be maximum one entry
//! with the same PK. When there are multiple entries with the same PK, only first seen entry is
//! kept, with the rest getting removed altogether.
//!
//! ### Item's default effect
//! Every item can have a maximum of one default effect. For any given item,
//! [`edt::ItemEffect`](crate::edt::EItemEffect) which is marked as default will be marked as
//! non-default past first seen entry.
//!
//! ### Ability-to-effect data transfer
//! Reefast assumes that effects which power fighter abilities are used only by those abilities and
//! nothing else. During EVE reefast type generation, this assumption allows to move all the fighter
//! ability data to data structures related to effects.
//!
//! - Data defined on [`edt::FighterAbil`](crate::edt::EFighterAbil) is moved to
//!   [`ert::Effect`](crate::adt::AEffect).
//! - Data defined on [`edt::ItemFighterAbil`](crate::edt::EItemFighterAbil) is moved to
//!   [`ert::ItemEffData`](crate::adt::AItemEffData), which describe effect properties specific to
//!   parent [`ert::Item`](crate::adt::AItem).
//!
//! Since multiple abilities can map to the same effect, collisions are possible. In case of
//! collisions, data from colliding abilities is compared. If there are any mismatches, warnings are
//! logged, and data is not transferred to an effect.

pub use cont::Container;
pub use handler::EveDataHandler;
pub use result::Result;

mod cont;
mod handler;
mod result;
