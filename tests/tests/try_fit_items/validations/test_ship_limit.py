from fw import approx
from fw.api import ValOptions


def test_type_single(client, consts):
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_module_id = client.mk_eve_module(attrs={eve_type_attr_id: eve_allowed_ship_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[], failing=[eve_module_id])
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[eve_module_id], failing=[])


def test_type_multiple_different(client, consts):
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_type_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type2, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship1_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_allowed_ship2_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_module_id = client.mk_eve_module(
        attrs={eve_type_attr1_id: eve_allowed_ship1_id, eve_type_attr2_id: eve_allowed_ship2_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[], failing=[eve_module_id])
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship2_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[eve_module_id], failing=[])


def test_type_multiple_same_rounding(client, consts):
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_type_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type2, unit_id=consts.EveAttrUnit.item_id)
    eve_type_attr3_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type3, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship1_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_allowed_ship2_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_module_id = client.mk_eve_module(attrs={
        eve_type_attr1_id: eve_allowed_ship1_id - 0.4,
        eve_type_attr2_id: eve_allowed_ship2_id,
        eve_type_attr3_id: eve_allowed_ship1_id + 0.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[], failing=[eve_module_id])
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship1_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[eve_module_id], failing=[])


def test_group_single(client, consts):
    eve_allowed_grp_id = client.mk_eve_ship_group()
    eve_disallowed_grp_id = client.mk_eve_ship_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_allowed_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_disallowed_grp_id)
    eve_module_id = client.mk_eve_module(attrs={eve_group_attr_id: eve_allowed_grp_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[], failing=[eve_module_id])
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[eve_module_id], failing=[])


def test_group_multiple_different(client, consts):
    eve_allowed_grp1_id = client.mk_eve_ship_group()
    eve_allowed_grp2_id = client.mk_eve_ship_group()
    eve_disallowed_grp_id = client.mk_eve_ship_group()
    eve_group_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group2, unit_id=consts.EveAttrUnit.group_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_allowed_grp2_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_disallowed_grp_id)
    eve_module_id = client.mk_eve_module(
        attrs={eve_group_attr1_id: eve_allowed_grp1_id, eve_group_attr2_id: eve_allowed_grp2_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[], failing=[eve_module_id])
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[eve_module_id], failing=[])


def test_group_multiple_same_rounding(client, consts):
    eve_allowed_grp1_id = client.mk_eve_ship_group()
    eve_allowed_grp2_id = client.mk_eve_ship_group()
    eve_disallowed_grp_id = client.mk_eve_ship_group()
    eve_group_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group2, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr3_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group3, unit_id=consts.EveAttrUnit.group_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_allowed_grp1_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_disallowed_grp_id)
    eve_module_id = client.mk_eve_module(attrs={
        eve_group_attr1_id: eve_allowed_grp1_id - 0.4,
        eve_group_attr2_id: eve_allowed_grp2_id,
        eve_group_attr3_id: eve_allowed_grp1_id + 0.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[], failing=[eve_module_id])
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[eve_module_id], failing=[])


