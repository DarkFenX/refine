use std::sync::Arc;

use crate::rd::{RAbil, RAttr, RBuff, REffect, RItem, RItemList, RMuta};

pub(crate) type REffectKey = usize;
// Arcs to entities returned by handler
pub(crate) type RcAbil = Arc<RAbil>;
pub(crate) type RcAttr = Arc<RAttr>;
pub(crate) type RcBuff = Arc<RBuff>;
pub(crate) type RcEffect = Arc<REffect>;
pub(crate) type RcItem = Arc<RItem>;
pub(crate) type RcItemList = Arc<RItemList>;
pub(crate) type RcMuta = Arc<RMuta>;
