from fw import approx
from fw.api import ValOptions


def test_ship(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig1_id = client.mk_eve_rig(attrs={eve_attr_id: 1})
    eve_rig2_id = client.mk_eve_rig(attrs={eve_attr_id: 3})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(rig_size=True), passing=[eve_rig2_id], failing=[eve_rig1_id])


def test_struct(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig1_id = client.mk_eve_rig(attrs={eve_attr_id: 1})
    eve_rig2_id = client.mk_eve_rig(attrs={eve_attr_id: 3})
    eve_struct_id = client.mk_eve_struct(attrs={eve_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(rig_size=True), passing=[eve_rig2_id], failing=[eve_rig1_id])


def test_known_failures(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig1_id = client.mk_eve_rig(attrs={eve_attr_id: 1})
    eve_rig2_id = client.mk_eve_rig(attrs={eve_attr_id: 3})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig1_id)
    api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(rig_size=[api_rig1.id, api_rig2.id]),
        passing=[eve_rig2_id],
        failing=[eve_rig1_id])


def test_rounding(client, consts):
    # Unrealistic scenario - EVE rig size has only integer value. But here we check that no rounding
    # happens, just straight comparison
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig1_id = client.mk_eve_rig(attrs={eve_attr_id: 1.2})
    eve_rig2_id = client.mk_eve_rig(attrs={eve_attr_id: 2.9})
    eve_rig3_id = client.mk_eve_rig(attrs={eve_attr_id: 2.95})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 2.95})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(rig_size=True),
        passing=[eve_rig3_id],
        failing=[eve_rig1_id, eve_rig2_id])


def test_modified(client, consts):
    # Unrealistic scenario since EVE rig size is never modified, we check that unmodified values are
    # taken as detail of implementation
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_rig1_id = client.mk_eve_rig(attrs={eve_attr_id: 1})
    eve_rig2_id = client.mk_eve_rig(attrs={eve_attr_id: 3})
    eve_rig3_id = client.mk_eve_rig(attrs={eve_attr_id: 2})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 3})
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod1, eve_mod2])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 2}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    assert api_ship.update().attrs[eve_attr_id].modified == approx(2)
    assert api_rig.update().attrs[eve_attr_id].modified == approx(2)
    api_fit.check_try_fit_items(
        val_options=ValOptions(rig_size=True),
        passing=[eve_rig2_id],
        failing=[eve_rig1_id, eve_rig3_id])


def test_ship_absent(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig_id = client.mk_eve_rig(attrs={eve_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(rig_size=True), passing=[eve_rig_id], failing=[])


def test_non_positive(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig1_id = client.mk_eve_rig(attrs={eve_attr_id: 0})
    eve_rig2_id = client.mk_eve_rig(attrs={eve_attr_id: -3})
    eve_ship1_id = client.mk_eve_ship(attrs={eve_attr_id: 0})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_attr_id: -3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(rig_size=True), passing=[eve_rig1_id], failing=[eve_rig2_id])
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(rig_size=True), passing=[eve_rig2_id], failing=[eve_rig1_id])


def test_no_value_ship(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig_id = client.mk_eve_rig(attrs={eve_attr_id: 2})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(rig_size=True), passing=[eve_rig_id], failing=[])


def test_no_value_rig(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig_id = client.mk_eve_rig()
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(rig_size=True), passing=[], failing=[eve_rig_id])


def test_no_attr(client, consts):
    eve_attr_id = consts.EveAttr.rig_size
    eve_rig_id = client.mk_eve_rig(attrs={eve_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification - no attribute means values can't be fetched for comparison, which means
    # validation always passes
    api_fit.check_try_fit_items(val_options=ValOptions(rig_size=True), passing=[eve_rig_id], failing=[])


def test_ship_not_loaded(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig_id = client.mk_eve_rig(attrs={eve_attr_id: 1})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(rig_size=True), passing=[eve_rig_id], failing=[])


def test_criterion_item_kind(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 1})
    eve_booster_id = client.mk_eve_booster(attrs={eve_attr_id: 2})
    eve_charge_id = client.mk_eve_charge(attrs={eve_attr_id: 3})
    eve_drone_id = client.mk_eve_drone(attrs={eve_attr_id: 4})
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_attr_id: 5})
    eve_implant_id = client.mk_eve_implant(attrs={eve_attr_id: 6})
    eve_module_id = client.mk_eve_module(attrs={eve_attr_id: 7})
    eve_rig_id = client.mk_eve_rig(attrs={eve_attr_id: 8})
    eve_service_id = client.mk_eve_service(attrs={eve_attr_id: 9})
    eve_subsystem_id = client.mk_eve_subsystem(attrs={eve_attr_id: 10})
    eve_launcher_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_launcher_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(rig_size=True),
        passing=[
            eve_booster_id,
            eve_charge_id,
            eve_drone_id,
            eve_fighter_id,
            eve_implant_id,
            eve_module_id,
            eve_service_id,
            eve_subsystem_id],
        failing=[eve_rig_id])
