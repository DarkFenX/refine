use crate::{
    AddedFitIdResp, AddedFleetIdResp, AddedItemIdsResp, ChangedItemIdsResp, FitId, FitInfo, FleetId, FleetInfo, ItemId,
    ItemInfo, ItemTypeId, SolInfo,
    stats::{FitStatsResp, FleetStatsResp, ItemStatsResp, SolStatsResp},
    val::{FitValResult, SolValResult},
};

#[derive(Clone)]
pub enum CmdResp {
    AddedFleetId(AddedFleetIdResp),
    AddedFitId(AddedFitIdResp),
    AddedItemIds(AddedItemIdsResp),
    ChangedItemIds(ChangedItemIdsResp),
    // Infos are large (e.g. fit is more than 500 bytes)
    SolInfo(Box<SolInfo>),
    FleetInfo(Box<FleetInfo>),
    FitInfo(Box<FitInfo>),
    ItemInfo(Box<ItemInfo>),
    // Stats are large as well (e.g. fit is almost 1.5 kb)
    SolStats(Box<SolStatsResp>),
    FleetStats(Box<FleetStatsResp>),
    FitStats(Box<FitStatsResp>),
    ItemStats(Box<ItemStatsResp>),
    // Validation results are large
    SolVal(Box<SolValResult>),
    FitVal(Box<FitValResult>),
    TryItems(Vec<ItemTypeId>),
    // Misc
    NoData,
}
impl CmdResp {
    pub fn get_fleet_id(&self) -> Option<FleetId> {
        match self {
            Self::AddedFleetId(resp) => Some(resp.fleet_id),
            _ => None,
        }
    }
    pub fn get_fit_id(&self) -> Option<FitId> {
        match self {
            Self::AddedFitId(resp) => Some(resp.fit_id),
            _ => None,
        }
    }
    pub fn get_item_id(&self) -> Option<ItemId> {
        match self {
            Self::AddedItemIds(resp) => Some(resp.item_id),
            _ => None,
        }
    }
    pub fn get_charge_item_id(&self) -> Option<ItemId> {
        match self {
            Self::AddedItemIds(resp) => resp.charge_item_id,
            Self::ChangedItemIds(resp) => resp.charge_item_id,
            _ => None,
        }
    }
    pub fn get_sol_info(&self) -> Option<&SolInfo> {
        match self {
            Self::SolInfo(info) => Some(info),
            _ => None,
        }
    }
    pub fn get_fleet_info(&self) -> Option<&FleetInfo> {
        match self {
            Self::FleetInfo(info) => Some(info),
            _ => None,
        }
    }
    pub fn get_fit_info(&self) -> Option<&FitInfo> {
        match self {
            Self::FitInfo(info) => Some(info),
            _ => None,
        }
    }
    pub fn get_item_info(&self) -> Option<&ItemInfo> {
        match self {
            Self::ItemInfo(info) => Some(info),
            _ => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<()> for CmdResp {
    fn from(_: ()) -> Self {
        CmdResp::NoData
    }
}
impl From<AddedFitIdResp> for CmdResp {
    fn from(resp: AddedFitIdResp) -> Self {
        CmdResp::AddedFitId(resp)
    }
}
impl From<AddedFleetIdResp> for CmdResp {
    fn from(resp: AddedFleetIdResp) -> Self {
        CmdResp::AddedFleetId(resp)
    }
}
impl From<AddedItemIdsResp> for CmdResp {
    fn from(resp: AddedItemIdsResp) -> Self {
        CmdResp::AddedItemIds(resp)
    }
}
impl From<ChangedItemIdsResp> for CmdResp {
    fn from(resp: ChangedItemIdsResp) -> Self {
        CmdResp::ChangedItemIds(resp)
    }
}
impl From<SolInfo> for CmdResp {
    fn from(info: SolInfo) -> Self {
        CmdResp::SolInfo(Box::new(info))
    }
}
impl From<FleetInfo> for CmdResp {
    fn from(info: FleetInfo) -> Self {
        CmdResp::FleetInfo(Box::new(info))
    }
}
impl From<FitInfo> for CmdResp {
    fn from(info: FitInfo) -> Self {
        CmdResp::FitInfo(Box::new(info))
    }
}
impl From<ItemInfo> for CmdResp {
    fn from(info: ItemInfo) -> Self {
        CmdResp::ItemInfo(Box::new(info))
    }
}
impl From<SolStatsResp> for CmdResp {
    fn from(stats: SolStatsResp) -> Self {
        CmdResp::SolStats(Box::new(stats))
    }
}
impl From<FleetStatsResp> for CmdResp {
    fn from(stats: FleetStatsResp) -> Self {
        CmdResp::FleetStats(Box::new(stats))
    }
}
impl From<FitStatsResp> for CmdResp {
    fn from(stats: FitStatsResp) -> Self {
        CmdResp::FitStats(Box::new(stats))
    }
}
impl From<ItemStatsResp> for CmdResp {
    fn from(stats: ItemStatsResp) -> Self {
        CmdResp::ItemStats(Box::new(stats))
    }
}
impl From<SolValResult> for CmdResp {
    fn from(val_result: SolValResult) -> Self {
        CmdResp::SolVal(Box::new(val_result))
    }
}
impl From<FitValResult> for CmdResp {
    fn from(val_result: FitValResult) -> Self {
        CmdResp::FitVal(Box::new(val_result))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::ser::{Serialize, SerializeStruct, Serializer};

    use super::*;

    impl Serialize for CmdResp {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AddedFleetId(inner) => inner.serialize(serializer),
                Self::AddedFitId(inner) => inner.serialize(serializer),
                Self::AddedItemIds(inner) => inner.serialize(serializer),
                Self::ChangedItemIds(inner) => inner.serialize(serializer),
                Self::SolInfo(inner) => inner.serialize(serializer),
                Self::FleetInfo(inner) => inner.serialize(serializer),
                Self::FitInfo(inner) => inner.serialize(serializer),
                Self::ItemInfo(inner) => inner.serialize(serializer),
                Self::SolStats(inner) => inner.serialize(serializer),
                Self::FleetStats(inner) => inner.serialize(serializer),
                Self::FitStats(inner) => inner.serialize(serializer),
                Self::ItemStats(inner) => inner.serialize(serializer),
                Self::SolVal(inner) => inner.serialize(serializer),
                Self::FitVal(inner) => inner.serialize(serializer),
                Self::TryItems(inner) => inner.serialize(serializer),
                // Command response has custom serialization implementation just for the NoData
                // variant to be serialized as {} in JSON instead of null
                Self::NoData => {
                    let empty = serializer.serialize_struct("Empty", 0)?;
                    empty.end()
                }
            }
        }
    }
}
