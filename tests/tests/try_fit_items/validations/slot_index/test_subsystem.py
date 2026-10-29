from fw import approx
from fw.api import ValOptions


def test_multiple(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem1_id = client.mk_eve_subsystem(attrs={eve_slot_attr_id: 1})
    eve_subsystem2_id = client.mk_eve_subsystem(attrs={eve_slot_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_subsystem(type_id=eve_subsystem1_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(subsystem_slot_index=True),
        passing=[eve_subsystem2_id],
        failing=[eve_subsystem1_id])


def test_different_slots(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem1_id = client.mk_eve_subsystem(attrs={eve_slot_attr_id: 1})
    eve_subsystem2_id = client.mk_eve_subsystem(attrs={eve_slot_attr_id: 2})
    eve_subsystem3_id = client.mk_eve_subsystem(attrs={eve_slot_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_subsystem(type_id=eve_subsystem1_id)
    api_fit.add_subsystem(type_id=eve_subsystem2_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(subsystem_slot_index=True),
        passing=[eve_subsystem3_id],
        failing=[eve_subsystem1_id, eve_subsystem2_id])


def test_known_failures(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem1_id = client.mk_eve_subsystem(attrs={eve_slot_attr_id: 1})
    eve_subsystem2_id = client.mk_eve_subsystem(attrs={eve_slot_attr_id: 2})
    eve_subsystem3_id = client.mk_eve_subsystem(attrs={eve_slot_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_subsystem1 = api_fit.add_subsystem(type_id=eve_subsystem1_id)
    api_fit.add_subsystem(type_id=eve_subsystem2_id)
    api_subsystem3 = api_fit.add_subsystem(type_id=eve_subsystem1_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(subsystem_slot_index=[api_subsystem1.id]),
        passing=[],
        failing=[eve_subsystem3_id, eve_subsystem1_id, eve_subsystem2_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(subsystem_slot_index=[api_subsystem3.id]),
        passing=[],
        failing=[eve_subsystem3_id, eve_subsystem1_id, eve_subsystem2_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(subsystem_slot_index=[api_subsystem1.id, api_subsystem3.id]),
        passing=[eve_subsystem3_id],
        failing=[eve_subsystem1_id, eve_subsystem2_id])


def test_rounding(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem1_id = client.mk_eve_subsystem(attrs={eve_slot_attr_id: 1.2})
    eve_subsystem2_id = client.mk_eve_subsystem(attrs={eve_slot_attr_id: 1.4})
    eve_subsystem3_id = client.mk_eve_subsystem(attrs={eve_slot_attr_id: 1.6})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_subsystem(type_id=eve_subsystem1_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(subsystem_slot_index=True),
        passing=[eve_subsystem3_id],
        failing=[eve_subsystem2_id])


def test_modified_index(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem1_id = client.mk_eve_subsystem(attrs={eve_slot_attr_id: 2})
    eve_subsystem2_id = client.mk_eve_subsystem(attrs={eve_slot_attr_id: 3})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_slot_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 3}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_subsystem = api_fit.add_subsystem(type_id=eve_subsystem1_id)
    # Verification
    assert api_subsystem.update().attrs[eve_slot_attr_id].modified == approx(2)
    api_fit.check_try_fit_items(
        val_options=ValOptions(subsystem_slot_index=True),
        passing=[eve_subsystem2_id],
        failing=[eve_subsystem1_id])
    # Action
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification - attribute is modified, but not for purposes of validation
    assert api_subsystem.update().attrs[eve_slot_attr_id].modified == approx(3)
    api_fit.check_try_fit_items(
        val_options=ValOptions(subsystem_slot_index=True),
        passing=[eve_subsystem2_id],
        failing=[eve_subsystem1_id])


def test_no_value(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem_id = client.mk_eve_subsystem()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(subsystem_slot_index=True),
        passing=[eve_subsystem_id],
        failing=[])


def test_no_attr(client, consts):
    eve_slot_attr_id = consts.EveAttr.subsystem_slot
    eve_subsystem_id = client.mk_eve_subsystem(attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification - validation passes if attr does not exist
    api_fit.check_try_fit_items(
        val_options=ValOptions(subsystem_slot_index=True),
        passing=[eve_subsystem_id],
        failing=[])


def test_criterion_subsystem_state(client, consts):
    # Disabled subsystems still compete for slots
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem_id = client.mk_eve_subsystem(attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_subsystem(type_id=eve_subsystem_id, state=False)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(subsystem_slot_index=True),
        passing=[],
        failing=[eve_subsystem_id])


def test_criterion_item_kind(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_subsystem_id = client.mk_eve_subsystem(attrs={eve_slot_attr_id: 1})
    eve_booster_id = client.mk_eve_booster(attrs={eve_slot_attr_id: 1})
    eve_charge_id = client.mk_eve_charge(attrs={eve_slot_attr_id: 1})
    eve_drone_id = client.mk_eve_drone(attrs={eve_slot_attr_id: 1})
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_slot_attr_id: 1})
    eve_implant_id = client.mk_eve_implant(attrs={eve_slot_attr_id: 1})
    eve_module_id = client.mk_eve_module(attrs={eve_slot_attr_id: 1})
    eve_rig_id = client.mk_eve_rig(attrs={eve_slot_attr_id: 1})
    eve_service_id = client.mk_eve_service(attrs={eve_slot_attr_id: 1})
    eve_launcher_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_subsystem(type_id=eve_subsystem_id)
    api_fit.add_module(type_id=eve_launcher_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(subsystem_slot_index=True),
        passing=[
            eve_booster_id,
            eve_charge_id,
            eve_drone_id,
            eve_fighter_id,
            eve_implant_id,
            eve_module_id,
            eve_rig_id,
            eve_service_id],
        failing=[eve_subsystem_id])