def test_combined(client, consts):
    eve_allowed_grp1_id = client.mk_eve_ship_group()
    eve_allowed_grp2_id = client.mk_eve_ship_group()
    eve_disallowed_grp_id = client.mk_eve_ship_group()
    eve_type_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type8, unit_id=consts.EveAttrUnit.item_id)
    eve_type_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type10, unit_id=consts.EveAttrUnit.item_id)
    eve_group_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group4, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group9, unit_id=consts.EveAttrUnit.group_id)
    eve_allowed_type_ship_id = client.mk_eve_ship(grp_id=eve_disallowed_grp_id)
    eve_allowed_group_ship_id = client.mk_eve_ship(grp_id=eve_allowed_grp2_id)
    eve_allowed_both_ship_id = client.mk_eve_ship(grp_id=eve_allowed_grp1_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_disallowed_grp_id)
    eve_module_id = client.mk_eve_module(attrs={
        eve_type_attr1_id: eve_allowed_type_ship_id,
        eve_type_attr2_id: eve_allowed_both_ship_id,
        eve_group_attr1_id: eve_allowed_grp1_id,
        eve_group_attr2_id: eve_allowed_grp2_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[], failing=[eve_module_id])
    # Action
    api_fit.set_ship(type_id=eve_allowed_type_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[eve_module_id], failing=[])
    # Action
    api_fit.set_ship(type_id=eve_allowed_group_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[eve_module_id], failing=[])
    # Action
    api_fit.set_ship(type_id=eve_allowed_both_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[eve_module_id], failing=[])


def test_struct(client, consts):
    eve_struct_grp_id = client.mk_eve_struct_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_struct_id = client.mk_eve_struct(grp_id=eve_struct_grp_id)
    eve_disallowed_struct_id = client.mk_eve_struct(grp_id=eve_struct_grp_id)
    eve_module_id = client.mk_eve_module(
        cat_id=consts.EveItemCat.structure_module,
        attrs={eve_type_attr_id: eve_allowed_struct_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_struct_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[], failing=[eve_module_id])
    # Action
    api_fit.set_ship(type_id=eve_allowed_struct_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[eve_module_id], failing=[])


def test_rig(client, consts):
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type12, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_rig_id = client.mk_eve_rig(attrs={eve_type_attr_id: eve_allowed_ship_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[], failing=[eve_rig_id])
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[eve_rig_id], failing=[])


def test_service(client, consts):
    eve_struct_grp_id = client.mk_eve_struct_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type11, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_struct_id = client.mk_eve_struct(grp_id=eve_struct_grp_id)
    eve_disallowed_struct_id = client.mk_eve_struct(grp_id=eve_struct_grp_id)
    eve_service_id = client.mk_eve_service(attrs={eve_type_attr_id: eve_allowed_struct_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_struct_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[], failing=[eve_service_id])
    # Action
    api_fit.set_ship(type_id=eve_allowed_struct_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[eve_service_id], failing=[])


def test_subsystem(client, consts):
    # Subsystems have their special limit attribute
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.fits_to_ship_type, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_subsystem_id = client.mk_eve_subsystem(attrs={eve_type_attr_id: eve_allowed_ship_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[], failing=[eve_subsystem_id])
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[eve_subsystem_id], failing=[])


def test_known_failures(client, consts):
    eve_grp1_id = client.mk_eve_ship_group()
    eve_grp2_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type9, unit_id=consts.EveAttrUnit.item_id)
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group18, unit_id=consts.EveAttrUnit.group_id)
    eve_ship_id = client.mk_eve_ship(grp_id=eve_grp1_id)
    eve_other_ship_id = client.mk_eve_ship(grp_id=eve_grp2_id)
    eve_limited_module_id = client.mk_eve_module(
        attrs={eve_type_attr_id: eve_other_ship_id, eve_group_attr_id: eve_grp2_id})
    eve_unlimited_module_id = client.mk_eve_module()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_limited_module_id)
    api_module2 = api_fit.add_module(type_id=eve_limited_module_id)
    # Verification - no ship case has to be checked as well, since there is no-ship logic
    api_fit.check_try_fit_items(
        val_options=ValOptions(ship_limit=[api_module1.id]),
        passing=[],
        failing=[eve_unlimited_module_id, eve_limited_module_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(ship_limit=[api_module2.id]),
        passing=[],
        failing=[eve_unlimited_module_id, eve_limited_module_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(ship_limit=[api_module1.id, api_module2.id]),
        passing=[eve_unlimited_module_id],
        failing=[eve_limited_module_id])
    # Action
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(ship_limit=[api_module1.id]),
        passing=[],
        failing=[eve_unlimited_module_id, eve_limited_module_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(ship_limit=[api_module2.id]),
        passing=[],
        failing=[eve_unlimited_module_id, eve_limited_module_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(ship_limit=[api_module1.id, api_module2.id]),
        passing=[eve_unlimited_module_id],
        failing=[eve_limited_module_id])


def test_modified_type(client, consts):
    # Verification - unrealistic scenario, but testing here detail of implementation: raw values of
    # attributes are taken, their modification does not affect validation
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_module_id = client.mk_eve_module(attrs={eve_type_attr_id: eve_allowed_ship_id})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_type_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: eve_disallowed_ship_id}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[], failing=[eve_module_id])
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    assert api_module.update().attrs[eve_type_attr_id].modified == approx(eve_disallowed_ship_id)
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[eve_module_id], failing=[])


def test_modified_group(client, consts):
    # Verification - unrealistic scenario, but testing here detail of implementation: raw values of
    # attributes are taken, their modification does not affect validation
    eve_allowed_grp_id = client.mk_eve_ship_group()
    eve_disallowed_grp_id = client.mk_eve_ship_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_allowed_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_disallowed_grp_id)
    eve_module_id = client.mk_eve_module(attrs={eve_group_attr_id: eve_allowed_grp_id})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_group_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: eve_disallowed_grp_id}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[], failing=[eve_module_id])
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    assert api_module.update().attrs[eve_group_attr_id].modified == approx(eve_disallowed_grp_id)
    api_fit.check_try_fit_items(val_options=ValOptions(ship_limit=True), passing=[eve_module_id], failing=[])


