from tests import approx, check_no_field


def test_rolls_range(client, consts):
    # Check processing of roll values - within range and out of range
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_add_lower_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_add_within_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_add_higher_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_change_lower_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_change_within_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_change_higher_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_remove_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_base_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2], attrs={
        eve_add_lower_attr_id: 100,
        eve_add_within_attr_id: 100,
        eve_add_higher_attr_id: 100,
        eve_change_lower_attr_id: 100,
        eve_change_within_attr_id: 100,
        eve_change_higher_attr_id: 100,
        eve_remove_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2])
    eve_mutator_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_mutator(
        datas=[eve_d2],
        id_=eve_mutator_id,
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={
            eve_add_lower_attr_id: (0.8, 1.2),
            eve_add_within_attr_id: (0.8, 1.2),
            eve_add_higher_attr_id: (0.8, 1.2),
            eve_change_lower_attr_id: (0.8, 1.2),
            eve_change_within_attr_id: (0.8, 1.2),
            eve_change_higher_attr_id: (0.8, 1.2),
            eve_remove_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_change_lower_attr_id: {consts.ApiAttrMutation.roll: 111},
        eve_change_within_attr_id: {consts.ApiAttrMutation.roll: 0.6},
        eve_change_higher_attr_id: {consts.ApiAttrMutation.roll: -8},
        eve_remove_attr_id: {consts.ApiAttrMutation.roll: 0.8}}))
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    assert api_item.attrs[eve_add_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_higher_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_higher_attr_id].base == approx(100)
    assert api_item.attrs[eve_remove_attr_id].base == approx(100)
    # Action
    api_item.change_mod(mutation={
        eve_add_lower_attr_id: {consts.ApiAttrMutation.roll: -5},
        eve_add_within_attr_id: {consts.ApiAttrMutation.roll: 0.3},
        eve_add_higher_attr_id: {consts.ApiAttrMutation.roll: 128},
        eve_change_lower_attr_id: {consts.ApiAttrMutation.roll: -60},
        eve_change_within_attr_id: {consts.ApiAttrMutation.roll: 0.1},
        eve_change_higher_attr_id: {consts.ApiAttrMutation.roll: 1.1},
        eve_remove_attr_id: None})
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    assert api_item.attrs[eve_add_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_higher_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_higher_attr_id].base == approx(100)
    assert api_item.attrs[eve_remove_attr_id].base == approx(100)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 7
    assert api_item.mutation.attrs[eve_add_lower_attr_id].roll == approx(0)
    assert api_item.mutation.attrs[eve_add_lower_attr_id].absolute == approx(80)
    assert api_item.mutation.attrs[eve_add_within_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_add_within_attr_id].absolute == approx(92)
    assert api_item.mutation.attrs[eve_add_higher_attr_id].roll == approx(1)
    assert api_item.mutation.attrs[eve_add_higher_attr_id].absolute == approx(120)
    assert api_item.mutation.attrs[eve_change_lower_attr_id].roll == approx(0)
    assert api_item.mutation.attrs[eve_change_lower_attr_id].absolute == approx(80)
    assert api_item.mutation.attrs[eve_change_within_attr_id].roll == approx(0.1)
    assert api_item.mutation.attrs[eve_change_within_attr_id].absolute == approx(84)
    assert api_item.mutation.attrs[eve_change_higher_attr_id].roll == approx(1)
    assert api_item.mutation.attrs[eve_change_higher_attr_id].absolute == approx(120)
    assert api_item.mutation.attrs[eve_remove_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_remove_attr_id].absolute == approx(100)
    assert api_item.attrs[eve_add_lower_attr_id].base == approx(80)
    assert api_item.attrs[eve_add_within_attr_id].base == approx(92)
    assert api_item.attrs[eve_add_higher_attr_id].base == approx(120)
    assert api_item.attrs[eve_change_lower_attr_id].base == approx(80)
    assert api_item.attrs[eve_change_within_attr_id].base == approx(84)
    assert api_item.attrs[eve_change_higher_attr_id].base == approx(120)
    assert api_item.attrs[eve_remove_attr_id].base == approx(100)


