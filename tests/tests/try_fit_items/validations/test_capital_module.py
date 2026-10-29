from fw import approx
from fw.api import ValOptions


def test_main(client, consts):
    # Test regular module and capital volume on regular ship, capital ship and structure
    eve_skill_id = client.mk_eve_item(id_=consts.EveItem.capital_ships)
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_cap_module_id = client.mk_eve_module(attrs={eve_attr_id: 4000})
    eve_subcap_module_id = client.mk_eve_module(attrs={eve_attr_id: 50})
    eve_subcap_ship_id = client.mk_eve_ship()
    eve_cap_ship_id = client.mk_eve_ship(srqs={eve_skill_id: 1})
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_subcap_ship_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(capital_module=True),
        passing=[eve_subcap_module_id],
        failing=[eve_cap_module_id])
    # Action
    api_fit.set_ship(type_id=eve_cap_ship_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(capital_module=True),
        passing=[eve_cap_module_id, eve_subcap_module_id],
        failing=[])
    # Action
    api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(capital_module=True),
        passing=[eve_cap_module_id, eve_subcap_module_id],
        failing=[])
    # Action
    api_fit.set_ship(type_id=eve_subcap_ship_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(capital_module=True),
        passing=[eve_subcap_module_id],
        failing=[eve_cap_module_id])


def test_known_failures(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_cap_module_id = client.mk_eve_module(attrs={eve_attr_id: 4000})
    eve_subcap_module_id = client.mk_eve_module(attrs={eve_attr_id: 50})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_cap_module_id)
    api_module2 = api_fit.add_module(type_id=eve_cap_module_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(capital_module=[api_module1.id, api_module2.id]),
        passing=[eve_subcap_module_id],
        failing=[eve_cap_module_id])


def test_modified(client, consts):
    # Unrealistic scenario, but validation takes unmodified volume value
    eve_vol_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_module1_id = client.mk_eve_module(attrs={eve_vol_attr_id: 3000})
    eve_module2_id = client.mk_eve_module(attrs={eve_vol_attr_id: 4000})
    eve_ship_id = client.mk_eve_ship()
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_vol_attr_id)
    eve_implant_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1000}, eff_ids=[eve_implant_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module1_id)
    # Verification
    assert api_module.update().attrs[eve_vol_attr_id].modified == approx(4000)
    api_fit.check_try_fit_items(
        val_options=ValOptions(capital_module=True),
        passing=[eve_module1_id],
        failing=[eve_module2_id])
    # Action
    api_implant.remove()
    # Verification
    assert api_module.update().attrs[eve_vol_attr_id].modified == approx(3000)
    api_fit.check_try_fit_items(
        val_options=ValOptions(capital_module=True),
        passing=[eve_module1_id],
        failing=[eve_module2_id])


def test_no_ship(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_module_id = client.mk_eve_module(attrs={eve_attr_id: 4000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(capital_module=True), passing=[eve_module_id], failing=[])


def test_no_value(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_module_id = client.mk_eve_module()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(capital_module=True), passing=[eve_module_id], failing=[])


def test_no_skill(client, consts):
    eve_skill_id = consts.EveItem.capital_ships
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_module_id = client.mk_eve_module(attrs={eve_attr_id: 4000})
    eve_ship_id = client.mk_eve_ship(srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(capital_module=True), passing=[eve_module_id], failing=[])


def test_no_attr(client, consts):
    eve_attr_id = consts.EveAttr.volume
    eve_module_id = client.mk_eve_module(attrs={eve_attr_id: 4000})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification - validation passes if volume attribute does not exist
    api_fit.check_try_fit_items(val_options=ValOptions(capital_module=True), passing=[eve_module_id], failing=[])


def test_not_loaded_ship(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_module_id = client.mk_eve_module(attrs={eve_attr_id: 4000})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(capital_module=True), passing=[eve_module_id], failing=[])


def test_criterion_volume(client, consts):
    # Threshold for capital modules is >3500. The value has most likely been taken from EVE client
    # source, but right now can't say for sure if it's from there, and when it was
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_cap_module_id = client.mk_eve_module(attrs={eve_attr_id: 3500.1})
    eve_subcap_module_id = client.mk_eve_module(attrs={eve_attr_id: 3500})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(capital_module=True),
        passing=[eve_subcap_module_id],
        failing=[eve_cap_module_id])


def test_criterion_item_category(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_ship_module_id = client.mk_eve_module(attrs={eve_attr_id: 4000})
    eve_struct_module_id = client.mk_eve_module(cat_id=consts.EveItemCat.structure_module, attrs={eve_attr_id: 4000})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(capital_module=True),
        passing=[eve_struct_module_id],
        failing=[eve_ship_module_id])


def test_criterion_item_kind(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_booster_id = client.mk_eve_booster(attrs={eve_attr_id: 4000})
    eve_charge_id = client.mk_eve_charge(attrs={eve_attr_id: 4000})
    eve_drone_id = client.mk_eve_drone(attrs={eve_attr_id: 4000})
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_attr_id: 4000})
    eve_implant_id = client.mk_eve_implant(attrs={eve_attr_id: 4000})
    eve_module_id = client.mk_eve_module(attrs={eve_attr_id: 4000})
    eve_rig_id = client.mk_eve_rig(attrs={eve_attr_id: 4000})
    eve_service_id = client.mk_eve_service(attrs={eve_attr_id: 4000})
    eve_subsystem_id = client.mk_eve_subsystem(attrs={eve_attr_id: 4000})
    eve_launcher_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_launcher_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(capital_module=True),
        passing=[
            eve_booster_id,
            eve_charge_id,
            eve_drone_id,
            eve_fighter_id,
            eve_implant_id,
            eve_rig_id,
            eve_service_id,
            eve_subsystem_id],
        failing=[eve_module_id])
