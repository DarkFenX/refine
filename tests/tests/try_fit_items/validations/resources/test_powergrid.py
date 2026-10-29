from fw import Muta
from fw.api import FitStatsOptions, ValOptions


def test_fail_single(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_module1_id = client.mk_eve_module(attrs={eve_use_attr_id: 150})
    eve_module2_id = client.mk_eve_module(attrs={eve_use_attr_id: 100})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(powergrid=True))
    assert api_stats.powergrid.one() == (0, 125)
    api_fit.check_try_fit_items(
        val_options=ValOptions(powergrid=True),
        passing=[eve_module2_id],
        failing=[eve_module1_id])


def test_fail_multiple_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_module1_id = client.mk_eve_module(attrs={eve_use_attr_id: 50})
    eve_module2_id = client.mk_eve_module(attrs={eve_use_attr_id: 100})
    eve_module3_id = client.mk_eve_module(attrs={eve_use_attr_id: 75})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(powergrid=True))
    assert api_stats.powergrid.one() == (50, 125)
    api_fit.check_try_fit_items(
        val_options=ValOptions(powergrid=True),
        passing=[eve_module3_id],
        failing=[eve_module2_id])


def test_fail_multiple_struct(client, consts):
    # Test service PG use as well
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_module_id = client.mk_eve_module(cat_id=consts.EveItemCat.structure_module, attrs={eve_use_attr_id: 50})
    eve_service1_id = client.mk_eve_service(attrs={eve_use_attr_id: 100})
    eve_service2_id = client.mk_eve_service(attrs={eve_use_attr_id: 75})
    eve_struct_id = client.mk_eve_struct(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(powergrid=True))
    assert api_stats.powergrid.one() == (50, 125)
    api_fit.check_try_fit_items(
        val_options=ValOptions(powergrid=True),
        passing=[eve_service2_id],
        failing=[eve_service1_id])


def test_equal(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_module_id = client.mk_eve_module(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(powergrid=True))
    assert api_stats.powergrid.one() == (0, 150)
    api_fit.check_try_fit_items(val_options=ValOptions(powergrid=True), passing=[eve_module_id], failing=[])


def test_known_failures(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_module1_id = client.mk_eve_module(attrs={eve_use_attr_id: 150})
    eve_module2_id = client.mk_eve_module(attrs={eve_use_attr_id: 100})
    eve_module3_id = client.mk_eve_module(attrs={eve_use_attr_id: -10})
    eve_module4_id = client.mk_eve_module(attrs={eve_use_attr_id: 0})
    eve_module5_id = client.mk_eve_module(attrs={eve_use_attr_id: 0.5})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.online)
    api_module2 = api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.online)
    # Verification - items which do not use the resource do not fail it
    api_stats = api_fit.get_stats(options=FitStatsOptions(powergrid=True))
    assert api_stats.powergrid.one() == (250, 125)
    api_fit.check_try_fit_items(
        val_options=ValOptions(powergrid=[api_module1.id]),
        passing=[],
        failing=[eve_module3_id, eve_module4_id, eve_module2_id, eve_module5_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(powergrid=[api_module2.id]),
        passing=[],
        failing=[eve_module3_id, eve_module4_id, eve_module2_id, eve_module5_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(powergrid=[api_module1.id, api_module2.id]),
        passing=[eve_module3_id, eve_module4_id],
        failing=[eve_module2_id, eve_module5_id])


def test_modified_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_use_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_module1_id = client.mk_eve_module(attrs={eve_use_attr_id: 20})
    eve_module2_id = client.mk_eve_module(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: -50}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.online)
    # Verification
    assert api_module.update().attrs[eve_use_attr_id].modified == 20
    api_stats = api_fit.get_stats(options=FitStatsOptions(powergrid=True))
    assert api_stats.powergrid.one() == (20, 125)
    api_fit.check_try_fit_items(val_options=ValOptions(powergrid=True), passing=[], failing=[eve_module2_id])
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_module.update().attrs[eve_use_attr_id].modified == 10
    api_stats = api_fit.get_stats(options=FitStatsOptions(powergrid=True))
    assert api_stats.powergrid.one() == (10, 125)
    api_fit.check_try_fit_items(val_options=ValOptions(powergrid=True), passing=[eve_module2_id], failing=[])


def test_modified_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_module_id = client.mk_eve_module(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 120})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 50}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].modified == 120
    api_stats = api_fit.get_stats(options=FitStatsOptions(powergrid=True))
    assert api_stats.powergrid.one() == (0, 120)
    api_fit.check_try_fit_items(val_options=ValOptions(powergrid=True), passing=[], failing=[eve_module_id])
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].modified == 180
    api_stats = api_fit.get_stats(options=FitStatsOptions(powergrid=True))
    assert api_stats.powergrid.one() == (0, 180)
    api_fit.check_try_fit_items(val_options=ValOptions(powergrid=True), passing=[eve_module_id], failing=[])


