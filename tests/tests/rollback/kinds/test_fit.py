from fw import approx, check_no_field
from tests.rollback.kinds import rollback_batch, setup_mod_basics


def setup_buff(*, client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_fleet_ships, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    return eve_affectee_attr_id, eve_module_id


def test_add(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_fit = api_sol.create_fit(fleet_id=api_fleet.id)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.create_fit(fleet_id=api_fleet.id, sec_status=-5, rah_incoming_dps=(1, 1, 0, 0))
    assert api_sol.update().fits.keys() == {api_fit.id}
    assert api_fleet.update().fits.keys() == {api_fit.id}


def test_remove(client, consts):
    # Fit has an item of every kind. Its items are projected onto another ship, and a module of
    # another fit and a projected effect are projected onto its ship
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_buff_affectee_attr_id, eve_buff_module_id = setup_buff(client=client, consts=consts)
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_ship_effect_id = client.mk_eve_effect(mod_info=[eve_basic_info.ship_mod])
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_basic_info.tgt_mod])
    eve_system_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_basic_info.ship_mod])
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_autocharge,
        cat_id=consts.EveEffCat.active)
    eve_autocharge_tgt_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.tgt_normal1,
        cat_id=consts.EveEffCat.target,
        mod_info=[eve_basic_info.tgt_mod])
    # One type serves every item kind which applies to the ship of its fit
    eve_local_id = client.mk_eve_item(attrs={eve_basic_info.affector_attr_id: 10}, eff_ids=[eve_ship_effect_id])
    eve_module_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 10},
        eff_ids=[eve_tgt_effect_id],
        defeff_id=eve_tgt_effect_id)
    eve_drone_id = client.mk_eve_drone(
        attrs={eve_basic_info.affector_attr_id: 10},
        eff_ids=[eve_tgt_effect_id],
        defeff_id=eve_tgt_effect_id)
    eve_autocharge_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 10},
        eff_ids=[eve_autocharge_tgt_effect_id],
        defeff_id=eve_autocharge_tgt_effect_id)
    eve_fighter_id = client.mk_eve_fighter(
        attrs={eve_autocharge_attr_id: eve_autocharge_id},
        eff_ids=[eve_autocharge_effect_id],
        defeff_id=eve_autocharge_effect_id)
    eve_proj_effect_id = client.mk_eve_item(attrs={eve_basic_info.affector_attr_id: 10}, eff_ids=[eve_system_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100, eve_buff_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_booster_fit = api_sol.create_fit(fleet_id=api_fleet.id)
    api_booster_fit.add_module(type_id=eve_buff_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit(fleet_id=api_fleet.id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_character = api_fit.set_character(type_id=eve_local_id)
    api_skill = api_fit.add_skill(type_id=eve_local_id, level=1)
    api_implant = api_fit.add_implant(type_id=eve_local_id)
    api_booster = api_fit.add_booster(type_id=eve_local_id)
    api_stance = api_fit.set_stance(type_id=eve_local_id)
    api_subsystem = api_fit.add_subsystem(type_id=eve_local_id)
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_local_id,
        proj_item_ids=[api_tgt_ship.id])
    api_charge_id = api_module.charge.id
    api_rig = api_fit.add_rig(type_id=eve_local_id)
    api_service = api_fit.add_service(type_id=eve_local_id)
    api_drone = api_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        proj_item_ids=[api_tgt_ship.id])
    api_fighter = api_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        proj_item_ids=[api_tgt_ship.id])
    api_autocharge_id = api_fighter.autocharges[eve_autocharge_effect_id].id
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_local_id)
    api_other_module = api_tgt_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id, proj_item_ids=[api_ship.id])
    # Verification - 10 items of the fit and 2 projected items apply to its ship
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.affectee_attr_id].modified == approx(313.842838)
    assert api_ship.attrs[eve_buff_affectee_attr_id].modified == approx(37.5)
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(133.1)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.remove_fit(fit_id=api_fit.id)
    assert api_sol.update().fits.keys() == {api_booster_fit.id, api_tgt_fit.id, api_fit.id}
    assert api_fleet.update().fits.keys() == {api_booster_fit.id, api_fit.id}
    api_fit.update()
    assert api_fit.fleet_id == api_fleet.id
    assert api_fit.ship.id == api_ship.id
    assert api_fit.character.id == api_character.id
    assert api_fit.skills.keys() == {api_skill.id}
    assert api_fit.implants.keys() == {api_implant.id}
    assert api_fit.boosters.keys() == {api_booster.id}
    assert api_fit.stance.id == api_stance.id
    assert api_fit.subsystems.keys() == {api_subsystem.id}
    assert api_fit.modules.high.map(lambda m: m.id) == [api_module.id]
    assert api_fit.rigs.keys() == {api_rig.id}
    assert api_fit.services.keys() == {api_service.id}
    assert api_fit.drones.keys() == {api_drone.id}
    assert api_fit.fighters.keys() == {api_fighter.id}
    assert api_fit.fw_effects.keys() == {api_fw_effect.id}
    api_module.update()
    assert api_module.charge.id == api_charge_id
    assert api_module.projs.keys() == {api_tgt_ship.id}
    assert api_drone.update().projs.keys() == {api_tgt_ship.id}
    api_fighter.update()
    assert api_fighter.autocharges[eve_autocharge_effect_id].id == api_autocharge_id
    assert api_fighter.projs.keys() == {api_tgt_ship.id}
    assert api_other_module.update().projs.keys() == {api_ship.id}
    assert api_proj_effect.update().proj_item_ids == [api_ship.id]
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.affectee_attr_id].modified == approx(313.842838)
    assert api_ship.attrs[eve_buff_affectee_attr_id].modified == approx(37.5)
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(133.1)


