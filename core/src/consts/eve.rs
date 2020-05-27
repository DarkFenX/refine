use crate::defines::ReeInt;

#[derive(Debug)]
pub enum EffectCategory {
    Passive,
    Active,
    Target,
    Area,
    Online,
    Overheat,
    Dungeon,
    System,
}

#[derive(Debug)]
pub enum ModAfeeFilter {
    Direct(ModDomain),
    Loc(ModDomain),
    LogGrp(ModDomain, ReeInt),
    LocSrq(ModDomain, ReeInt),
    OwnSrq(ModDomain, ReeInt),
}

#[derive(Debug)]
pub enum ModDomain {
    Ship,
    Char,
    Item,
}

#[derive(Debug)]
pub enum ModAggrMode {
    Stack,
    Min(ReeInt),
    Max(ReeInt),
}

#[derive(Debug)]
pub enum ModOperation {
    PreAssign,
    PreMul,
    PreDiv,
    Add,
    Sub,
    PostMul,
    PostDiv,
    PostPercent,
    PostAssign,
}
