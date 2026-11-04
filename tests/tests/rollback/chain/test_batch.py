from fw import Spool, approx, check_no_field
from fw.api import (
    FleetStatsOptions,
    ItemStatsOptions,
    StatsOptionItemOutCps,
    StatsOptionJump,
    StatsOptionMass,
    ValOptions,
)


def test_sol_additions_manual(client, consts):
    eve_skill_level_attr_id = client.mk_eve_attr(id_=consts.EveAttr.skill_level)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_attr_id = client.mk_eve_attr()
    eve_tgt_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_tgt_mod])
    eve_ship_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_ship_effect_id = client.mk_eve_effect(mod_info=[eve_ship_mod])
    eve_system_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_ship_mod])
    eve_skill_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_skill_level_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_skill_effect_id = client.mk_eve_effect(mod_info=[eve_skill_mod])
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_tgt_effect_id],
        defeff_id=eve_tgt_effect_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_affector_attr_id: 50}, eff_ids=[eve_ship_effect_id])
    eve_drone_id = client.mk_eve_drone(
        attrs={eve_affector_attr_id: 30},
        eff_ids=[eve_tgt_effect_id],
        defeff_id=eve_tgt_effect_id)
    eve_skill_id = client.mk_eve_item(attrs={eve_skill_level_attr_id: 0}, eff_ids=[eve_skill_effect_id])
    eve_sw_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 10}, eff_ids=[eve_system_effect_id])
    eve_proj_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 40}, eff_ids=[eve_system_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec, default_spool=Spool.cycles_to_api(count=0))
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_other_item = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_item.remove()
    # Action & verification - every command but the last adds or sets up something
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'ITM-001',
            'message': f'failed to remove item: item {api_other_item.id} not found',
            'cmd_index': 10,
    }) as api_sol_batch:
        api_new_fleet = api_sol_batch.create_fleet()
        api_new_fit = api_sol_batch.create_fit(fleet_id=api_new_fleet.id)
        api_sol_batch.set_ship(fit_id=api_new_fit.id, type_id=eve_ship_id)
        api_sol_batch.add_module(
            fit_id=api_fit.id,
            type_id=eve_module_id,
            state=consts.ApiModuleState.active,
            charge_type_id=eve_charge_id,
            proj_item_ids=[api_tgt_ship.id])
        api_sol_batch.add_drone(
            fit_id=api_fit.id,
            type_id=eve_drone_id,
            state=consts.ApiMinionState.engaging,
            proj_item_ids=[api_tgt_ship.id])
        api_sol_batch.add_skill(fit_id=api_fit.id, type_id=eve_skill_id, level=5)
        api_sol_batch.add_sw_effect(type_id=eve_sw_effect_id)
        api_sol_batch.add_proj_effect(type_id=eve_proj_effect_id, proj_item_ids=[api_tgt_ship.id])
        api_sol_batch.change_sol(sec_zone=consts.ApiSecZone.nullsec, default_spool=Spool.cycles_to_api(count=1))
        api_sol_batch.change_fit(fit_id=api_fit.id, sec_status=-5)
        api_sol_batch.remove_item(item_id=api_other_item.id)
    api_sol.update()
    assert api_sol.sec_zone == consts.ApiSecZone.hisec
    assert api_sol.default_spool == Spool.cycles_to_api(count=0)
    with check_no_field():
        api_sol.fleets  # ruff:ignore[useless-expression]
    assert api_sol.fits.keys() == {api_tgt_fit.id, api_fit.id}
    with check_no_field():
        api_sol.sw_effects  # ruff:ignore[useless-expression]
    with check_no_field():
        api_sol.proj_effects  # ruff:ignore[useless-expression]
    api_fit.update()
    assert api_fit.sec_status == approx(0)
    with check_no_field():
        api_fit.modules  # ruff:ignore[useless-expression]
    with check_no_field():
        api_fit.drones  # ruff:ignore[useless-expression]
    with check_no_field():
        api_fit.skills  # ruff:ignore[useless-expression]
    assert api_ship.update().attrs[eve_attr_id].modified == approx(100)
    assert api_tgt_ship.update().attrs[eve_attr_id].modified == approx(100)


