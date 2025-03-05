from tests import check_no_field


def test_drone_add_remove(client, consts):
    eve_group1_id = client.alloc_group_id()
    eve_group2_id = client.alloc_group_id()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allowed_drone_group1)
    eve_ship_id = client.mk_eve_ship(attrs={eve_limit_attr_id: eve_group1_id})
    eve_drone_id = client.mk_eve_item(grp_id=eve_group2_id)
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
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group1_id], {api_drone.id: eve_group2_id})
    # Action
    api_drone.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.drone_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_ship_add_set_remove(client, consts):
    eve_group1_id = client.alloc_group_id()
    eve_group2_id = client.alloc_group_id()
    eve_group3_id = client.alloc_group_id()
    eve_limit_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.allowed_drone_group1)
    eve_limit_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.allowed_drone_group2)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_limit_attr1_id: eve_group1_id})
    eve_ship2_id = client.mk_eve_ship()
    eve_ship3_id = client.mk_eve_ship(attrs={eve_limit_attr2_id: eve_group2_id})
    eve_drone_id = client.mk_eve_item(grp_id=eve_group3_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
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
