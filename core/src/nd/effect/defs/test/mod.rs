// Since all EVE effects which apply modifiers to a target contain some sort of modifier
// customization, it is inconvenient to test some lib features. To help with testing, the lib
// contains a few hardcoded effects which use passed modifiers as they are, and define how they are
// applied to target according to range to it.

pub(in crate::nd::effect) mod d10000000_mod_proj_simple;
pub(in crate::nd::effect) mod d10000001_mod_proj_normal1;
pub(in crate::nd::effect) mod d10000002_mod_proj_normal2;
pub(in crate::nd::effect) mod d10000003_buff_fleet_filtered;
