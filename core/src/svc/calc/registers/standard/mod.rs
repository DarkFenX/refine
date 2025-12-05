//! Standard register is pretty much heart of attribute calculations. It collects all the data
//! related to standard attribute dependencies (which are 95%+ of all attribute dependencies) to
//! answer two ultimate questions:
//!
//! - which modifiers affect value of an attribute on an item;
//! - which attribute items on which values depend on a modifier.
//!
//! To answer those questions properly, one has to define contract for each modifier sub-kind (e.g.
//! projected system modifier). The contract defines which context modifier types does it register
//! for which affectee filters. The same principle has to be maintained when registering/
//! unregistering modifiers, and when getting affectees.
//!
//! For example, fleet buffs keep context modifiers only for those buffs which pass item list
//! filter, and put them into:
//!
//! - direct modifications: direct container with item context;
//! - location modifications: location container with fit+item context;
//! - location-group modifications: location-group container with fit+item context;
//! - location-skillreq modifications: location-skillreq container with fit+item context;
//!
//! Affectee getter is implemented so that all of those configurations are properly supported for
//! fleet buff modifiers. Also, thanks to checking item list filter upon buff application, whenever
//! buff value changes, affectee getter does not need to check if fit's ship passes the item list
//! filter.

pub(in crate::svc::calc) use data::StandardRegister;

mod affectee_fill;
mod affectee_reg;
mod data;
mod debug;
mod iter_locs_pot;
mod modifier;