def test_sol_additions_autodetect(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_attr_id = client.mk_eve_attr()
    eve_ship_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_ship_effect_id = client.mk_eve_effect(mod_info=[eve_ship_mod])
    eve_drone_id = client.mk_eve_drone()
    eve_implant_id = client.mk_eve_implant(attrs={eve_affector_attr_id: 30}, eff_ids=[eve_ship_effect_id])
    eve_module_id = client.mk_eve_module(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_ship_effect_id],
        rack=consts.Rack.high)
    eve_filler_module_id = client.mk_eve_module(rack=consts.Rack.high)
    eve_rig_id = client.mk_eve_rig(attrs={eve_affector_attr_id: 10}, eff_ids=[eve_ship_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_filler_module_id, add_mode={consts.ApiModAddMode.replace: 0})
    api_module3 = api_fit.add_module(type_id=eve_filler_module_id, add_mode={consts.ApiModAddMode.replace: 2})
    api_other_item = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_item.remove()
    # Verification
    assert api_ship.update().attrs[eve_attr_id].modified == approx(100)
    # Action & verification - autodetected items are added, the module into the free slot, and
    # everything is calculated before the batch fails
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'ITM-001',
            'message': f'failed to remove item: item {api_other_item.id} not found',
            'cmd_index': 5,
    }) as api_sol_batch:
        api_sol_batch.autodetect_add_item(fit_id=api_fit.id, type_id=eve_drone_id)
        api_sol_batch.autodetect_add_item(fit_id=api_fit.id, type_id=eve_implant_id)
        api_sol_batch.autodetect_add_item(fit_id=api_fit.id, type_id=eve_module_id)
        api_sol_batch.autodetect_add_item(fit_id=api_fit.id, type_id=eve_rig_id)
        api_sol_batch.get_sol_info(
            sol_mode=consts.ApiSolInfoMode.full,
            fleet_mode=consts.ApiFleetInfoMode.full,
            fit_mode=consts.ApiFitInfoMode.full,
            item_mode=consts.ApiItemInfoMode.full)
        api_sol_batch.remove_item(item_id=api_other_item.id)
    api_fit.update()
    assert api_fit.modules.high.map(lambda m: m.id) == [api_module1.id, None, api_module3.id]
    with check_no_field():
        api_fit.drones  # ruff:ignore[useless-expression]
    with check_no_field():
        api_fit.implants  # ruff:ignore[useless-expression]
    with check_no_field():
        api_fit.rigs  # ruff:ignore[useless-expression]
    assert api_ship.update().attrs[eve_attr_id].modified == approx(100)


