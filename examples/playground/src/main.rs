#![allow(warnings, unused)]
#![feature(core_intrinsics)]

use std::{intrinsics::black_box, path::PathBuf, sync::Arc, thread::sleep, time::Duration};

use chrono::Utc;
use itertools::Itertools;
use tracing_subscriber::prelude::*;

use rc::{ed::EveDataHandler, SolAddMode, SolItemState, SolModRack, SolValOptions, SolarSystem, Src, VERSION};

fn setup_logger() -> () {
    let time_format_full = time::macros::format_description!(
        version = 2,
        r"\[[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]\]"
    );
    // We always log warnings and higher to stdout
    let stdout_log = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout.with_max_level(tracing::Level::TRACE))
        .with_ansi(true)
        .with_timer(tracing_subscriber::fmt::time::UtcTime::new(time_format_full))
        .with_target(false)
        .pretty();
    tracing_subscriber::registry()
        .with(stdout_log)
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_default(tracing::Level::ERROR)
                .with_target("reefast_core", tracing::Level::TRACE)
                .with_target("reefast_dh_eve", tracing::Level::TRACE)
                .with_target("reefast_dh_adapted", tracing::Level::TRACE),
        )
        .init();
}

fn main() {
    setup_logger();
    let dh = Box::new(rdhe::PhbFileEdh::new("/home/dfx/Desktop/phobos_tq_en-us".into()));
    let ch = Box::new(rdha::RamJsonAdh::new(
        PathBuf::from("/home/dfx/Workspace/eve/reefast/examples/playground/cache/"),
        "tq".to_string(),
    ));
    // test_crusader(dh, ch);
    test_nphoon(dh, ch);
}

fn test_crusader(dh: Box<rdhe::PhbFileEdh>, ch: Box<rdha::RamJsonAdh>) {
    let skill_ids = get_skill_ids(&dh);
    let src = Src::new(dh, ch).unwrap();
    let mut sol_sys = SolarSystem::new(src);
    let fit = sol_sys.add_fit();
    let ship = sol_sys.set_fit_ship(fit.id, 11184, true).unwrap();
    for skill_id in skill_ids.iter() {
        sol_sys.add_skill(fit.id, skill_id.to_owned(), 5, true);
    }
    // RAH
    // sol_sys.add_module(
    //     fit.id,
    //     SolModRack::Low,
    //     SolOrdAddMode::Equip,
    //     4403,
    //     SolItemState::Active,
    //     None,
    //     None,
    // );

    // for (attr_id, val) in sol_sys.iter_item_attrs(&ship.id).unwrap().sorted_by_key(|v| v.0) {
    //     println!("{attr_id} {}", val.extra);
    // }

    let iterations = 1000000;
    tracing::error!("starting crusader test");
    let before = Utc::now();
    for _ in 0..iterations {
        let anp = sol_sys
            .add_module(
                fit.id,
                SolModRack::Low,
                SolAddMode::Equip,
                1306,
                SolItemState::Online,
                None,
                None,
            )
            .unwrap();
        black_box(sol_sys.iter_item_attrs(&ship.id).iter().for_each(drop));
        sol_sys.remove_item(&anp.id, rc::SolRmMode::Free);
        black_box(sol_sys.iter_item_attrs(&ship.id).iter().for_each(drop));
    }
    let after = Utc::now();
    tracing::error!("done with crusader test");
    let delta_seconds = (after - before).num_milliseconds() as f64 / 1000.0;
    let ips = iterations as f64 / delta_seconds;
    println!("{iterations} iterations done in {delta_seconds:.3} seconds, {ips:.2} iterations per second")
}

