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
        datas=[eve_d1],
        id_=eve_mutator_id,
        # Valid input or output item is needed just to keep mutator data alive during cleanup
        items=[([client.mk_eve_item(datas=[eve_d1])], eve_mutated_item_id)],
        attrs={
            eve_add_lower_attr_id: (0.8, 1.2),
            eve_add_within_attr_id: (0.8, 1.2),
            eve_add_higher_attr_id: (0.8, 1.2),
            eve_change_lower_attr_id: (0.8, 1.2),
            eve_change_within_attr_id: (0.8, 1.2),
            eve_change_higher_attr_id: (0.8, 1.2),
            eve_remove_attr_id: (0.8, 1.2)})
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
    api_item = api_fit.add_module(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_change_lower_attr_id: {consts.ApiAttrMutation.roll: 111},
        eve_change_within_attr_id: {consts.ApiAttrMutation.roll: 0.6},
        eve_change_higher_attr_id: {consts.ApiAttrMutation.roll: -8},
        eve_remove_attr_id: {consts.ApiAttrMutation.roll: 0.8}}))
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_add_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_higher_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_higher_attr_id].base == approx(100)
    assert api_item.attrs[eve_remove_attr_id].base == approx(100)
    # Action
    api_item.change_module(mutation={
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
        api_item.mutation  # noqa: B018
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
        datas=[eve_d1],
        id_=eve_mutator_id,
        # Valid input or output item is needed just to keep mutator data alive during cleanup
        items=[([client.mk_eve_item(datas=[eve_d1])], eve_mutated_item_id)],
        attrs={
            eve_add_lower_attr_id: (0.8, 1.2),
            eve_add_within_attr_id: (0.8, 1.2),
            eve_add_higher_attr_id: (0.8, 1.2),
            eve_change_lower_attr_id: (0.8, 1.2),
            eve_change_within_attr_id: (0.8, 1.2),
            eve_change_higher_attr_id: (0.8, 1.2),
            eve_remove_attr_id: (0.8, 1.2)})
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
    api_item = api_fit.add_module(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_change_lower_attr_id: {consts.ApiAttrMutation.absolute: 260},
        eve_change_within_attr_id: {consts.ApiAttrMutation.absolute: 104},
        eve_change_higher_attr_id: {consts.ApiAttrMutation.absolute: 0.5},
        eve_remove_attr_id: {consts.ApiAttrMutation.absolute: 112}}))
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_add_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_higher_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_higher_attr_id].base == approx(100)
    assert api_item.attrs[eve_remove_attr_id].base == approx(100)
    # Action
    api_item.change_module(mutation={
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
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_add_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_higher_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_higher_attr_id].base == approx(100)
    assert api_item.attrs[eve_remove_attr_id].base == approx(100)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - on the first source absolute values couldn't be set due to mutation being
    # incomplete, so defaults are exposed on the second source
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
        datas=[eve_d1],
        id_=eve_mutator_id,
        # Valid input or output item is needed just to keep mutator data alive during cleanup
        items=[([client.mk_eve_item(datas=[eve_d1])], eve_mutated_item_id)],
        attrs={
            eve_add_roll_attr_id: (0.8, 1.2),
            eve_add_absolute_attr_id: (0.8, 1.2),
            eve_change_roll_attr_id: (0.8, 1.2),
            eve_change_absolute_attr_id: (0.8, 1.2),
            eve_remove_roll_attr_id: (0.8, 1.2),
            eve_remove_absolute_attr_id: (0.8, 1.2)})
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
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_change_roll_attr_id: {consts.ApiAttrMutation.roll: 0.7},
        eve_change_absolute_attr_id: {consts.ApiAttrMutation.absolute: 52},
        eve_remove_roll_attr_id: {consts.ApiAttrMutation.roll: 0.8},
        eve_remove_absolute_attr_id: {consts.ApiAttrMutation.absolute: 55}}))
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    with check_no_field():
        api_item.attrs  # noqa: B018
    # Action
    api_item.change_module(mutation={
        eve_add_roll_attr_id: {consts.ApiAttrMutation.roll: 0.9},
        eve_add_absolute_attr_id: {consts.ApiAttrMutation.absolute: 59},
        eve_change_roll_attr_id: {consts.ApiAttrMutation.roll: 0.3},
        eve_change_absolute_attr_id: {consts.ApiAttrMutation.absolute: 48},
        eve_remove_roll_attr_id: None,
        eve_remove_absolute_attr_id: None})
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    with check_no_field():
        api_item.attrs  # noqa: B018
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