def test_sol_removals(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_attr_id)])
    eve_buff_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_fleet_ships, cat_id=consts.EveEffCat.active)
    eve_tgt_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_tgt_mod])
    eve_ship_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_ship_effect_id = client.mk_eve_effect(mod_info=[eve_ship_mod])
    eve_system_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_ship_mod])
    eve_buff_module_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 2},
        eff_ids=[eve_buff_effect_id],
        defeff_id=eve_buff_effect_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_tgt_effect_id],
        defeff_id=eve_tgt_effect_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_affector_attr_id: 50}, eff_ids=[eve_ship_effect_id])
    eve_drone_id = client.mk_eve_drone(
        attrs={eve_affector_attr_id: 30},
        eff_ids=[eve_tgt_effect_id],
        defeff_id=eve_tgt_effect_id)
    eve_sw_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 10}, eff_ids=[eve_system_effect_id])
    eve_proj_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 40}, eff_ids=[eve_system_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    api_fleet = api_sol.create_fleet()
    api_booster_fit = api_sol.create_fit(fleet_id=api_fleet.id)
    api_booster_fit.add_module(type_id=eve_buff_module_id, state=consts.ApiModuleState.active)
    api_boosted_fit = api_sol.create_fit(fleet_id=api_fleet.id)
    api_boosted_ship = api_boosted_fit.set_ship(type_id=eve_ship_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id, proj_item_ids=[api_tgt_ship.id])
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id,
        proj_item_ids=[api_tgt_ship.id])
    api_charge_id = api_module.charge.id
    api_drone = api_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        proj_item_ids=[api_tgt_ship.id])
    # Verification
    assert api_boosted_ship.update().attrs[eve_attr_id].modified == approx(220)
    assert api_ship.update().attrs[eve_attr_id].modified == approx(165)
    assert api_tgt_ship.update().attrs[eve_attr_id].modified == approx(240.24)
    # Action & verification - every command but the last removes something; the last one fails on an
    # item which was removed earlier in the batch
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'ITM-001',
            'message': f'failed to change module: item {api_module.id} not found',
            'cmd_index': 6,
    }) as api_sol_batch:
        api_sol_batch.remove_item(item_id=api_module.id)
        api_sol_batch.remove_item(item_id=api_drone.id)
        api_sol_batch.remove_fit(fit_id=api_boosted_fit.id)
        api_sol_batch.remove_fleet(fleet_id=api_fleet.id)
        api_sol_batch.remove_item(item_id=api_tgt_ship.id)
        api_sol_batch.remove_item(item_id=api_sw_effect.id)
        api_sol_batch.change_module(item_id=api_module.id, state=consts.ApiModuleState.online)
    api_sol.update()
    assert api_sol.fleets.keys() == {api_fleet.id}
    assert api_sol.fits.keys() == {api_booster_fit.id, api_boosted_fit.id, api_tgt_fit.id, api_fit.id}
    assert api_sol.sw_effects.keys() == {api_sw_effect.id}
    assert api_fleet.update().fits.keys() == {api_booster_fit.id, api_boosted_fit.id}
    api_module.update()
    assert api_module.state == consts.ApiModuleState.active
    assert api_module.charge.id == api_charge_id
    assert api_module.projs.keys() == {api_tgt_ship.id}
    assert api_drone.update().projs.keys() == {api_tgt_ship.id}
    assert api_proj_effect.update().proj_item_ids == [api_tgt_ship.id]
    assert api_tgt_fit.update().ship.id == api_tgt_ship.id
    assert api_boosted_ship.update().attrs[eve_attr_id].modified == approx(220)
    assert api_ship.update().attrs[eve_attr_id].modified == approx(165)
    assert api_tgt_ship.update().attrs[eve_attr_id].modified == approx(240.24)


def test_sol_changes(client, consts):
    eve_skill_level_attr_id = client.mk_eve_attr(id_=consts.EveAttr.skill_level)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_attr_id = client.mk_eve_attr()
    eve_ship_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_ship_effect_id = client.mk_eve_effect(mod_info=[eve_ship_mod])
    eve_skill_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_skill_level_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_skill_effect_id = client.mk_eve_effect(mod_info=[eve_skill_mod])
    eve_module1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_ship_effect_id])
    eve_module2_id = client.mk_eve_item(attrs={eve_affector_attr_id: 30}, eff_ids=[eve_ship_effect_id])
    eve_charge_id = client.mk_eve_item(attrs={eve_affector_attr_id: 50}, eff_ids=[eve_ship_effect_id])
    eve_skill_id = client.mk_eve_item(attrs={eve_skill_level_attr_id: 0}, eff_ids=[eve_skill_effect_id])
    eve_ship1_id = client.mk_eve_ship(attrs={eve_attr_id: 100})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol(default_spool=Spool.cycles_to_api(count=0))
    api_fleet = api_sol.create_fleet()
    api_fit = api_sol.create_fit(sec_status=2)
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    api_module1 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.online)
    api_module2 = api_fit.add_module(type_id=eve_module1_id, charge_type_id=eve_charge_id)
    api_charge_id = api_module2.charge.id
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=5)
    # Verification
    assert api_ship.update().attrs[eve_attr_id].modified == approx(226.8)
    # Action & verification - every command but the last changes something; the last one refers to a
    # command which has no results
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #20 does not have results recorded',
            'cmd_index': 7,
    }) as api_sol_batch:
        api_sol_batch.change_module(item_id=api_module1.id, type_id=eve_module2_id, state=consts.ApiModuleState.offline)
        api_sol_batch.change_module(item_id=api_module2.id, charge_type_id=None)
        api_sol_batch.add_module(fit_id=api_fit.id, type_id=eve_module2_id, add_mode={consts.ApiModAddMode.replace: 1})
        api_sol_batch.change_skill(item_id=api_skill.id, level=1)
        api_sol_batch.set_ship(fit_id=api_fit.id, type_id=eve_ship2_id)
        api_sol_batch.change_fit(fit_id=api_fit.id, fleet_id=api_fleet.id, sec_status=-5)
        api_sol_batch.change_sol(default_spool=Spool.cycles_to_api(count=1))
        api_sol_batch.change_module(item_id='#20', state=consts.ApiModuleState.online)
    assert api_sol.update().default_spool == Spool.cycles_to_api(count=0)
    api_fit.update()
    assert api_fit.sec_status == approx(2)
    with check_no_field():
        api_fit.fleet_id  # ruff:ignore[useless-expression]
    assert api_fit.ship.id == api_ship.id
    assert api_fit.modules.high.map(lambda m: m.id) == [api_module1.id, api_module2.id]
    api_module1.update()
    assert api_module1.type_id == eve_module1_id
    assert api_module1.state == consts.ApiModuleState.online
    assert api_module2.update().charge.id == api_charge_id
    assert api_skill.update().level == 5
    assert api_ship.update().type_id == eve_ship1_id
    assert api_ship.attrs[eve_attr_id].modified == approx(226.8)