def test_mutation_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_base_module_id = client.mk_eve_module(attrs={eve_use_attr_id: 120})
    eve_mutated_module_id = client.mk_eve_module()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={eve_use_attr_id: (0.8, 1.2)})
    eve_module1_id = client.mk_eve_module(attrs={eve_use_attr_id: 17})
    eve_module2_id = client.mk_eve_module(attrs={eve_use_attr_id: 18})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_base_module_id,
        state=consts.ApiModuleState.online,
        mutation=(eve_mutator_id, {eve_use_attr_id: Muta.roll_to_api(val=0.25)}))
    # Verification
    assert api_module.update().attrs[eve_use_attr_id].modified == 108
    api_stats = api_fit.get_stats(options=FitStatsOptions(powergrid=True))
    assert api_stats.powergrid.one() == (108, 125)
    api_fit.check_try_fit_items(
        val_options=ValOptions(powergrid=True),
        passing=[eve_module1_id],
        failing=[eve_module2_id])
    # Action
    api_module.change_module(mutation=None)
    # Verification
    assert api_module.update().attrs[eve_use_attr_id].modified == 120
    api_stats = api_fit.get_stats(options=FitStatsOptions(powergrid=True))
    assert api_stats.powergrid.one() == (120, 125)
    api_fit.check_try_fit_items(
        val_options=ValOptions(powergrid=True),
        passing=[],
        failing=[eve_module1_id, eve_module2_id])


def test_rounding(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_module1_id = client.mk_eve_module(attrs={eve_use_attr_id: 0.006})
    eve_module2_id = client.mk_eve_module(attrs={eve_use_attr_id: 5.227})
    eve_module3_id = client.mk_eve_module(attrs={eve_use_attr_id: 5.224})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 5.234})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(powergrid=True))
    assert api_stats.powergrid.one() == (0.01, 5.23)
    api_fit.check_try_fit_items(
        val_options=ValOptions(powergrid=True),
        passing=[eve_module3_id],
        failing=[eve_module2_id])


def test_sum_rounding(client, consts):
    # Individual PG attribute values are rounded to 2nd decimal digit; check that total sum of
    # users is rounded; if there would be no rounding, sum of three 0.1 elements would exceed 0.3
    # due to float inaccuracies
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_module_id = client.mk_eve_module(attrs={eve_use_attr_id: 0.1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0.3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(powergrid=True))
    assert api_stats.powergrid.one() == (0.2, 0.3)
    api_fit.check_try_fit_items(val_options=ValOptions(powergrid=True), passing=[eve_module_id], failing=[])


def test_no_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_module1_id = client.mk_eve_module(attrs={eve_use_attr_id: 5})
    eve_module2_id = client.mk_eve_module()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(powergrid=True))
    assert api_stats.powergrid.one() == (0, None)
    api_fit.check_try_fit_items(
        val_options=ValOptions(powergrid=True),
        passing=[eve_module2_id],
        failing=[eve_module1_id])


def test_not_loaded_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_module1_id = client.mk_eve_module(attrs={eve_use_attr_id: 5})
    eve_module2_id = client.mk_eve_module()
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(powergrid=True))
    assert api_stats.powergrid.one() == (0, None)
    api_fit.check_try_fit_items(
        val_options=ValOptions(powergrid=True),
        passing=[eve_module2_id],
        failing=[eve_module1_id])