def test_change(client, consts):
    eve_buff_affectee_attr_id, eve_buff_module_id = setup_buff(client=client, consts=consts)
    eve_sec_affectee_attr_id = client.mk_eve_attr()
    eve_sec_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.pilot_security_status)
    eve_sec_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_sec_affector_attr_id,
        affectee_attr_id=eve_sec_affectee_attr_id)
    eve_sec_effect_id = client.mk_eve_effect(mod_info=[eve_sec_mod])
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_buff_affectee_attr_id: 7.5, eve_sec_affectee_attr_id: 100},
        eff_ids=[eve_sec_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet1 = api_sol.create_fleet()
    api_fleet2 = api_sol.create_fleet()
    api_booster_fit = api_sol.create_fit(fleet_id=api_fleet2.id)
    api_booster_fit.add_module(type_id=eve_buff_module_id, state=consts.ApiModuleState.active)
    api_fit = api_sol.create_fit(fleet_id=api_fleet1.id, sec_status=2)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_buff_affectee_attr_id].modified == approx(7.5)
    assert api_ship.attrs[eve_sec_affectee_attr_id].modified == approx(200)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_fit(fit_id=api_fit.id, fleet_id=api_fleet2.id, sec_status=-5)
    api_fit.update()
    assert api_fit.fleet_id == api_fleet1.id
    assert api_fit.sec_status == approx(2)
    assert api_fleet1.update().fits.keys() == {api_fit.id}
    assert api_fleet2.update().fits.keys() == {api_booster_fit.id}
    api_ship.update()
    assert api_ship.attrs[eve_buff_affectee_attr_id].modified == approx(7.5)
    assert api_ship.attrs[eve_sec_affectee_attr_id].modified == approx(200)


def test_change_rah_incoming_dps(client, consts):
    eve_res_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_max_dmg_resonance, def_val=1)
    eve_res_em_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_em_dmg_resonance,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_therm_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_therm_dmg_resonance,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_kin_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_kin_dmg_resonance,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_expl_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_expl_dmg_resonance,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_shift_attr_id = client.mk_eve_attr(id_=consts.EveAttr.resist_shift_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.duration)
    eve_rah_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.adaptive_armor_hardener,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_rah_id = client.mk_eve_item(
        attrs={
            eve_res_em_attr_id: 0.85,
            eve_res_therm_attr_id: 0.85,
            eve_res_kin_attr_id: 0.85,
            eve_res_expl_attr_id: 0.85,
            eve_res_shift_attr_id: 6,
            eve_cycle_time_attr_id: 10000},
        eff_ids=[eve_rah_effect_id],
        defeff_id=eve_rah_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={
        eve_res_em_attr_id: 0.5,
        eve_res_therm_attr_id: 0.65,
        eve_res_kin_attr_id: 0.75,
        eve_res_expl_attr_id: 0.9})
    client.create_sources()
    api_sol = client.create_sol(default_incoming_dps=(0, 0, 1, 1))
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_res_em_attr_id].modified == approx(0.5)
    assert api_ship.attrs[eve_res_therm_attr_id].modified == approx(0.65)
    assert api_ship.attrs[eve_res_kin_attr_id].modified == approx(0.525)
    assert api_ship.attrs[eve_res_expl_attr_id].modified == approx(0.63)
    # Action & verification - fit gets its own profile instead of the default one
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_fit(fit_id=api_fit.id, rah_incoming_dps=(1, 1, 0, 0))
    with check_no_field():
        api_fit.update().rah_incoming_dps  # ruff:ignore[useless-expression]
    api_rah.update()
    assert api_rah.attrs[eve_res_em_attr_id].modified == approx(1)
    assert api_rah.attrs[eve_res_therm_attr_id].modified == approx(1)
    assert api_rah.attrs[eve_res_kin_attr_id].modified == approx(0.7)
    assert api_rah.attrs[eve_res_expl_attr_id].modified == approx(0.7)
    api_ship.update()
    assert api_ship.attrs[eve_res_em_attr_id].modified == approx(0.5)
    assert api_ship.attrs[eve_res_therm_attr_id].modified == approx(0.65)
    assert api_ship.attrs[eve_res_kin_attr_id].modified == approx(0.525)
    assert api_ship.attrs[eve_res_expl_attr_id].modified == approx(0.63)