def test_sol_info_fail(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_attr_id = client.mk_eve_attr()
    eve_ship_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_ship_effect_id = client.mk_eve_effect(mod_info=[eve_ship_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_ship_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_other_item = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_item.remove()
    # Action & verification - info request for a missing item fails after module is added
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'ITM-001',
            'message': f'item {api_other_item.id} not found',
            'cmd_index': 1,
    }) as api_sol_batch:
        api_sol_batch.add_module(fit_id=api_fit.id, type_id=eve_module_id)
        api_sol_batch.get_item_info(item_id=api_other_item.id)
    api_fit.update()
    with check_no_field():
        api_fit.modules  # ruff:ignore[useless-expression]
    assert api_ship.update().attrs[eve_attr_id].modified == approx(100)


def test_sol_stats_fail(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_attr_id = client.mk_eve_attr()
    eve_ship_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_ship_effect_id = client.mk_eve_effect(mod_info=[eve_ship_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_ship_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_other_fit = api_sol.create_fit()
    api_other_fit.remove()
    # Action & verification - stats request for a missing fit fails after module is added
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'FIT-001',
            'message': f'fit {api_other_fit.id} not found',
            'cmd_index': 1,
    }) as api_sol_batch:
        api_sol_batch.add_module(fit_id=api_fit.id, type_id=eve_module_id)
        api_sol_batch.get_fit_stats(fit_id=api_other_fit.id)
    api_fit.update()
    with check_no_field():
        api_fit.modules  # ruff:ignore[useless-expression]
    assert api_ship.update().attrs[eve_attr_id].modified == approx(100)


def test_sol_try_fit_items(client, consts):
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_attr_id = client.mk_eve_attr()
    eve_ship_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_ship_effect_id = client.mk_eve_effect(mod_info=[eve_ship_mod])
    eve_grp1_id = client.mk_eve_charge_group()
    eve_grp2_id = client.mk_eve_charge_group()
    eve_charge1_id = client.mk_eve_charge(
        grp_id=eve_grp1_id,
        attrs={eve_affector_attr_id: 50},
        eff_ids=[eve_ship_effect_id])
    eve_charge2_id = client.mk_eve_charge(
        grp_id=eve_grp1_id,
        attrs={eve_affector_attr_id: 30},
        eff_ids=[eve_ship_effect_id])
    eve_charge3_id = client.mk_eve_charge(
        grp_id=eve_grp2_id,
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_ship_effect_id])
    eve_module1_id = client.mk_eve_module(attrs={eve_group_attr_id: eve_grp1_id, eve_capacity_attr_id: 1})
    eve_module2_id = client.mk_eve_module(attrs={eve_group_attr_id: eve_grp2_id, eve_capacity_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_module1_id,
        add_mode={consts.ApiModAddMode.replace: 1},
        charge_type_id=eve_charge1_id)
    api_charge_id = api_module.charge.id
    api_other_item = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_item.remove()
    # Verification
    assert api_ship.update().attrs[eve_attr_id].modified == approx(150)
    # Action & verification - try-fit rolls back to its savepoints between batch changes, and the
    # batch fails after it. The new module comes first in the rack and accepts only charge 3, so
    # charge 1 is tried in the old module too, detaching charges loaded by the batch in both.
    # Commands after try-fit need the new module and its charge, and the swap moves the old module
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'ITM-001',
            'message': f'failed to remove item: item {api_other_item.id} not found',
            'cmd_index': 5,
    }) as api_sol_batch:
        api_sol_batch.change_module(item_id=api_module.id, charge_type_id=eve_charge2_id)
        api_new_module = api_sol_batch.add_module(
            fit_id=api_fit.id,
            type_id=eve_module2_id,
            add_mode={consts.ApiModAddMode.replace: 0},
            charge_type_id=eve_charge3_id)
        api_sol_batch.try_fit_items(
            fit_id=api_fit.id,
            type_ids=[eve_module1_id, eve_charge1_id, eve_charge3_id],
            val_options=ValOptions(charge_group=True))
        api_sol_batch.change_charge(item_id=api_new_module.charge.id, state=False)
        api_sol_batch.change_module(item_id=api_new_module.id, move={consts.ApiModMvMode.swap: 1})
        api_sol_batch.remove_item(item_id=api_other_item.id)
    assert api_fit.update().modules.high.map(lambda m: m.id) == [None, api_module.id]
    api_module.update()
    assert api_module.charge.id == api_charge_id
    assert api_module.charge.type_id == eve_charge1_id
    assert api_ship.update().attrs[eve_attr_id].modified == approx(150)