def test_non_positive(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_module1_id = client.mk_eve_module(attrs={eve_use_attr_id: 0})
    eve_module2_id = client.mk_eve_module(attrs={eve_use_attr_id: 150})
    eve_module3_id = client.mk_eve_module(attrs={eve_use_attr_id: -10})
    eve_module4_id = client.mk_eve_module(attrs={eve_use_attr_id: 130})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module3_id, state=consts.ApiModuleState.online)
    # Verification - items with negative use reduce total use
    api_stats = api_fit.get_stats(options=FitStatsOptions(powergrid=True))
    assert api_stats.powergrid.one() == (-10, 125)
    api_fit.check_try_fit_items(
        val_options=ValOptions(powergrid=True),
        passing=[eve_module1_id, eve_module3_id, eve_module4_id],
        failing=[eve_module2_id])


def test_no_value_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_module1_id = client.mk_eve_module(attrs={eve_use_attr_id: 150})
    eve_module2_id = client.mk_eve_module()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(powergrid=True))
    assert api_stats.powergrid.one() == (0, 125)
    api_fit.check_try_fit_items(
        val_options=ValOptions(powergrid=True),
        passing=[eve_module2_id],
        failing=[eve_module1_id])


def test_no_value_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_module1_id = client.mk_eve_module(attrs={eve_use_attr_id: 150})
    eve_module2_id = client.mk_eve_module()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(powergrid=True))
    assert api_stats.powergrid.one() == (0, 0)
    api_fit.check_try_fit_items(
        val_options=ValOptions(powergrid=True),
        passing=[eve_module2_id],
        failing=[eve_module1_id])


def test_no_attr_use(client, consts):
    eve_use_attr_id = consts.EveAttr.power
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_module_id = client.mk_eve_module(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification - users are assumed to take no resource when attribute does not exist
    api_fit.check_try_fit_items(val_options=ValOptions(powergrid=True), passing=[eve_module_id], failing=[])


def test_no_attr_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_max_attr_id = consts.EveAttr.power_output
    eve_module1_id = client.mk_eve_module(attrs={eve_use_attr_id: 50})
    eve_module2_id = client.mk_eve_module()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification - when output attr does not exist, it is assumed to be 0
    api_fit.check_try_fit_items(
        val_options=ValOptions(powergrid=True),
        passing=[eve_module2_id],
        failing=[eve_module1_id])


def test_criterion_module_state(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_module1_id = client.mk_eve_module(attrs={eve_use_attr_id: 150})
    eve_module2_id = client.mk_eve_module(attrs={eve_use_attr_id: 100})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.offline)
    # Verification - offline modules do not use the resource
    api_stats = api_fit.get_stats(options=FitStatsOptions(powergrid=True))
    assert api_stats.powergrid.one() == (0, 125)
    api_fit.check_try_fit_items(
        val_options=ValOptions(powergrid=True),
        passing=[eve_module2_id],
        failing=[eve_module1_id])


def test_criterion_item_kind(client, consts):
    # Validation applies only to modules and services
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_booster_id = client.mk_eve_booster(attrs={eve_use_attr_id: 150})
    eve_charge_id = client.mk_eve_charge(attrs={eve_use_attr_id: 150})
    eve_drone_id = client.mk_eve_drone(attrs={eve_use_attr_id: 150})
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 150})
    eve_implant_id = client.mk_eve_implant(attrs={eve_use_attr_id: 150})
    eve_module_id = client.mk_eve_module(attrs={eve_use_attr_id: 150})
    eve_rig_id = client.mk_eve_rig(attrs={eve_use_attr_id: 150})
    eve_service_id = client.mk_eve_service(attrs={eve_use_attr_id: 150})
    eve_subsystem_id = client.mk_eve_subsystem(attrs={eve_use_attr_id: 150})
    eve_launcher_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125, eve_use_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(powergrid=True))
    assert api_stats.powergrid.one() == (0, 125)
    api_fit.check_try_fit_items(
        val_options=ValOptions(powergrid=True),
        passing=[
            eve_booster_id,
            eve_charge_id,
            eve_drone_id,
            eve_fighter_id,
            eve_implant_id,
            eve_rig_id,
            eve_subsystem_id],
        failing=[eve_module_id, eve_service_id])
