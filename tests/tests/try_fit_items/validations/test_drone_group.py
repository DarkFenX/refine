from fw import approx
from fw.api import ValOptions


def test_drone_add_remove(client, consts):
    # Also check matching against multiple allowed groups
    eve_group1_id = client.mk_eve_drone_group()
    eve_group2_id = client.mk_eve_drone_group()
    eve_group3_id = client.mk_eve_drone_group()
    eve_limit_attr1_id = client.mk_eve_attr(
        id_=consts.EveAttr.allowed_drone_group1,
        unit_id=consts.EveAttrUnit.group_id)
    eve_limit_attr2_id = client.mk_eve_attr(
        id_=consts.EveAttr.allowed_drone_group2,
        unit_id=consts.EveAttrUnit.group_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_limit_attr1_id: eve_group1_id, eve_limit_attr2_id: eve_group2_id})
    eve_drone1_id = client.mk_eve_drone(grp_id=eve_group1_id)
    eve_drone2_id = client.mk_eve_drone(grp_id=eve_group2_id)
    eve_drone3_id = client.mk_eve_drone(grp_id=eve_group3_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_group=True),
        passing=[eve_drone1_id, eve_drone2_id],
        failing=[eve_drone3_id])


def test_ship_add_set_remove(client, consts):
    eve_group1_id = client.mk_eve_drone_group()
    eve_group2_id = client.mk_eve_drone_group()
    eve_group3_id = client.mk_eve_drone_group()
    eve_limit_attr1_id = client.mk_eve_attr(
        id_=consts.EveAttr.allowed_drone_group1,
        unit_id=consts.EveAttrUnit.group_id)
    eve_limit_attr2_id = client.mk_eve_attr(
        id_=consts.EveAttr.allowed_drone_group2,
        unit_id=consts.EveAttrUnit.group_id)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_limit_attr1_id: eve_group1_id})
    eve_ship2_id = client.mk_eve_ship()
    eve_ship3_id = client.mk_eve_ship(attrs={eve_limit_attr2_id: eve_group2_id})
    eve_drone_id = client.mk_eve_drone(grp_id=eve_group3_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(drone_group=True), passing=[eve_drone_id], failing=[])
    # Action
    api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(drone_group=True), passing=[], failing=[eve_drone_id])
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(drone_group=True), passing=[eve_drone_id], failing=[])
    # Action
    api_fit.set_ship(type_id=eve_ship3_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(drone_group=True), passing=[], failing=[eve_drone_id])
    # Action
    api_fit.unset_ship()
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(drone_group=True), passing=[eve_drone_id], failing=[])


def test_rounding(client, consts):
    eve_group1_id = client.mk_eve_drone_group()
    eve_group2_id = client.mk_eve_drone_group()
    eve_limit_attr1_id = client.mk_eve_attr(
        id_=consts.EveAttr.allowed_drone_group1,
        unit_id=consts.EveAttrUnit.group_id)
    eve_limit_attr2_id = client.mk_eve_attr(
        id_=consts.EveAttr.allowed_drone_group2,
        unit_id=consts.EveAttrUnit.group_id)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_limit_attr1_id: eve_group1_id - 0.4, eve_limit_attr2_id: eve_group1_id + 0.4})
    eve_drone1_id = client.mk_eve_drone(grp_id=eve_group1_id)
    eve_drone2_id = client.mk_eve_drone(grp_id=eve_group2_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_group=True),
        passing=[eve_drone1_id],
        failing=[eve_drone2_id])


def test_known_failures(client, consts):
    eve_group1_id = client.mk_eve_drone_group()
    eve_group2_id = client.mk_eve_drone_group()
    eve_group3_id = client.mk_eve_drone_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allowed_drone_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_limit_attr_id: eve_group1_id})
    eve_drone1_id = client.mk_eve_drone(grp_id=eve_group1_id)
    eve_drone2_id = client.mk_eve_drone(grp_id=eve_group2_id)
    eve_drone3_id = client.mk_eve_drone(grp_id=eve_group3_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_bay)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_bay)
    api_drone3 = api_fit.add_drone(type_id=eve_drone3_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_group=[api_drone2.id, api_drone3.id]),
        passing=[eve_drone1_id],
        failing=[eve_drone2_id, eve_drone3_id])


def test_modified_limit(client, consts):
    eve_group1_id = client.mk_eve_drone_group()
    eve_group2_id = client.mk_eve_drone_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allowed_drone_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_limit_attr_id: eve_group1_id})
    eve_drone1_id = client.mk_eve_drone(grp_id=eve_group1_id)
    eve_drone2_id = client.mk_eve_drone(grp_id=eve_group2_id)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_limit_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: eve_group2_id}, eff_ids=[eve_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    assert api_ship.update().attrs[eve_limit_attr_id].modified == approx(eve_group1_id)
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_group=True),
        passing=[eve_drone1_id],
        failing=[eve_drone2_id])
    # Action
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification - attribute is modified, but not for purposes of validation
    assert api_ship.update().attrs[eve_limit_attr_id].modified == approx(eve_group2_id)
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_group=True),
        passing=[eve_drone1_id],
        failing=[eve_drone2_id])


def test_no_attr(client, consts):
    eve_group1_id = client.mk_eve_drone_group()
    eve_group2_id = client.mk_eve_drone_group()
    eve_limit_attr1_id = consts.EveAttr.allowed_drone_group1
    eve_limit_attr2_id = client.mk_eve_attr(
        id_=consts.EveAttr.allowed_drone_group2,
        unit_id=consts.EveAttrUnit.group_id)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_limit_attr1_id: eve_group2_id})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_limit_attr1_id: eve_group1_id, eve_limit_attr2_id: eve_group2_id})
    eve_drone_id = client.mk_eve_drone(grp_id=eve_group1_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    # Verification - limits defined with attribute which doesn't exist are ignored. Without it,
    # ship has no limit, so validation passes
    api_fit.check_try_fit_items(val_options=ValOptions(drone_group=True), passing=[eve_drone_id], failing=[])
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification - having some attributes which do not exist does not prevent existing attributes
    # from functioning
    api_fit.check_try_fit_items(val_options=ValOptions(drone_group=True), passing=[], failing=[eve_drone_id])


def test_not_loaded_ship(client):
    eve_ship_id = client.alloc_item_id()
    eve_drone_id = client.mk_eve_drone()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(drone_group=True), passing=[eve_drone_id], failing=[])


def test_criterion_item_kind(client, consts):
    eve_drone_group1_id = client.mk_eve_drone_group()
    eve_drone_group2_id = client.mk_eve_drone_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allowed_drone_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_limit_attr_id: eve_drone_group1_id})
    eve_booster_id = client.mk_eve_booster()
    eve_charge_id = client.mk_eve_charge()
    eve_drone_id = client.mk_eve_drone(grp_id=eve_drone_group2_id)
    eve_fighter_id = client.mk_eve_fighter()
    eve_implant_id = client.mk_eve_implant()
    eve_module_id = client.mk_eve_module()
    eve_rig_id = client.mk_eve_rig()
    eve_service_id = client.mk_eve_service()
    eve_subsystem_id = client.mk_eve_subsystem()
    eve_launcher_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_launcher_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_group=True),
        passing=[
            eve_booster_id,
            eve_charge_id,
            eve_fighter_id,
            eve_implant_id,
            eve_module_id,
            eve_rig_id,
            eve_service_id,
            eve_subsystem_id],
        failing=[eve_drone_id])