def test_no_ship(client, consts):
    eve_ship_grp1_id = client.mk_eve_ship_group()
    eve_ship_grp2_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp2_id)
    eve_limited_module_id = client.mk_eve_module(
        attrs={eve_type_attr_id: eve_ship_id, eve_group_attr_id: eve_ship_grp1_id})
    eve_unlimited_module_id = client.mk_eve_module()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(ship_limit=True),
        passing=[eve_unlimited_module_id],
        failing=[eve_limited_module_id])


def test_no_attr(client, consts):
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr1_id = consts.EveAttr.can_fit_ship_type1
    eve_type_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type2, unit_id=consts.EveAttrUnit.item_id)
    eve_ship1_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_ship2_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_module1_id = client.mk_eve_module(attrs={eve_type_attr1_id: eve_ship1_id})
    eve_module2_id = client.mk_eve_module(attrs={eve_type_attr1_id: eve_ship1_id, eve_type_attr2_id: eve_ship2_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship1_id)
    # Verification - when the only defined limit attribute doesn't exist, the lib treats item as
    # having no ship limit. One attribute not existing doesn't prevent other attributes from
    # defining the ship limit
    api_fit.check_try_fit_items(
        val_options=ValOptions(ship_limit=True),
        passing=[eve_module1_id],
        failing=[eve_module2_id])


def test_not_loaded_ship(client, consts):
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_ship1_id = client.alloc_item_id()
    eve_ship2_id = client.alloc_item_id()
    eve_module1_id = client.mk_eve_module(attrs={eve_type_attr_id: eve_ship1_id})
    eve_module2_id = client.mk_eve_module(attrs={eve_type_attr_id: eve_ship2_id})
    eve_module3_id = client.mk_eve_module(attrs={eve_type_attr_id: eve_ship1_id, eve_group_attr_id: eve_ship_grp_id})
    eve_module4_id = client.mk_eve_module(attrs={eve_type_attr_id: eve_ship2_id, eve_group_attr_id: eve_ship_grp_id})
    eve_module5_id = client.mk_eve_module(attrs={eve_group_attr_id: eve_ship_grp_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification - when ship is not loaded, we fail validation only we 100% know it will fail with
    # the info we have.
    # - Module 1 fails because item type is mismatched
    # - Module 2 passes because item type is matched
    # - Module 3 fails because type is mismatched, and ship group is not available
    # - Module 4 passes because item type is matched, even if ship group is not available
    # - Module 5 fails because ship group is not available
    api_fit.check_try_fit_items(
        val_options=ValOptions(ship_limit=True),
        passing=[eve_module2_id, eve_module4_id],
        failing=[eve_module1_id, eve_module3_id, eve_module5_id])


def test_criterion_item_kind(client, consts):
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_ship_id = client.mk_eve_ship()
    eve_other_ship_id = client.mk_eve_ship()
    eve_booster_id = client.mk_eve_booster(attrs={eve_type_attr_id: eve_other_ship_id})
    eve_charge_id = client.mk_eve_charge(attrs={eve_type_attr_id: eve_other_ship_id})
    eve_drone_id = client.mk_eve_drone(attrs={eve_type_attr_id: eve_other_ship_id})
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_type_attr_id: eve_other_ship_id})
    eve_implant_id = client.mk_eve_implant(attrs={eve_type_attr_id: eve_other_ship_id})
    eve_module_id = client.mk_eve_module(attrs={eve_type_attr_id: eve_other_ship_id})
    eve_rig_id = client.mk_eve_rig(attrs={eve_type_attr_id: eve_other_ship_id})
    eve_service_id = client.mk_eve_service(attrs={eve_type_attr_id: eve_other_ship_id})
    eve_subsystem_id = client.mk_eve_subsystem(attrs={eve_type_attr_id: eve_other_ship_id})
    eve_launcher_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_launcher_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(ship_limit=True),
        passing=[eve_booster_id, eve_charge_id, eve_drone_id, eve_fighter_id, eve_implant_id],
        failing=[eve_module_id, eve_rig_id, eve_service_id, eve_subsystem_id])