def test_sol_stats_state_switch(client, consts):
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_attr_id = client.mk_eve_attr()
    eve_transfer_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_ship_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_ship_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online, mod_info=[eve_ship_mod])
    eve_online_effect_id = client.mk_eve_online_effect()
    eve_module_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 351, eve_cycle_time_attr_id: 5000, eve_affector_attr_id: 20},
        eff_ids=[eve_transfer_effect_id, eve_ship_effect_id, eve_online_effect_id],
        defeff_id=eve_transfer_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    api_other_item = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_item.remove()
    # Verification
    assert api_ship.update().attrs[eve_attr_id].modified == approx(100)
    # Action & verification - stats request activates the module the batch put online, rolls back to
    # its savepoint, and the batch fails after it. The swap after it needs the module added right
    # before the stats request, and moves the old module
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'ITM-001',
            'message': f'failed to remove item: item {api_other_item.id} not found',
            'cmd_index': 4,
    }) as api_sol_batch:
        api_sol_batch.change_module(item_id=api_module.id, state=consts.ApiModuleState.online)
        api_new_module = api_sol_batch.add_module(
            fit_id=api_fit.id,
            type_id=eve_module_id,
            state=consts.ApiModuleState.online)
        api_sol_batch.get_item_stats(item_id=api_module.id, item_options=ItemStatsOptions(
            outgoing_cps=[StatsOptionItemOutCps(state=consts.ApiStatItemState.switch)]))
        api_sol_batch.change_module(item_id=api_new_module.id, move={consts.ApiModMvMode.swap: 0})
        api_sol_batch.remove_item(item_id=api_other_item.id)
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module.id]
    assert api_module.update().state == consts.ApiModuleState.offline
    assert api_ship.update().attrs[eve_attr_id].modified == approx(100)