def test_no_base_value(client, consts):
    # Rolls accepted, absolutes discarded when base value is not available
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_add_roll_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_add_absolute_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_change_roll_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_change_absolute_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_remove_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_base_item_id = client.alloc_item_id()
    client.mk_eve_item(datas=[eve_d1], id_=eve_base_item_id)
    client.mk_eve_item(datas=[eve_d2], id_=eve_base_item_id, attrs={
        eve_add_roll_attr_id: 50,
        eve_add_absolute_attr_id: 50,
        eve_change_roll_attr_id: 50,
        eve_change_absolute_attr_id: 50,
        eve_remove_attr_id: 50})
    eve_mutated_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2])
    eve_mutator_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_mutator(
        datas=[eve_d1],
        id_=eve_mutator_id,
        # Valid input or output item is needed just to keep mutator data alive during cleanup
        items=[([client.mk_eve_item(datas=[eve_d1])], eve_mutated_item_id)],
        attrs={
            eve_add_roll_attr_id: (0.8, 1.2),
            eve_add_absolute_attr_id: (0.8, 1.2),
            eve_change_roll_attr_id: (0.8, 1.2),
            eve_change_absolute_attr_id: (0.8, 1.2),
            eve_remove_attr_id: (0.8, 1.2)})
    client.mk_eve_mutator(
        datas=[eve_d2],
        id_=eve_mutator_id,
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={
            eve_add_roll_attr_id: (0.8, 1.2),
            eve_add_absolute_attr_id: (0.8, 1.2),
            eve_change_roll_attr_id: (0.8, 1.2),
            eve_change_absolute_attr_id: (0.8, 1.2),
            eve_remove_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_change_roll_attr_id: {consts.ApiAttrMutation.roll: 0.3},
        eve_change_absolute_attr_id: {consts.ApiAttrMutation.absolute: 46},
        eve_remove_attr_id: {consts.ApiAttrMutation.roll: 0.2}}))
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    with check_no_field():
        api_item.attrs  # noqa: B018
    # Action
    api_item.change_module(mutation={
        eve_add_roll_attr_id: {consts.ApiAttrMutation.roll: 0.7},
        eve_add_absolute_attr_id: {consts.ApiAttrMutation.absolute: 54},
        eve_change_roll_attr_id: {consts.ApiAttrMutation.roll: 0.4},
        eve_change_absolute_attr_id: {consts.ApiAttrMutation.absolute: 45},
        eve_remove_attr_id: None})
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    with check_no_field():
        api_item.attrs  # noqa: B018
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - since there were no base attribute values on first source, attribute mutations
    # defined via absolute value were discarded. However, on second source roll and absolute value
    # are still exposed, but without mutation applied
    api_item.update()
    assert len(api_item.mutation.attrs) == 5
    assert api_item.mutation.attrs[eve_add_roll_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_add_roll_attr_id].absolute == approx(54)
    assert api_item.mutation.attrs[eve_add_absolute_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_absolute_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].roll == approx(0.4)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].absolute == approx(48)
    assert api_item.mutation.attrs[eve_change_absolute_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_change_absolute_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_remove_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_remove_attr_id].absolute == approx(50)
    assert api_item.attrs[eve_add_roll_attr_id].base == approx(54)
    assert api_item.attrs[eve_add_absolute_attr_id].base == approx(50)
    assert api_item.attrs[eve_change_roll_attr_id].base == approx(48)
    assert api_item.attrs[eve_change_absolute_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_attr_id].base == approx(50)


