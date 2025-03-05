from tests import approx, check_no_field


def test_drone_add_remove(client, consts):
    # Also check matching against multiple allowed groups
    eve_group1_id = client.mk_eve_item_group()
    eve_group2_id = client.mk_eve_item_group()
    eve_group3_id = client.mk_eve_item_group()
    eve_limit_attr1_id = client.mk_eve_attr(
        id_=consts.EveAttr.allowed_drone_group1,
        unit_id=consts.EveAttrUnit.group_id)
    eve_limit_attr2_id = client.mk_eve_attr(
        id_=consts.EveAttr.allowed_drone_group2,
        unit_id=consts.EveAttrUnit.group_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_limit_attr1_id: eve_group1_id, eve_limit_attr2_id: eve_group2_id})
    eve_drone_id = client.mk_eve_item(grp_id=eve_group3_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group1_id, eve_group2_id], {api_drone.id: eve_group3_id})
    # Action
    api_drone.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_ship_add_set_remove(client, consts):
    eve_group1_id = client.mk_eve_item_group()
    eve_group2_id = client.mk_eve_item_group()
    eve_group3_id = client.mk_eve_item_group()
    eve_limit_attr1_id = client.mk_eve_attr(
        id_=consts.EveAttr.allowed_drone_group1,
        unit_id=consts.EveAttrUnit.group_id)
    eve_limit_attr2_id = client.mk_eve_attr(
        id_=consts.EveAttr.allowed_drone_group2,
        unit_id=consts.EveAttrUnit.group_id)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_limit_attr1_id: eve_group1_id})
    eve_ship2_id = client.mk_eve_ship()
    eve_ship3_id = client.mk_eve_ship(attrs={eve_limit_attr2_id: eve_group2_id})
    eve_drone_id = client.mk_eve_item(grp_id=eve_group3_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group1_id], {api_drone.id: eve_group3_id})
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_ship = api_fit.set_ship(type_id=eve_ship3_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group2_id], {api_drone.id: eve_group3_id})
    # Action
    api_ship.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_drone_add_remove_non_limited_ship(client, consts):
    eve_group1_id = client.mk_eve_item_group()
    eve_group2_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allowed_drone_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_ship1_id = client.mk_eve_ship()
    eve_ship2_id = client.mk_eve_ship(attrs={eve_limit_attr_id: eve_group1_id})
    eve_drone_id = client.mk_eve_item(grp_id=eve_group2_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group1_id], {api_drone.id: eve_group2_id})
    # Action
    api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_rounding(client, consts):
    # Also check that only unique groups are returned
    eve_group1_id = client.mk_eve_item_group()
    eve_group2_id = client.mk_eve_item_group()
    eve_limit_attr1_id = client.mk_eve_attr(
        id_=consts.EveAttr.allowed_drone_group1,
        unit_id=consts.EveAttrUnit.group_id)
    eve_limit_attr2_id = client.mk_eve_attr(
        id_=consts.EveAttr.allowed_drone_group2,
        unit_id=consts.EveAttrUnit.group_id)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_limit_attr1_id: eve_group1_id - 0.4, eve_limit_attr2_id: eve_group1_id + 0.4})
    eve_drone1_id = client.mk_eve_item(grp_id=eve_group1_id)
    eve_drone2_id = client.mk_eve_item(grp_id=eve_group2_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone1_id)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group1_id], {api_drone2.id: eve_group2_id})


def test_no_attr(client, consts):
    eve_group1_id = client.mk_eve_item_group()
    eve_group2_id = client.mk_eve_item_group()
    eve_limit_attr1_id = consts.EveAttr.allowed_drone_group1
    eve_ship_id = client.mk_eve_ship(attrs={eve_limit_attr1_id: eve_group1_id})
    eve_drone_id = client.mk_eve_item(grp_id=eve_group2_id)
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(grp_id=eve_group1_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group1_id], {api_drone.id: eve_group2_id})


def test_modified_limit(client, consts):
    eve_group1_id = client.mk_eve_item_group()
    eve_group2_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allowed_drone_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_limit_attr_id: eve_group1_id})
    eve_drone_id = client.mk_eve_item(grp_id=eve_group2_id)
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
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    assert api_ship.update().attrs[eve_limit_attr_id].extra == approx(eve_group1_id)
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group1_id], {api_drone.id: eve_group2_id})
    # Action
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification - attribute is modified, but not for purposes of validation
    assert api_ship.update().attrs[eve_limit_attr_id].extra == approx(eve_group2_id)
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group1_id], {api_drone.id: eve_group2_id})


def test_mutation_drone_group(client, consts):
    eve_group1_id = client.mk_eve_item_group()
    eve_group2_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allowed_drone_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_limit_attr_id: eve_group1_id})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_limit_attr_id: eve_group2_id})
    eve_base_drone_id = client.mk_eve_item(grp_id=eve_group2_id)
    eve_mutated_drone_id = client.mk_eve_item(grp_id=eve_group1_id)
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_drone_id], eve_mutated_drone_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship1_id)
    api_drone = api_fit.add_drone(type_id=eve_base_drone_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group1_id], {api_drone.id: eve_group2_id})
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone.change_drone(mutation=eve_mutator_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group2_id], {api_drone.id: eve_group1_id})
    # Action
    api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone.change_drone(mutation=None)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group1_id], {api_drone.id: eve_group2_id})


def test_not_loaded_ship(client, consts):
    eve_ship_id = client.alloc_item_id()
    eve_drone_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded_drone(client, consts):
    eve_group_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allowed_drone_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_ship_id = client.mk_eve_item(attrs={eve_limit_attr_id: eve_group_id})
    eve_drone_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_item_kind(client, consts):
    eve_group1_id = client.mk_eve_item_group()
    eve_group2_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allowed_drone_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_limit_attr_id: eve_group1_id})
    eve_fighter_id = client.mk_eve_item(grp_id=eve_group2_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
