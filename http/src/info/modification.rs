#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HModificationInfo {
    pub src_item_id: rc::SsItemId,
    pub src_attr_id: Option<rc::EAttrId>,
    pub val: rc::AttrVal,
    pub op: HModOp,
    pub penalized: bool,
    pub aggr_mode: HModAggrMode,
}
impl HModificationInfo {
    fn new(
        src_item_id: rc::SsItemId,
        src_attr_id: Option<rc::EAttrId>,
        val: rc::AttrVal,
        op: HModOp,
        penalized: bool,
        aggr_mode: HModAggrMode,
    ) -> Self {
        Self {
            src_item_id,
            src_attr_id,
            op,
            penalized,
            aggr_mode,
            val,
        }
    }
}
impl From<&rc::ModificationInfo> for HModificationInfo {
    fn from(core_mod_info: &rc::ModificationInfo) -> Self {
        Self::new(
            core_mod_info.src_item_id,
            core_mod_info.src_attr_id,
            core_mod_info.val,
            (&core_mod_info.op).into(),
            core_mod_info.penalized,
            (&core_mod_info.aggr_mode).into(),
        )
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HModOp {
    PreAssign,
    PreMul,
    PreDiv,
    Add,
    Sub,
    PostMul,
    PostMulImmune,
    PostDiv,
    PostPerc,
    PostAssign,
    ExtraMul,
}
impl From<&rc::ModOp> for HModOp {
    fn from(core_op: &rc::ModOp) -> Self {
        match core_op {
            rc::ModOp::PreAssign => Self::PreAssign,
            rc::ModOp::PreMul => Self::PreMul,
            rc::ModOp::PreDiv => Self::PreDiv,
            rc::ModOp::Add => Self::Add,
            rc::ModOp::Sub => Self::Sub,
            rc::ModOp::PostMul => Self::PostMul,
            rc::ModOp::PostMulImmune => Self::PostMulImmune,
            rc::ModOp::PostDiv => Self::PostDiv,
            rc::ModOp::PostPerc => Self::PostPerc,
            rc::ModOp::PostAssign => Self::PostAssign,
            rc::ModOp::ExtraMul => Self::ExtraMul,
        }
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HModAggrMode {
    Stack,
    Min(rc::EBuffId),
    Max(rc::EBuffId),
}
impl From<&rc::ModAggrMode> for HModAggrMode {
    fn from(core_aggr_mode: &rc::ModAggrMode) -> Self {
        match core_aggr_mode {
            rc::ModAggrMode::Stack => Self::Stack,
            rc::ModAggrMode::Min(key) => Self::Min(*key),
            rc::ModAggrMode::Max(key) => Self::Max(*key),
        }
    }
}