def test_sol_stats_affector_switch_item(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_mass_attr_id)
    eve_online_effect_id = client.mk_eve_online_effect()
    eve_mod_online_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online, mod_info=[eve_mod])
    eve_mod_active_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod])
    eve_online_module_id = client.mk_eve_item(
        attrs={eve_mod_attr_id: 25},
        eff_ids=[eve_mod_online_effect_id, eve_online_effect_id])
    eve_active_module_id = client.mk_eve_item(
        attrs={eve_mod_attr_id: 25},
        eff_ids=[eve_mod_active_effect_id],
        defeff_id=eve_mod_active_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_online_module_id)
    api_other_item = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_item.remove()
    # Verification
    assert api_ship.update().attrs[eve_mass_attr_id].modified == approx(100)
    # Action & verification - stats request offlines modules the batch put online and added, rolls
    # back to its savepoint, and the batch fails after it. The swap after it needs the module added
    # right before the stats request, and moves the old module
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'ITM-001',
            'message': f'failed to remove item: item {api_other_item.id} not found',
            'cmd_index': 4,
    }) as api_sol_batch:
        api_sol_batch.change_module(item_id=api_module.id, state=consts.ApiModuleState.online)
        api_new_module = api_sol_batch.add_module(
            fit_id=api_fit.id,
            type_id=eve_active_module_id,
            state=consts.ApiModuleState.active)
        api_sol_batch.get_item_stats(item_id=api_ship.id, item_options=ItemStatsOptions(
            mass=[StatsOptionMass(affectors=consts.ApiCtlAffector.offline)]))
        api_sol_batch.change_module(item_id=api_new_module.id, move={consts.ApiModMvMode.swap: 0})
        api_sol_batch.remove_item(item_id=api_other_item.id)
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module.id]
    assert api_module.update().state == consts.ApiModuleState.offline
    assert api_ship.update().attrs[eve_mass_attr_id].modified == approx(100)


def test_sol_stats_affector_switch_fleet(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_mass_attr_id)
    eve_online_effect_id = client.mk_eve_online_effect()
    eve_mod_online_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online, mod_info=[eve_mod])
    eve_mod_active_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod])
    eve_online_module_id = client.mk_eve_item(
        attrs={eve_mod_attr_id: 25},
        eff_ids=[eve_mod_online_effect_id, eve_online_effect_id])
    eve_active_module_id = client.mk_eve_item(
        attrs={eve_mod_attr_id: 25},
        eff_ids=[eve_mod_active_effect_id],
        defeff_id=eve_mod_active_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_fit = api_sol.create_fit(fleet_id=api_fleet.id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_online_module_id)
    api_other_item = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_item.remove()
    # Verification
    assert api_ship.update().attrs[eve_mass_attr_id].modified == approx(100)
    # Action & verification - fleet stats request offlines modules the batch put online and added,
    # rolls back to its savepoint, and the batch fails after it. The swap after it needs the module
    # added right before the stats request, and moves the old module
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'ITM-001',
            'message': f'failed to remove item: item {api_other_item.id} not found',
            'cmd_index': 4,
    }) as api_sol_batch:
        api_sol_batch.change_module(item_id=api_module.id, state=consts.ApiModuleState.online)
        api_new_module = api_sol_batch.add_module(
            fit_id=api_fit.id,
            type_id=eve_active_module_id,
            state=consts.ApiModuleState.active)
        api_sol_batch.get_fleet_stats(fleet_id=api_fleet.id, fleet_options=FleetStatsOptions(
            mass=[StatsOptionMass(affectors=consts.ApiCtlAffector.offline)]))
        api_sol_batch.change_module(item_id=api_new_module.id, move={consts.ApiModMvMode.swap: 0})
        api_sol_batch.remove_item(item_id=api_other_item.id)
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module.id]
    assert api_module.update().state == consts.ApiModuleState.offline
    assert api_ship.update().attrs[eve_mass_attr_id].modified == approx(100)


