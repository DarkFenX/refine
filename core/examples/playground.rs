use std::collections::HashMap;
use std::path::PathBuf;

use chrono;

use reefast::consts::{EveEffectCategory, EveModDomain, EveModOperator};
use reefast::ct::{Attr, Effect, Item, ItemModifier};
use reefast::dh::{self, DataHandler};
use reefast::dh_impls::phobos;

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%H:%M:%S%.3f]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn print_data<T>(name: &'static str, data: dh::Result<dh::Container<T>>) {
    match data {
        Ok(r) => {
            println!("{}: {} returned, {} failed", name, r.data.len(), r.errors.len());
            for e in r.errors.iter() {
                println!("  error: {}", e)
            }
        }
        Err(e) => println!("{} failed: {}", name, e),
    }
}

fn main() {
    setup_logger().unwrap();
    let dh = phobos::PhobosHandler::new(PathBuf::from("/home/dfx/Desktop/phobos_tq_en-us"));
    println!("using {:?}", dh);
    print_data("items", dh.get_items());
    print_data("item groups", dh.get_item_groups());
    print_data("attributes", dh.get_attrs());
    print_data("item attributes", dh.get_item_attrs());
    print_data("effects", dh.get_effects());
    print_data("item effects", dh.get_item_effects());
    print_data("mutaplasmid item conversions", dh.get_muta_item_convs());
    print_data("mutaplasmid attr modifications", dh.get_muta_attr_mods());
    print_data("buffs", dh.get_buffs());
    print_data("fighter abilities", dh.get_fighter_abils());
    print_data("item fighter abilities", dh.get_item_fighter_abils());
    print_data("item skill requirements", dh.get_item_skill_reqs());
    match dh.get_version() {
        Ok(r) => println!("data version: {}", r),
        Err(e) => println!("version failed: {}", e),
    }
}
