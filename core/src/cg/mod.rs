mod data;

use crate::dh::{self, DataHandler};
use crate::util::Named;

struct Data {
    items: Vec<dh::Item>,
    item_groups: Vec<dh::ItemGroup>,
    attrs: Vec<dh::Attr>,
    item_attrs: Vec<dh::ItemAttr>,
    effects: Vec<dh::Effect>,
    item_effects: Vec<dh::ItemEffect>,
    fighter_abils: Vec<dh::FighterAbil>,
    item_fighter_abils: Vec<dh::ItemFighterAbil>,
    buffs: Vec<dh::Buff>,
    item_skill_reqs: Vec<dh::ItemSkillReq>,
    muta_item_convs: Vec<dh::MutaItemConv>,
    muta_attr_mods: Vec<dh::MutaAttrMod>,
}
impl Data {
    fn new() -> Data {
        Data {
            items: Vec::new(),
            item_groups: Vec::new(),
            attrs: Vec::new(),
            item_attrs: Vec::new(),
            effects: Vec::new(),
            item_effects: Vec::new(),
            fighter_abils: Vec::new(),
            item_fighter_abils: Vec::new(),
            buffs: Vec::new(),
            item_skill_reqs: Vec::new(),
            muta_item_convs: Vec::new(),
            muta_attr_mods: Vec::new(),
        }
    }
}

fn fill_data<S, F, T>(handler: &S, func: F, vec: &mut Vec<T>)
where
    S: ?Sized + DataHandler,
    F: Fn(&S) -> dh::Result<dh::Container<T>>,
    T: Named,
{
    println!("processing {}", T::get_name());
    match func(handler) {
        Ok(cont) => vec.extend(cont.data),
        Err(e) => {}
    }
}

pub fn generate_cache(data_handler: &dyn DataHandler) {
    let mut data = Data::new();
    fill_data(data_handler, DataHandler::get_items, &mut data.items);
    fill_data(data_handler, DataHandler::get_item_groups, &mut data.item_groups);
    fill_data(data_handler, DataHandler::get_attrs, &mut data.attrs);
    fill_data(data_handler, DataHandler::get_item_attrs, &mut data.item_attrs);
    fill_data(data_handler, DataHandler::get_effects, &mut data.effects);
    fill_data(data_handler, DataHandler::get_item_effects, &mut data.item_effects);
    fill_data(data_handler, DataHandler::get_fighter_abils, &mut data.fighter_abils);
    fill_data(
        data_handler,
        DataHandler::get_item_fighter_abils,
        &mut data.item_fighter_abils,
    );
    fill_data(data_handler, DataHandler::get_buffs, &mut data.buffs);
    fill_data(
        data_handler,
        DataHandler::get_item_skill_reqs,
        &mut data.item_skill_reqs,
    );
    fill_data(
        data_handler,
        DataHandler::get_muta_item_convs,
        &mut data.muta_item_convs,
    );
    fill_data(data_handler, DataHandler::get_muta_attr_mods, &mut data.muta_attr_mods);
}