def test_sol_stats_affector_switch_jump(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_mass_attr_id)
    eve_online_effect_id = client.mk_eve_online_effect()
    eve_mod_online_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online, mod_info=[eve_mod])
    eve_mod_active_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod])
    eve_online_module_id = client.mk_eve_item(
        attrs={eve_mod_attr_id: 25},
        eff_ids=[eve_mod_online_effect_id, eve_online_effect_id])
    eve_active_module_id = client.mk_eve_item(
        attrs={eve_mod_attr_id: 25},
        eff_ids=[eve_mod_active_effect_id],
        defeff_id=eve_mod_active_effect_id)
    eve_fuel_id = client.mk_eve_item()
    eve_main_ship_id = client.mk_eve_ship(attrs={eve_range_attr_id: 5, eve_fuel_type_attr_id: eve_fuel_id})
    eve_psg_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_main_fit = api_sol.create_fit()
    api_main_ship = api_main_fit.set_ship(type_id=eve_main_ship_id)
    api_psg_fit = api_sol.create_fit()
    api_psg_ship = api_psg_fit.set_ship(type_id=eve_psg_ship_id)
    api_module = api_psg_fit.add_module(type_id=eve_online_module_id)
    api_other_item = api_sol.add_sw_effect(type_id=eve_psg_ship_id)
    api_other_item.remove()
    # Verification
    assert api_psg_ship.update().attrs[eve_mass_attr_id].modified == approx(100)
    # Action & verification - jump stats request offlines passenger modules the batch put online and
    # added, rolls back to its savepoint, and the batch fails after it. The swap after it needs the
    # module added right before the stats request, and moves the old module
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'ITM-001',
            'message': f'failed to remove item: item {api_other_item.id} not found',
            'cmd_index': 4,
    }) as api_sol_batch:
        api_sol_batch.change_module(item_id=api_module.id, state=consts.ApiModuleState.online)
        api_new_module = api_sol_batch.add_module(
            fit_id=api_psg_fit.id,
            type_id=eve_active_module_id,
            state=consts.ApiModuleState.active)
        api_sol_batch.get_item_stats(item_id=api_main_ship.id, item_options=ItemStatsOptions(jump=[StatsOptionJump(
            passenger_fit_ids=[api_psg_fit.id],
            passenger_fuel_affectors=consts.ApiCtlAffector.offline)]))
        api_sol_batch.change_module(item_id=api_new_module.id, move={consts.ApiModMvMode.swap: 0})
        api_sol_batch.remove_item(item_id=api_other_item.id)
    assert api_psg_fit.update().modules.high.map(lambda m: m.id) == [api_module.id]
    assert api_module.update().state == consts.ApiModuleState.offline
    assert api_psg_ship.update().attrs[eve_mass_attr_id].modified == approx(100)


def test_fit(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_attr_id = client.mk_eve_attr()
    eve_ship_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_ship_effect_id = client.mk_eve_effect(mod_info=[eve_ship_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_ship_effect_id])
    eve_rig_id = client.mk_eve_item(attrs={eve_affector_attr_id: 10}, eff_ids=[eve_ship_effect_id])
    eve_implant_id = client.mk_eve_item(attrs={eve_affector_attr_id: 30}, eff_ids=[eve_ship_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    assert api_ship.update().attrs[eve_attr_id].modified == approx(132)
    api_other_item = api_sol.add_sw_effect(type_id=eve_rig_id)
    api_other_item.remove()
    # Action & verification - fit batch adds, changes and removes items before failing on a missing
    # item
    with api_fit.batch(status_code=400, json_predicate={
            'code': 'ITM-001',
            'message': f'failed to change rig: item {api_other_item.id} not found',
            'cmd_index': 4,
    }) as api_fit_batch:
        api_fit_batch.add_implant(type_id=eve_implant_id)
        api_fit_batch.change_module(item_id=api_module.id, state=consts.ApiModuleState.disabled)
        api_fit_batch.remove_item(item_id=api_rig.id)
        api_fit_batch.change_fit(sec_status=-5)
        api_fit_batch.change_rig(item_id=api_other_item.id, state=False)
    api_fit.update()
    assert api_fit.sec_status == approx(0)
    with check_no_field():
        api_fit.implants  # ruff:ignore[useless-expression]
    assert api_fit.rigs.keys() == {api_rig.id}
    assert api_module.update().state == consts.ApiModuleState.offline
    assert api_ship.update().attrs[eve_attr_id].modified == approx(132)
