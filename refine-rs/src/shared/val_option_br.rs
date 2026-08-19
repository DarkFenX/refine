use crate::{CmdResps, ItemId, ItemIdBr, err::BrResolveError, val::ValOptions};

pub(crate) fn val_options_br_resolve(
    options: ValOptions<ItemIdBr>,
    cmd_resps: &CmdResps,
) -> Result<ValOptions<ItemId>, BrResolveError> {
    options.try_map_ids(|item_id| cmd_resps.resolve_item_id(item_id))
}