def test_absolute_value_range(client, consts):
    # Check processing of absolute values - within range and out of range
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_add_lower_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_add_within_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_add_higher_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_change_lower_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_change_within_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_change_higher_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_remove_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_base_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2], attrs={
        eve_add_lower_attr_id: 100,
        eve_add_within_attr_id: 100,
        eve_add_higher_attr_id: 100,
        eve_change_lower_attr_id: 100,
        eve_change_within_attr_id: 100,
        eve_change_higher_attr_id: 100,
        eve_remove_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2])
    eve_mutator_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_mutator(
        datas=[eve_d2],
        id_=eve_mutator_id,
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={
            eve_add_lower_attr_id: (0.8, 1.2),
            eve_add_within_attr_id: (0.8, 1.2),
            eve_add_higher_attr_id: (0.8, 1.2),
            eve_change_lower_attr_id: (0.8, 1.2),
            eve_change_within_attr_id: (0.8, 1.2),
            eve_change_higher_attr_id: (0.8, 1.2),
            eve_remove_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_change_lower_attr_id: {consts.ApiAttrMutation.absolute: 260},
        eve_change_within_attr_id: {consts.ApiAttrMutation.absolute: 104},
        eve_change_higher_attr_id: {consts.ApiAttrMutation.absolute: 0.5},
        eve_remove_attr_id: {consts.ApiAttrMutation.absolute: 112}}))
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    assert api_item.attrs[eve_add_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_higher_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_higher_attr_id].base == approx(100)
    assert api_item.attrs[eve_remove_attr_id].base == approx(100)
    # Action
    api_item.change_mod(mutation={
        eve_add_lower_attr_id: {consts.ApiAttrMutation.absolute: -502},
        eve_add_within_attr_id: {consts.ApiAttrMutation.absolute: 92},
        eve_add_higher_attr_id: {consts.ApiAttrMutation.absolute: 1001},
        eve_change_lower_attr_id: {consts.ApiAttrMutation.absolute: 0},
        eve_change_within_attr_id: {consts.ApiAttrMutation.absolute: 84},
        eve_change_higher_attr_id: {consts.ApiAttrMutation.absolute: 130},
        eve_remove_attr_id: None})
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    assert api_item.attrs[eve_add_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_higher_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_higher_attr_id].base == approx(100)
    assert api_item.attrs[eve_remove_attr_id].base == approx(100)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - on first source lib couldn't interpret absolute values, so defaults are exposed
    api_item.update()
    assert len(api_item.mutation.attrs) == 7
    assert api_item.mutation.attrs[eve_add_lower_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_lower_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_add_within_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_within_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_add_higher_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_higher_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_change_lower_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_change_lower_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_change_within_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_change_within_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_change_higher_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_change_higher_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_remove_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_remove_attr_id].absolute == approx(100)
    assert api_item.attrs[eve_add_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_higher_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_higher_attr_id].base == approx(100)
    assert api_item.attrs[eve_remove_attr_id].base == approx(100)


def test_no_base_item(client, consts):
    # Check that roll mutations are accepted for items w/o base item
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_add_roll_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_add_absolute_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_change_roll_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_change_absolute_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_remove_roll_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_remove_absolute_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_base_item_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(datas=[eve_d2], id_=eve_base_item_id, attrs={
        eve_add_roll_attr_id: 50,
        eve_add_absolute_attr_id: 50,
        eve_change_roll_attr_id: 50,
        eve_change_absolute_attr_id: 50,
        eve_remove_roll_attr_id: 50,
        eve_remove_absolute_attr_id: 50})
    eve_mutated_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2])
    eve_mutator_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_mutator(
        datas=[eve_d2],
        id_=eve_mutator_id,
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={
            eve_add_roll_attr_id: (0.8, 1.2),
            eve_add_absolute_attr_id: (0.8, 1.2),
            eve_change_roll_attr_id: (0.8, 1.2),
            eve_change_absolute_attr_id: (0.8, 1.2),
            eve_remove_roll_attr_id: (0.8, 1.2),
            eve_remove_absolute_attr_id: (0.8, 1.2)})
    # Make an item to ensure that attributes are not cleaned up on 1st source
    client.mk_eve_item(datas=[eve_d1], attrs={
        eve_add_roll_attr_id: 1,
        eve_add_absolute_attr_id: 1,
        eve_change_roll_attr_id: 1,
        eve_change_absolute_attr_id: 1,
        eve_remove_roll_attr_id: 1,
        eve_remove_absolute_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_change_roll_attr_id: {consts.ApiAttrMutation.roll: 0.7},
        eve_change_absolute_attr_id: {consts.ApiAttrMutation.absolute: 52},
        eve_remove_roll_attr_id: {consts.ApiAttrMutation.roll: 0.8},
        eve_remove_absolute_attr_id: {consts.ApiAttrMutation.absolute: 55}}))
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    with check_no_field():
        api_item.attrs  # pylint: disable=W0104
    # Action
    api_item.change_mod(mutation={
        eve_add_roll_attr_id: {consts.ApiAttrMutation.roll: 0.9},
        eve_add_absolute_attr_id: {consts.ApiAttrMutation.absolute: 59},
        eve_change_roll_attr_id: {consts.ApiAttrMutation.roll: 0.3},
        eve_change_absolute_attr_id: {consts.ApiAttrMutation.absolute: 48},
        eve_remove_roll_attr_id: None,
        eve_remove_absolute_attr_id: None})
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    with check_no_field():
        api_item.attrs  # pylint: disable=W0104
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 6
    assert api_item.mutation.attrs[eve_add_roll_attr_id].roll == approx(0.9)
    assert api_item.mutation.attrs[eve_add_roll_attr_id].absolute == approx(58)
    assert api_item.mutation.attrs[eve_add_absolute_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_absolute_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].absolute == approx(46)
    assert api_item.mutation.attrs[eve_change_absolute_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_change_absolute_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_remove_roll_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_remove_roll_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_remove_absolute_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_remove_absolute_attr_id].absolute == approx(50)
    assert api_item.attrs[eve_add_roll_attr_id].base == approx(58)
    assert api_item.attrs[eve_add_absolute_attr_id].base == approx(50)
    assert api_item.attrs[eve_change_roll_attr_id].base == approx(46)
    assert api_item.attrs[eve_change_absolute_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_roll_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_absolute_attr_id].base == approx(50)
