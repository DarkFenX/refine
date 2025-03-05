#![allow(warnings, unused)]
#![feature(core_intrinsics)]

use std::{intrinsics::black_box, path::PathBuf, sync::Arc, thread::sleep, time::Duration};

use chrono::Utc;
use itertools::Itertools;
use tracing_subscriber::prelude::*;

use rc::{
    SolAddMode, SolMinionState, SolModRack, SolModuleState, SolValOptions, SolarSystem, Src, VERSION,
    ad::{AItemKind, AState, AdaptedDataHandler},
    ed::EveDataHandler,
};

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
    //test_crusader(dh, ch);
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
    //     SolModuleState::Active,
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
                SolModuleState::Online,
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
    let mut market = Market::new(&dh);
    let skill_ids = get_skill_ids(&dh);
    let src = Src::new(dh, ch).unwrap();
    market.fill(&src);

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
                SolModuleState::Overload,
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
                SolModuleState::Overload,
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
            SolModuleState::Active,
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
            SolModuleState::Active,
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
            SolModuleState::Active,
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
            SolModuleState::Active,
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
            SolModuleState::Active,
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
            SolModuleState::Online,
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
                SolModuleState::Online,
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
                SolModuleState::Online,
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
                SolModuleState::Online,
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
        sol_sys.add_drone(fit.id, 2446, SolMinionState::Engaging, None).unwrap(); // T2 ogre
    }
    for _ in 0..3 {
        sol_sys.add_drone(fit.id, 2446, SolMinionState::InBay, None).unwrap(); // T2 ogre
    }

    let val_options = SolValOptions::new_all_enabled();

    let iterations = 1000;
    tracing::error!(
        "starting nphoon test, trying {} items per iteration",
        market.boosters.len()
            + market.drones.len()
            + market.fighters.len()
            + market.implants.len()
            + market.modules_high.len()
            + market.modules_mid.len()
            + market.modules_low.len()
            + market.rigs.len()
            + market.subsystems.len()
    );
    let before = Utc::now();
    for _ in 0..iterations {
        for &type_id in market.boosters.iter() {
            let info = sol_sys.add_booster(fit.id, type_id, true).unwrap();
            sol_sys.validate_fit_fast(&fit.id, val_options).unwrap();
            sol_sys.remove_booster(&info.id).unwrap();
        }
        for &type_id in market.drones.iter() {
            let info = sol_sys.add_drone(fit.id, type_id, SolMinionState::InBay, None).unwrap();
            sol_sys.validate_fit_fast(&fit.id, val_options).unwrap();
            sol_sys.remove_drone(&info.id).unwrap();
        }
        for &type_id in market.fighters.iter() {
            let info = sol_sys.add_fighter(fit.id, type_id, SolMinionState::InBay).unwrap();
            sol_sys.validate_fit_fast(&fit.id, val_options).unwrap();
            sol_sys.remove_fighter(&info.id).unwrap();
        }
        for &type_id in market.implants.iter() {
            let info = sol_sys.add_implant(fit.id, type_id, true).unwrap();
            sol_sys.validate_fit_fast(&fit.id, val_options).unwrap();
            sol_sys.remove_implant(&info.id).unwrap();
        }
        for &(type_id, max_state) in market.modules_high.iter() {
            let info = sol_sys
                .add_module(
                    fit.id,
                    SolModRack::High,
                    SolAddMode::Equip,
                    type_id,
                    max_state,
                    None,
                    None,
                )
                .unwrap();
            sol_sys.validate_fit_fast(&fit.id, val_options).unwrap();
            sol_sys.remove_item(&info.id, rc::SolRmMode::Free).unwrap();
        }
        for &(type_id, max_state) in market.modules_mid.iter() {
            let info = sol_sys
                .add_module(
                    fit.id,
                    SolModRack::Mid,
                    SolAddMode::Equip,
                    type_id,
                    max_state,
                    None,
                    None,
                )
                .unwrap();
            sol_sys.validate_fit_fast(&fit.id, val_options).unwrap();
            sol_sys.remove_item(&info.id, rc::SolRmMode::Free).unwrap();
        }
        for &(type_id, max_state) in market.modules_low.iter() {
            let info = sol_sys
                .add_module(
                    fit.id,
                    SolModRack::Low,
                    SolAddMode::Equip,
                    type_id,
                    max_state,
                    None,
                    None,
                )
                .unwrap();
            sol_sys.validate_fit_fast(&fit.id, val_options).unwrap();
            sol_sys.remove_item(&info.id, rc::SolRmMode::Free).unwrap();
        }
        for &type_id in market.rigs.iter() {
            let info = sol_sys.add_rig(fit.id, type_id, true).unwrap();
            sol_sys.validate_fit_fast(&fit.id, val_options).unwrap();
            sol_sys.remove_rig(&info.id).unwrap();
        }
        for &type_id in market.subsystems.iter() {
            let info = sol_sys.add_subsystem(fit.id, type_id, true).unwrap();
            sol_sys.validate_fit_fast(&fit.id, val_options).unwrap();
            sol_sys.remove_subsystem(&info.id).unwrap();
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

struct Market {
    all_ids: Vec<rc::EItemId>,
    boosters: Vec<rc::EItemId>,
    drones: Vec<rc::EItemId>,
    fighters: Vec<rc::EItemId>,
    implants: Vec<rc::EItemId>,
    modules_high: Vec<(rc::EItemId, SolModuleState)>,
    modules_mid: Vec<(rc::EItemId, SolModuleState)>,
    modules_low: Vec<(rc::EItemId, SolModuleState)>,
    rigs: Vec<rc::EItemId>,
    // stances: Vec<rc::EItemId>,
    subsystems: Vec<rc::EItemId>,
}
impl Market {
    fn new(dh: &Box<rdhe::PhbFileEdh>) -> Self {
        Self {
            all_ids: dh.get_items().unwrap().data.into_iter().map(|v| v.id).collect(),
            boosters: Vec::new(),
            drones: Vec::new(),
            fighters: Vec::new(),
            implants: Vec::new(),
            modules_high: Vec::new(),
            modules_mid: Vec::new(),
            modules_low: Vec::new(),
            rigs: Vec::new(),
            subsystems: Vec::new(),
        }
    }
    fn fill(&mut self, src: &Src) {
        for type_id in self.all_ids.iter() {
            let a_item = match src.get_a_item(&type_id) {
                Some(a_item) => a_item,
                None => continue,
            };
            match a_item.extras.kind {
                Some(AItemKind::Booster) => self.boosters.push(*type_id),
                Some(AItemKind::Drone) => self.drones.push(*type_id),
                Some(AItemKind::Fighter) => self.fighters.push(*type_id),
                Some(AItemKind::Implant) => self.implants.push(*type_id),
                Some(AItemKind::ModuleHigh) => self.modules_high.push((*type_id, conv_state(a_item.extras.max_state))),
                Some(AItemKind::ModuleMid) => self.modules_mid.push((*type_id, conv_state(a_item.extras.max_state))),
                Some(AItemKind::ModuleLow) => self.modules_low.push((*type_id, conv_state(a_item.extras.max_state))),
                Some(AItemKind::Rig) => self.rigs.push(*type_id),
                Some(AItemKind::Subsystem) => self.subsystems.push(*type_id),
                _ => continue,
            }
        }
        tracing::error!(
            "collected: {} boosters, {} drones, {} fighters, {} implants, {} highslot mods, {} midslot mods, {} lowslot mods, {} rigs, {} subsystems",
            self.boosters.len(),
            self.drones.len(),
            self.fighters.len(),
            self.implants.len(),
            self.modules_high.len(),
            self.modules_mid.len(),
            self.modules_low.len(),
            self.rigs.len(),
            self.subsystems.len(),
        );
    }
}

fn conv_state(a_state: AState) -> SolModuleState {
    match a_state {
        AState::Offline => SolModuleState::Offline,
        AState::Online => SolModuleState::Online,
        AState::Active => SolModuleState::Active,
        AState::Overload => SolModuleState::Overload,
    }
}
