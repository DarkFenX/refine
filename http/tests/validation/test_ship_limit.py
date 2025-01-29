from tests import check_no_field


def test_type_single(client, consts):
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_module_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_allowed_ship_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.mismatches == {api_module.id: ([eve_allowed_ship_id], [])}
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_type_multiple_different(client, consts):
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_type_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type2, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship1_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_allowed_ship2_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_type_attr1_id: eve_allowed_ship1_id, eve_type_attr2_id: eve_allowed_ship2_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.mismatches == {
        api_module.id: (sorted([eve_allowed_ship1_id, eve_allowed_ship2_id]), [])}
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_type_multiple_same(client, consts):
    # Test rounding as well
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_type_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type2, unit_id=consts.EveAttrUnit.item_id)
    eve_type_attr3_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type3, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship1_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_allowed_ship2_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_module_id = client.mk_eve_item(attrs={
        eve_type_attr1_id: eve_allowed_ship1_id - 0.4,
        eve_type_attr2_id: eve_allowed_ship2_id,
        eve_type_attr3_id: eve_allowed_ship1_id + 0.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.mismatches == {
        api_module.id: (sorted([eve_allowed_ship1_id, eve_allowed_ship2_id]), [])}
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship1_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_group_single(client, consts):
    eve_allowed_grp_id = client.mk_eve_ship_group()
    eve_disallowed_grp_id = client.mk_eve_ship_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_allowed_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_disallowed_grp_id)
    eve_module_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_allowed_grp_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_disallowed_grp_id
    assert api_val.details.ship_limit.mismatches == {api_module.id: ([], [eve_allowed_grp_id])}
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_group_multiple_different(client, consts):
    eve_allowed_grp1_id = client.mk_eve_ship_group()
    eve_allowed_grp2_id = client.mk_eve_ship_group()
    eve_disallowed_grp_id = client.mk_eve_ship_group()
    eve_group_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group2, unit_id=consts.EveAttrUnit.group_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_allowed_grp2_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_disallowed_grp_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_group_attr1_id: eve_allowed_grp1_id, eve_group_attr2_id: eve_allowed_grp2_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_disallowed_grp_id
    assert api_val.details.ship_limit.mismatches == {
        api_module.id: ([], sorted([eve_allowed_grp1_id, eve_allowed_grp2_id]))}
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_group_multiple_same(client, consts):
    # Test rounding as well
    eve_allowed_grp1_id = client.mk_eve_ship_group()
    eve_allowed_grp2_id = client.mk_eve_ship_group()
    eve_disallowed_grp_id = client.mk_eve_ship_group()
    eve_group_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group2, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr3_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group3, unit_id=consts.EveAttrUnit.group_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_allowed_grp1_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_disallowed_grp_id)
    eve_module_id = client.mk_eve_item(attrs={
        eve_group_attr1_id: eve_allowed_grp1_id - 0.4,
        eve_group_attr2_id: eve_allowed_grp2_id,
        eve_group_attr3_id: eve_allowed_grp1_id + 0.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_disallowed_grp_id
    assert api_val.details.ship_limit.mismatches == {
        api_module.id: ([], sorted([eve_allowed_grp1_id, eve_allowed_grp2_id]))}
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


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
    eve_module_id = client.mk_eve_item(attrs={
        eve_type_attr1_id: eve_allowed_type_ship_id,
        eve_type_attr2_id: eve_allowed_both_ship_id,
        eve_group_attr1_id: eve_allowed_grp1_id,
        eve_group_attr2_id: eve_allowed_grp2_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_disallowed_grp_id
    assert api_val.details.ship_limit.mismatches == {api_module.id: (
        sorted([eve_allowed_type_ship_id, eve_allowed_both_ship_id]),
        sorted([eve_allowed_grp1_id, eve_allowed_grp2_id]))}
    # Action
    api_fit.set_ship(type_id=eve_allowed_type_ship_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_fit.set_ship(type_id=eve_allowed_group_ship_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_fit.set_ship(type_id=eve_allowed_both_ship_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_struct(client, consts):
    eve_struct_grp_id = client.mk_eve_struct_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_struct_id = client.mk_eve_struct(grp_id=eve_struct_grp_id)
    eve_disallowed_struct_id = client.mk_eve_struct(grp_id=eve_struct_grp_id)
    eve_module_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_allowed_struct_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_struct_id)
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_struct_id
    assert api_val.details.ship_limit.ship_group_id == eve_struct_grp_id
    assert api_val.details.ship_limit.mismatches == {api_module.id: ([eve_allowed_struct_id], [])}
    # Action
    api_fit.set_ship(type_id=eve_allowed_struct_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_subsystem(client, consts):
    # Subsystems have their special limit attribute
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.fits_to_ship_type, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_subsystem_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_allowed_ship_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_subsystem = api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.mismatches == {api_subsystem.id: ([eve_allowed_ship_id], [])}
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_rig(client, consts):
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_rig_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_allowed_ship_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.mismatches == {api_rig.id: ([eve_allowed_ship_id], [])}
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_no_ship(client, consts):
    eve_ship_grp1_id = client.mk_eve_ship_group()
    eve_ship_grp2_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp2_id)
    eve_module_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_ship_id, eve_group_attr_id: eve_ship_grp1_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id is None
    assert api_val.details.ship_limit.ship_group_id is None
    assert api_val.details.ship_limit.mismatches == {api_module.id: ([eve_ship_id], [eve_ship_grp1_id])}


def test_not_loaded_ship(client, consts):
    eve_ship_grp1_id = client.mk_eve_ship_group()
    eve_ship_grp2_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_ship1_id = client.mk_eve_ship(grp_id=eve_ship_grp2_id)
    eve_ship2_id = client.alloc_item_id()
    eve_module1_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_ship1_id, eve_group_attr_id: eve_ship_grp1_id})
    eve_module2_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_ship2_id, eve_group_attr_id: eve_ship_grp1_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship2_id)
    api_module1 = api_fit.add_mod(type_id=eve_module1_id)
    api_fit.add_mod(type_id=eve_module2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_ship2_id
    assert api_val.details.ship_limit.ship_group_id is None
    assert api_val.details.ship_limit.mismatches == {api_module1.id: ([eve_ship1_id], [eve_ship_grp1_id])}
    # Action
    api_module1.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_criterion_state(client, consts):
    # Restriction applies even to ghosted mods and disabled rigs
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_module_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_allowed_ship_id})
    eve_rig_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_allowed_ship_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.ghost)
    api_rig = api_fit.add_rig(type_id=eve_rig_id, state=False)
    api_subsystem = api_fit.add_subsystem(type_id=eve_rig_id, state=False)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.mismatches == {
        api_module.id: ([eve_allowed_ship_id], []),
        api_rig.id: ([eve_allowed_ship_id], []),
        api_subsystem.id: ([eve_allowed_ship_id], [])}


def test_criterion_item_type(client, consts):
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_module_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_allowed_ship_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_fit.add_implant(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