def test_no_mutation_range(client, consts):
    # Check that absolute values are discarded when mutation range is not defined
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_add_roll_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_add_absolute_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_change_roll_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_change_absolute_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_remove_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_base_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2], attrs={
        eve_add_roll_attr_id: 50,
        eve_add_absolute_attr_id: 50,
        eve_change_roll_attr_id: 50,
        eve_change_absolute_attr_id: 50,
        eve_remove_attr_id: 50})
    eve_mutated_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2])
    eve_mutator_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_mutator(
        datas=[eve_d1],
        id_=eve_mutator_id,
        # Valid input or output item is needed just to keep mutator data alive during cleanup
        items=[([client.mk_eve_item(datas=[eve_d1])], eve_mutated_item_id)])
    client.mk_eve_mutator(
        datas=[eve_d2],
        id_=eve_mutator_id,
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={
            eve_add_roll_attr_id: (0.8, 1.2),
            eve_add_absolute_attr_id: (0.8, 1.2),
            eve_change_roll_attr_id: (0.8, 1.2),
            eve_change_absolute_attr_id: (0.8, 1.2),
            eve_remove_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_change_roll_attr_id: {consts.ApiAttrMutation.roll: 0.3},
        eve_change_absolute_attr_id: {consts.ApiAttrMutation.absolute: 46},
        eve_remove_attr_id: {consts.ApiAttrMutation.roll: 0.2}}))
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_add_roll_attr_id].base == approx(50)
    assert api_item.attrs[eve_add_absolute_attr_id].base == approx(50)
    assert api_item.attrs[eve_change_roll_attr_id].base == approx(50)
    assert api_item.attrs[eve_change_absolute_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_attr_id].base == approx(50)
    # Action
    api_item.change_module(mutation={
        eve_add_roll_attr_id: {consts.ApiAttrMutation.roll: 0.7},
        eve_add_absolute_attr_id: {consts.ApiAttrMutation.absolute: 54},
        eve_change_roll_attr_id: {consts.ApiAttrMutation.roll: 0.4},
        eve_change_absolute_attr_id: {consts.ApiAttrMutation.absolute: 45},
        eve_remove_attr_id: None})
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_add_roll_attr_id].base == approx(50)
    assert api_item.attrs[eve_add_absolute_attr_id].base == approx(50)
    assert api_item.attrs[eve_change_roll_attr_id].base == approx(50)
    assert api_item.attrs[eve_change_absolute_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_attr_id].base == approx(50)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - since there was no mutation range on first source, attribute mutations defined
    # via absolute value were discarded. However, on second source absolute value is still exposed,
    # but without mutation applied
    api_item.update()
    assert len(api_item.mutation.attrs) == 5
    assert api_item.mutation.attrs[eve_add_roll_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_add_roll_attr_id].absolute == approx(54)
    assert api_item.mutation.attrs[eve_add_absolute_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_absolute_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].roll == approx(0.4)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].absolute == approx(48)
    assert api_item.mutation.attrs[eve_change_absolute_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_change_absolute_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_remove_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_remove_attr_id].absolute == approx(50)
    assert api_item.attrs[eve_add_roll_attr_id].base == approx(54)
    assert api_item.attrs[eve_add_absolute_attr_id].base == approx(50)
    assert api_item.attrs[eve_change_roll_attr_id].base == approx(48)
    assert api_item.attrs[eve_change_absolute_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_attr_id].base == approx(50)


