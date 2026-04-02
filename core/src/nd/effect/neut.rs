use crate::{
    nd::{NEffectGeneralOutputGetter, NEffectProjOpcSpec},
    num::Value,
    rd::RAttrConsts,
    ud::UItem,
};

pub(crate) struct NEffectNeut {
    pub(crate) kind: NEffectNeutKind,
    pub(crate) checker: Option<NEffectNeutChecker>,
    pub(crate) ospec: NEffectProjOpcSpec<NEffectGeneralOutputGetter>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Item/effect type
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(crate) enum NEffectNeutKind {
    Module,
    Minion,
    Bomb,
    SideEffect,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Base item checker
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(crate) enum NEffectNeutChecker {
    Bomb,
}
impl NEffectNeutChecker {
    pub(crate) fn check(&self, u_item: &UItem, attr_consts: &RAttrConsts) -> bool {
        match self {
            Self::Bomb => check_bomb(u_item, attr_consts),
        }
    }
}

fn check_bomb(u_item: &UItem, attr_consts: &RAttrConsts) -> bool {
    let attr_rid = match attr_consts.energy_neut_amount {
        Some(attr_rid) => attr_rid,
        None => return false,
    };
    match u_item.get_attr(&attr_rid) {
        Some(value) => value > Value::FLOAT_TOLERANCE,
        None => false,
    }
}