fn test_nphoon(dh: Box<rdhe::PhbFileEdh>, ch: Box<rdha::RamJsonAdh>) {
    let low_mod_ids = get_low_slot_mods(&dh);
    let skill_ids = get_skill_ids(&dh);
    let src = Src::new(dh, ch).unwrap();

    let mut sol_sys = SolarSystem::new(src);
    let fit = sol_sys.add_fit();

    // Character
    sol_sys.set_fit_character(fit.id, 1373, true).unwrap();

    // Skills
    for skill_id in skill_ids.iter() {
        sol_sys.add_skill(fit.id, skill_id.to_owned(), 5, true);
    }

    // Implants
    sol_sys.add_implant(fit.id, 13231, true).unwrap(); // TD-603
    sol_sys.add_implant(fit.id, 10228, true).unwrap(); // SM-703
    sol_sys.add_implant(fit.id, 24663, true).unwrap(); // Zor hyperlink
    sol_sys.add_implant(fit.id, 13244, true).unwrap(); // SS-903
    sol_sys.add_implant(fit.id, 13219, true).unwrap(); // LP-1003

    // Boosters
    sol_sys.add_booster(fit.id, 28674, true).unwrap(); // Synth drop
    sol_sys.add_booster(fit.id, 28672, true).unwrap(); // Synth crash
    sol_sys.add_booster(fit.id, 45999, true).unwrap(); // Pyro 2

    // Ship
    sol_sys.set_fit_ship(fit.id, 32311, true).unwrap(); // NTyphoon

    // High slots
    for _ in 0..2 {
        sol_sys
            .add_module(
                fit.id,
                SolModRack::High,
                SolAddMode::Equip,
                2929,
                SolItemState::Overload,
                None,
                Some(12779),
            )
            .unwrap(); // T2 800mm with hail
    }
    for _ in 0..2 {
        sol_sys
            .add_module(
                fit.id,
                SolModRack::High,
                SolAddMode::Equip,
                2420,
                SolItemState::Overload,
                None,
                Some(2811),
            )
            .unwrap(); // T2 torps with thermal rages
    }

    // Mid slots
    sol_sys
        .add_module(
            fit.id,
            SolModRack::Mid,
            SolAddMode::Equip,
            5945,
            SolItemState::Active,
            None,
            None,
        )
        .unwrap(); // Enduring 500MN
    sol_sys
        .add_module(
            fit.id,
            SolModRack::Mid,
            SolAddMode::Equip,
            2024,
            SolItemState::Active,
            None,
            Some(32014),
        )
        .unwrap(); // T2 med cap booster with navy 800
    sol_sys
        .add_module(
            fit.id,
            SolModRack::Mid,
            SolAddMode::Equip,
            2301,
            SolItemState::Active,
            None,
            None,
        )
        .unwrap(); // T2 EM hardener
    sol_sys
        .add_module(
            fit.id,
            SolModRack::Mid,
            SolAddMode::Equip,
            448,
            SolItemState::Active,
            None,
            None,
        )
        .unwrap(); // T2 scram
    sol_sys
        .add_module(
            fit.id,
            SolModRack::Mid,
            SolAddMode::Equip,
            2281,
            SolItemState::Active,
            None,
            None,
        )
        .unwrap(); // T2 invuln

    // Low slots
    sol_sys
        .add_module(
            fit.id,
            SolModRack::Low,
            SolAddMode::Equip,
            2048,
            SolItemState::Online,
            None,
            None,
        )
        .unwrap(); // T2 DC
    for _ in 0..2 {
        sol_sys
            .add_module(
                fit.id,
                SolModRack::Low,
                SolAddMode::Equip,
                519,
                SolItemState::Online,
                None,
                None,
            )
            .unwrap(); // T2 gyrostab
    }
    for _ in 0..2 {
        sol_sys
            .add_module(
                fit.id,
                SolModRack::Low,
                SolAddMode::Equip,
                22291,
                SolItemState::Online,
                None,
                None,
            )
            .unwrap(); // T2 BCS
    }
    for _ in 0..1 {
        sol_sys
            .add_module(
                fit.id,
                SolModRack::Low,
                SolAddMode::Equip,
                4405,
                SolItemState::Online,
                None,
                None,
            )
            .unwrap(); // T2 DDA
    }

    // Rigs
    sol_sys.add_rig(fit.id, 26082, true).unwrap(); // T1 therm rig
    for _ in 0..2 {
        sol_sys.add_rig(fit.id, 26088, true).unwrap(); // T1 CDFE
    }

    // Drones
    for _ in 0..5 {
        sol_sys.add_drone(fit.id, 2446, SolItemState::Active, None).unwrap(); // T2 ogre
    }
    for _ in 0..3 {
        sol_sys.add_drone(fit.id, 2446, SolItemState::Offline, None).unwrap(); // T2 ogre
    }

    let val_options = SolValOptions::new_enabled();

    let iterations = 1000;
    tracing::error!(
        "starting nphoon test, trying {} modules per iteration",
        low_mod_ids.len()
    );
    let before = Utc::now();
    for _ in 0..iterations {
        for &low_mod_id in low_mod_ids.iter() {
            let info = sol_sys
                .add_module(
                    fit.id,
                    SolModRack::Low,
                    SolAddMode::Equip,
                    low_mod_id,
                    SolItemState::Online,
                    None,
                    None,
                )
                .unwrap();
            let r = sol_sys.validate_fit_fast(&fit.id, val_options).unwrap();
            // if !r {
            //     println!("{low_mod_id}");
            // }
            sol_sys.remove_item(&info.id, rc::SolRmMode::Free).unwrap();
        }
        // break
    }
    let after = Utc::now();
    tracing::error!("done with nphoon test");
    let delta_seconds = (after - before).num_milliseconds() as f64 / 1000.0;
    let ips = iterations as f64 / delta_seconds;
    println!("{iterations} iterations done in {delta_seconds:.3} seconds, {ips:.2} iterations per second")
}

fn get_skill_ids(dh: &Box<rdhe::PhbFileEdh>) -> Vec<rc::EItemId> {
    let grp_ids = dh
        .get_item_groups()
        .unwrap()
        .data
        .iter()
        .filter(|v| v.category_id == 16)
        .map(|v| v.id)
        .collect_vec();
    let skill_ids = dh
        .get_items()
        .unwrap()
        .data
        .iter()
        .filter(|v| grp_ids.contains(&v.group_id))
        .map(|v| v.id)
        .collect_vec();
    skill_ids
}

fn get_low_slot_mods(dh: &Box<rdhe::PhbFileEdh>) -> Vec<rc::EItemId> {
    let grp_ids = dh
        .get_item_groups()
        .unwrap()
        .data
        .iter()
        .filter(|v| v.category_id == 7)
        .map(|v| v.id)
        .collect_vec();
    let low_ids = dh
        .get_item_effects()
        .unwrap()
        .data
        .iter()
        .filter(|v| v.effect_id == 11)
        .map(|v| v.item_id)
        .collect_vec();
    let item_ids = dh
        .get_items()
        .unwrap()
        .data
        .iter()
        .filter(|v| low_ids.contains(&v.id) && grp_ids.contains(&v.group_id))
        .map(|v| v.id)
        .collect_vec();
    item_ids
}