def test_zero_mutation_range(client, consts):
    # Check that absolute values are discarded when mutation range has zero width
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_add_roll_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_add_absolute_low_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_add_absolute_mid_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_add_absolute_high_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_change_roll_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_change_absolute_low_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_change_absolute_mid_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_change_absolute_high_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_remove_roll_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_remove_absolute_low_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_remove_absolute_mid_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_remove_absolute_high_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_base_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2], attrs={
        eve_add_roll_attr_id: 50,
        eve_add_absolute_low_attr_id: 50,
        eve_add_absolute_mid_attr_id: 50,
        eve_add_absolute_high_attr_id: 50,
        eve_change_roll_attr_id: -50,
        eve_change_absolute_low_attr_id: -50,
        eve_change_absolute_mid_attr_id: -50,
        eve_change_absolute_high_attr_id: -50,
        eve_remove_roll_attr_id: 50,
        eve_remove_absolute_low_attr_id: 50,
        eve_remove_absolute_mid_attr_id: 50,
        eve_remove_absolute_high_attr_id: 50})
    eve_mutated_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2])
    eve_mutator_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_mutator(
        datas=[eve_d1],
        id_=eve_mutator_id,
        # Valid input or output item is needed just to keep mutator data alive during cleanup
        items=[([client.mk_eve_item(datas=[eve_d1])], eve_mutated_item_id)],
        attrs={
            eve_add_roll_attr_id: (1.08, 1.08),
            eve_add_absolute_low_attr_id: (0.92, 0.92),
            eve_add_absolute_mid_attr_id: (1, 1),
            eve_add_absolute_high_attr_id: (1.08, 1.08),
            eve_change_roll_attr_id: (0.92, 0.92),
            eve_change_absolute_low_attr_id: (0.92, 0.92),
            eve_change_absolute_mid_attr_id: (1, 1),
            eve_change_absolute_high_attr_id: (1.08, 1.08),
            eve_remove_roll_attr_id: (1.08, 1.08),
            eve_remove_absolute_low_attr_id: (0.92, 0.92),
            eve_remove_absolute_mid_attr_id: (1, 1),
            eve_remove_absolute_high_attr_id: (1.08, 1.08)})
    client.mk_eve_mutator(
        datas=[eve_d2],
        id_=eve_mutator_id,
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={
            eve_add_roll_attr_id: (0.8, 1.2),
            eve_add_absolute_low_attr_id: (0.8, 1.2),
            eve_add_absolute_mid_attr_id: (0.8, 1.2),
            eve_add_absolute_high_attr_id: (0.8, 1.2),
            eve_change_roll_attr_id: (0.8, 1.2),
            eve_change_absolute_low_attr_id: (0.8, 1.2),
            eve_change_absolute_mid_attr_id: (0.8, 1.2),
            eve_change_absolute_high_attr_id: (0.8, 1.2),
            eve_remove_roll_attr_id: (0.8, 1.2),
            eve_remove_absolute_low_attr_id: (0.8, 1.2),
            eve_remove_absolute_mid_attr_id: (0.8, 1.2),
            eve_remove_absolute_high_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_change_roll_attr_id: {consts.ApiAttrMutation.roll: 0.7},
        eve_change_absolute_low_attr_id: {consts.ApiAttrMutation.absolute: -54},
        eve_change_absolute_mid_attr_id: {consts.ApiAttrMutation.absolute: -54},
        eve_change_absolute_high_attr_id: {consts.ApiAttrMutation.absolute: -54},
        eve_remove_roll_attr_id: {consts.ApiAttrMutation.roll: 0.4},
        eve_remove_absolute_low_attr_id: {consts.ApiAttrMutation.absolute: 54},
        eve_remove_absolute_mid_attr_id: {consts.ApiAttrMutation.absolute: 54},
        eve_remove_absolute_high_attr_id: {consts.ApiAttrMutation.absolute: 54}}))
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_add_roll_attr_id].base == approx(50)
    assert api_item.attrs[eve_add_absolute_low_attr_id].base == approx(50)
    assert api_item.attrs[eve_add_absolute_mid_attr_id].base == approx(50)
    assert api_item.attrs[eve_add_absolute_high_attr_id].base == approx(50)
    assert api_item.attrs[eve_change_roll_attr_id].base == approx(-50)
    assert api_item.attrs[eve_change_absolute_low_attr_id].base == approx(-50)
    assert api_item.attrs[eve_change_absolute_mid_attr_id].base == approx(-50)
    assert api_item.attrs[eve_change_absolute_high_attr_id].base == approx(-50)
    assert api_item.attrs[eve_remove_roll_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_absolute_low_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_absolute_mid_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_absolute_high_attr_id].base == approx(50)
    # Action
    api_item.change_module(mutation={
        eve_add_roll_attr_id: {consts.ApiAttrMutation.roll: 0.7},
        eve_add_absolute_low_attr_id: {consts.ApiAttrMutation.absolute: 50},
        eve_add_absolute_mid_attr_id: {consts.ApiAttrMutation.absolute: 50},
        eve_add_absolute_high_attr_id: {consts.ApiAttrMutation.absolute: 50},
        eve_change_roll_attr_id: {consts.ApiAttrMutation.roll: 0.2},
        eve_change_absolute_low_attr_id: {consts.ApiAttrMutation.absolute: -50},
        eve_change_absolute_mid_attr_id: {consts.ApiAttrMutation.absolute: -50},
        eve_change_absolute_high_attr_id: {consts.ApiAttrMutation.absolute: -50},
        eve_remove_roll_attr_id: None,
        eve_remove_absolute_low_attr_id: None,
        eve_remove_absolute_mid_attr_id: None,
        eve_remove_absolute_high_attr_id: None})
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_add_roll_attr_id].base == approx(50)
    assert api_item.attrs[eve_add_absolute_low_attr_id].base == approx(50)
    assert api_item.attrs[eve_add_absolute_mid_attr_id].base == approx(50)
    assert api_item.attrs[eve_add_absolute_high_attr_id].base == approx(50)
    assert api_item.attrs[eve_change_roll_attr_id].base == approx(-50)
    assert api_item.attrs[eve_change_absolute_low_attr_id].base == approx(-50)
    assert api_item.attrs[eve_change_absolute_mid_attr_id].base == approx(-50)
    assert api_item.attrs[eve_change_absolute_high_attr_id].base == approx(-50)
    assert api_item.attrs[eve_remove_roll_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_absolute_low_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_absolute_mid_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_absolute_high_attr_id].base == approx(50)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 12
    assert api_item.mutation.attrs[eve_add_roll_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_add_roll_attr_id].absolute == approx(54)
    assert api_item.mutation.attrs[eve_add_absolute_low_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_absolute_low_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_add_absolute_mid_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_absolute_mid_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_add_absolute_high_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_absolute_high_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].roll == approx(0.2)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].absolute == approx(-44)
    assert api_item.mutation.attrs[eve_change_absolute_low_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_change_absolute_low_attr_id].absolute == approx(-50)
    assert api_item.mutation.attrs[eve_change_absolute_mid_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_change_absolute_mid_attr_id].absolute == approx(-50)
    assert api_item.mutation.attrs[eve_change_absolute_high_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_change_absolute_high_attr_id].absolute == approx(-50)
    assert api_item.mutation.attrs[eve_remove_roll_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_remove_roll_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_remove_absolute_low_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_remove_absolute_low_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_remove_absolute_mid_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_remove_absolute_mid_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_remove_absolute_high_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_remove_absolute_high_attr_id].absolute == approx(50)
    assert api_item.attrs[eve_add_roll_attr_id].base == approx(54)
    assert api_item.attrs[eve_add_absolute_low_attr_id].base == approx(50)
    assert api_item.attrs[eve_add_absolute_mid_attr_id].base == approx(50)
    assert api_item.attrs[eve_add_absolute_high_attr_id].base == approx(50)
    assert api_item.attrs[eve_change_roll_attr_id].base == approx(-44)
    assert api_item.attrs[eve_change_absolute_low_attr_id].base == approx(-50)
    assert api_item.attrs[eve_change_absolute_mid_attr_id].base == approx(-50)
    assert api_item.attrs[eve_change_absolute_high_attr_id].base == approx(-50)
    assert api_item.attrs[eve_remove_roll_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_absolute_low_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_absolute_mid_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_absolute_high_attr_id].base == approx(50)


