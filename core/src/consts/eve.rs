pub enum EffectCategory {
    Passive,
    Active,
    Target,
    Area,
    Online,
    Overheat,
    Dungeon,
    System
}

pub enum ModDomain {
    Ship,
    Char,
    Item
}

pub enum ModOperator {
    PreAssign,
    PreMul,
    PreDiv,
    Add,
    Sub,
    PostMul,
    PostDiv,
    PostPercent,
    PostAssign
}