def test_zero_base_value(client, consts):
    # Check that absolute values are discarded when base value is zero
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_add_roll_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_add_absolute_low_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_add_absolute_mid_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_add_absolute_high_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_change_roll_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_change_absolute_low_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_change_absolute_mid_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_change_absolute_high_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_remove_roll_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_remove_absolute_low_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_remove_absolute_mid_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_remove_absolute_high_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_base_item_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(datas=[eve_d1], id_=eve_base_item_id, attrs={
        eve_add_roll_attr_id: 0,
        eve_add_absolute_low_attr_id: 0,
        eve_add_absolute_mid_attr_id: 0,
        eve_add_absolute_high_attr_id: 0,
        eve_change_roll_attr_id: 0,
        eve_change_absolute_low_attr_id: 0,
        eve_change_absolute_mid_attr_id: 0,
        eve_change_absolute_high_attr_id: 0,
        eve_remove_roll_attr_id: 0,
        eve_remove_absolute_low_attr_id: 0,
        eve_remove_absolute_mid_attr_id: 0,
        eve_remove_absolute_high_attr_id: 0})
    client.mk_eve_item(datas=[eve_d2], id_=eve_base_item_id, attrs={
        eve_add_roll_attr_id: 50,
        eve_add_absolute_low_attr_id: 50,
        eve_add_absolute_mid_attr_id: 50,
        eve_add_absolute_high_attr_id: 50,
        eve_change_roll_attr_id: 50,
        eve_change_absolute_low_attr_id: 50,
        eve_change_absolute_mid_attr_id: 50,
        eve_change_absolute_high_attr_id: 50,
        eve_remove_roll_attr_id: 50,
        eve_remove_absolute_low_attr_id: 50,
        eve_remove_absolute_mid_attr_id: 50,
        eve_remove_absolute_high_attr_id: 50})
    eve_mutated_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2])
    eve_mutator_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_mutator(
        datas=[eve_d1],
        id_=eve_mutator_id,
        # Valid input or output item is needed just to keep mutator data alive during cleanup
        items=[([client.mk_eve_item(datas=[eve_d1])], eve_mutated_item_id)],
        attrs={
            eve_add_roll_attr_id: (0.8, 1.2),
            eve_add_absolute_low_attr_id: (0.8, 1.2),
            eve_add_absolute_mid_attr_id: (0.8, 1.2),
            eve_add_absolute_high_attr_id: (0.8, 1.2),
            eve_change_roll_attr_id: (0.8, 1.2),
            eve_change_absolute_low_attr_id: (0.8, 1.2),
            eve_change_absolute_mid_attr_id: (0.8, 1.2),
            eve_change_absolute_high_attr_id: (0.8, 1.2),
            eve_remove_roll_attr_id: (0.8, 1.2),
            eve_remove_absolute_low_attr_id: (0.8, 1.2),
            eve_remove_absolute_mid_attr_id: (0.8, 1.2),
            eve_remove_absolute_high_attr_id: (0.8, 1.2)})
    client.mk_eve_mutator(
        datas=[eve_d2],
        id_=eve_mutator_id,
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={
            eve_add_roll_attr_id: (0.8, 1.2),
            eve_add_absolute_low_attr_id: (0.8, 1.2),
            eve_add_absolute_mid_attr_id: (0.8, 1.2),
            eve_add_absolute_high_attr_id: (0.8, 1.2),
            eve_change_roll_attr_id: (0.8, 1.2),
            eve_change_absolute_low_attr_id: (0.8, 1.2),
            eve_change_absolute_mid_attr_id: (0.8, 1.2),
            eve_change_absolute_high_attr_id: (0.8, 1.2),
            eve_remove_roll_attr_id: (0.8, 1.2),
            eve_remove_absolute_low_attr_id: (0.8, 1.2),
            eve_remove_absolute_mid_attr_id: (0.8, 1.2),
            eve_remove_absolute_high_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_change_roll_attr_id: {consts.ApiAttrMutation.roll: 0.7},
        eve_change_absolute_low_attr_id: {consts.ApiAttrMutation.absolute: 0},
        eve_change_absolute_mid_attr_id: {consts.ApiAttrMutation.absolute: 0},
        eve_change_absolute_high_attr_id: {consts.ApiAttrMutation.absolute: 0},
        eve_remove_roll_attr_id: {consts.ApiAttrMutation.roll: 0.4},
        eve_remove_absolute_low_attr_id: {consts.ApiAttrMutation.absolute: -3},
        eve_remove_absolute_mid_attr_id: {consts.ApiAttrMutation.absolute: 0},
        eve_remove_absolute_high_attr_id: {consts.ApiAttrMutation.absolute: 6}}))
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_add_roll_attr_id].base == approx(0)
    assert api_item.attrs[eve_add_absolute_low_attr_id].base == approx(0)
    assert api_item.attrs[eve_add_absolute_mid_attr_id].base == approx(0)
    assert api_item.attrs[eve_add_absolute_high_attr_id].base == approx(0)
    assert api_item.attrs[eve_change_roll_attr_id].base == approx(0)
    assert api_item.attrs[eve_change_absolute_low_attr_id].base == approx(0)
    assert api_item.attrs[eve_change_absolute_mid_attr_id].base == approx(0)
    assert api_item.attrs[eve_change_absolute_high_attr_id].base == approx(0)
    assert api_item.attrs[eve_remove_roll_attr_id].base == approx(0)
    assert api_item.attrs[eve_remove_absolute_low_attr_id].base == approx(0)
    assert api_item.attrs[eve_remove_absolute_mid_attr_id].base == approx(0)
    assert api_item.attrs[eve_remove_absolute_high_attr_id].base == approx(0)
    # Action
    api_item.change_module(mutation={
        eve_add_roll_attr_id: {consts.ApiAttrMutation.roll: 0.7},
        eve_add_absolute_low_attr_id: {consts.ApiAttrMutation.absolute: -8},
        eve_add_absolute_mid_attr_id: {consts.ApiAttrMutation.absolute: 0},
        eve_add_absolute_high_attr_id: {consts.ApiAttrMutation.absolute: 2},
        eve_change_roll_attr_id: {consts.ApiAttrMutation.roll: 0.2},
        eve_change_absolute_low_attr_id: {consts.ApiAttrMutation.absolute: -3},
        eve_change_absolute_mid_attr_id: {consts.ApiAttrMutation.absolute: 0},
        eve_change_absolute_high_attr_id: {consts.ApiAttrMutation.absolute: 6},
        eve_remove_roll_attr_id: None,
        eve_remove_absolute_low_attr_id: None,
        eve_remove_absolute_mid_attr_id: None,
        eve_remove_absolute_high_attr_id: None})
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_add_roll_attr_id].base == approx(0)
    assert api_item.attrs[eve_add_absolute_low_attr_id].base == approx(0)
    assert api_item.attrs[eve_add_absolute_mid_attr_id].base == approx(0)
    assert api_item.attrs[eve_add_absolute_high_attr_id].base == approx(0)
    assert api_item.attrs[eve_change_roll_attr_id].base == approx(0)
    assert api_item.attrs[eve_change_absolute_low_attr_id].base == approx(0)
    assert api_item.attrs[eve_change_absolute_mid_attr_id].base == approx(0)
    assert api_item.attrs[eve_change_absolute_high_attr_id].base == approx(0)
    assert api_item.attrs[eve_remove_roll_attr_id].base == approx(0)
    assert api_item.attrs[eve_remove_absolute_low_attr_id].base == approx(0)
    assert api_item.attrs[eve_remove_absolute_mid_attr_id].base == approx(0)
    assert api_item.attrs[eve_remove_absolute_high_attr_id].base == approx(0)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 12
    assert api_item.mutation.attrs[eve_add_roll_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_add_roll_attr_id].absolute == approx(54)
    assert api_item.mutation.attrs[eve_add_absolute_low_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_absolute_low_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_add_absolute_mid_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_absolute_mid_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_add_absolute_high_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_absolute_high_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].roll == approx(0.2)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].absolute == approx(44)
    assert api_item.mutation.attrs[eve_change_absolute_low_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_change_absolute_low_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_change_absolute_mid_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_change_absolute_mid_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_change_absolute_high_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_change_absolute_high_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_remove_roll_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_remove_roll_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_remove_absolute_low_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_remove_absolute_low_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_remove_absolute_mid_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_remove_absolute_mid_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_remove_absolute_high_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_remove_absolute_high_attr_id].absolute == approx(50)
    assert api_item.attrs[eve_add_roll_attr_id].base == approx(54)
    assert api_item.attrs[eve_add_absolute_low_attr_id].base == approx(50)
    assert api_item.attrs[eve_add_absolute_mid_attr_id].base == approx(50)
    assert api_item.attrs[eve_add_absolute_high_attr_id].base == approx(50)
    assert api_item.attrs[eve_change_roll_attr_id].base == approx(44)
    assert api_item.attrs[eve_change_absolute_low_attr_id].base == approx(50)
    assert api_item.attrs[eve_change_absolute_mid_attr_id].base == approx(50)
    assert api_item.attrs[eve_change_absolute_high_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_roll_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_absolute_low_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_absolute_mid_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_absolute_high_attr_id].base == approx(50)
