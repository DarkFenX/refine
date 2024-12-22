from tests import approx, check_no_field


def test_rolls_range(client, consts):
    # Check processing of roll values - within range and out of range
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_lower_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_within_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_higher_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_base_item_id = client.mk_eve_item(
        datas=[eve_d1, eve_d2],
        attrs={eve_lower_attr_id: 100, eve_within_attr_id: 100, eve_higher_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2])
    eve_mutator_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_mutator(
        datas=[eve_d1],
        id_=eve_mutator_id,
        # Valid input or output item is needed just to keep mutator data alive during cleanup
        items=[([client.mk_eve_item(datas=[eve_d1])], eve_mutated_item_id)],
        attrs={eve_lower_attr_id: (0.8, 1.2), eve_within_attr_id: (0.8, 1.2), eve_higher_attr_id: (0.8, 1.2)})
    client.mk_eve_mutator(
        datas=[eve_d2],
        id_=eve_mutator_id,
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_lower_attr_id: (0.8, 1.2), eve_within_attr_id: (0.8, 1.2), eve_higher_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_lower_attr_id: {consts.ApiAttrMutation.roll: -5},
        eve_within_attr_id: {consts.ApiAttrMutation.roll: 0.3},
        eve_higher_attr_id: {consts.ApiAttrMutation.roll: 128}}))
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    assert api_item.attrs[eve_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_higher_attr_id].base == approx(100)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - on 2nd source item mutations with roll value can be properly applied
    api_item.update()
    assert len(api_item.mutation.attrs) == 3
    assert api_item.mutation.attrs[eve_lower_attr_id].roll == approx(0)
    assert api_item.mutation.attrs[eve_lower_attr_id].absolute == approx(80)
    assert api_item.mutation.attrs[eve_within_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_within_attr_id].absolute == approx(92)
    assert api_item.mutation.attrs[eve_higher_attr_id].roll == approx(1)
    assert api_item.mutation.attrs[eve_higher_attr_id].absolute == approx(120)
    assert api_item.attrs[eve_lower_attr_id].base == approx(80)
    assert api_item.attrs[eve_within_attr_id].base == approx(92)
    assert api_item.attrs[eve_higher_attr_id].base == approx(120)


def test_absolute_value_range(client, consts):
    # Check processing of absolute values - within range and out of range
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_lower_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_within_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_higher_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_base_item_id = client.mk_eve_item(
        datas=[eve_d1, eve_d2],
        attrs={eve_lower_attr_id: 100, eve_within_attr_id: 100, eve_higher_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2])
    eve_mutator_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_mutator(
        datas=[eve_d1],
        id_=eve_mutator_id,
        # Valid input or output item is needed just to keep mutator data alive during cleanup
        items=[([client.mk_eve_item(datas=[eve_d1])], eve_mutated_item_id)],
        attrs={eve_lower_attr_id: (0.8, 1.2), eve_within_attr_id: (0.8, 1.2), eve_higher_attr_id: (0.8, 1.2)})
    client.mk_eve_mutator(
        datas=[eve_d2],
        id_=eve_mutator_id,
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_lower_attr_id: (0.8, 1.2), eve_within_attr_id: (0.8, 1.2), eve_higher_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_lower_attr_id: {consts.ApiAttrMutation.absolute: -53},
        eve_within_attr_id: {consts.ApiAttrMutation.absolute: 92},
        eve_higher_attr_id: {consts.ApiAttrMutation.absolute: 1009}}))
    # Verification - mutation is not exposed, since it is not complete with data, but absolute
    # values were resolved into rolls using base item attributes
    api_item.update()
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    assert api_item.attrs[eve_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_higher_attr_id].base == approx(100)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - on 2nd source item mutations with roll value can be properly applied
    api_item.update()
    assert len(api_item.mutation.attrs) == 3
    assert api_item.mutation.attrs[eve_lower_attr_id].roll == approx(0)
    assert api_item.mutation.attrs[eve_lower_attr_id].absolute == approx(80)
    assert api_item.mutation.attrs[eve_within_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_within_attr_id].absolute == approx(92)
    assert api_item.mutation.attrs[eve_higher_attr_id].roll == approx(1)
    assert api_item.mutation.attrs[eve_higher_attr_id].absolute == approx(120)
    assert api_item.attrs[eve_lower_attr_id].base == approx(80)
    assert api_item.attrs[eve_within_attr_id].base == approx(92)
    assert api_item.attrs[eve_higher_attr_id].base == approx(120)


def test_no_base_item(client, consts):
    # Check that roll mutations are accepted for items w/o base item
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_roll_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_absolute_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_base_item_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(
        datas=[eve_d2],
        id_=eve_base_item_id,
        attrs={eve_roll_attr_id: 100, eve_absolute_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2])
    eve_mutator_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_mutator(
        datas=[eve_d1],
        id_=eve_mutator_id,
        # Valid input or output item is needed just to keep mutator data alive during cleanup
        items=[([client.mk_eve_item(datas=[eve_d1])], eve_mutated_item_id)],
        attrs={eve_roll_attr_id: (0.8, 1.2), eve_absolute_attr_id: (0.8, 1.2)})
    client.mk_eve_mutator(
        datas=[eve_d2],
        id_=eve_mutator_id,
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_roll_attr_id: (0.8, 1.2), eve_absolute_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_roll_attr_id: {consts.ApiAttrMutation.roll: 0.7},
        eve_absolute_attr_id: {consts.ApiAttrMutation.absolute: 104}}))
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    with check_no_field():
        api_item.attrs  # pylint: disable=W0104
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - on first source lib couldn't interpret absolute values, so defaults are exposed
    api_item.update()
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_roll_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_roll_attr_id].absolute == approx(108)
    assert api_item.mutation.attrs[eve_absolute_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_absolute_attr_id].absolute == approx(100)
    assert api_item.attrs[eve_roll_attr_id].base == approx(108)
    assert api_item.attrs[eve_absolute_attr_id].base == approx(100)


def test_no_base_value(client, consts):
    # Rolls accepted, absolutes discarded when base value is not available
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_roll_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_absolute_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_base_item_id = client.alloc_item_id()
    client.mk_eve_item(datas=[eve_d1], id_=eve_base_item_id)
    client.mk_eve_item(datas=[eve_d2], id_=eve_base_item_id, attrs={eve_roll_attr_id: 50, eve_absolute_attr_id: 50})
    eve_mutated_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2])
    eve_mutator_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_mutator(
        datas=[eve_d1],
        id_=eve_mutator_id,
        # Valid input or output item is needed just to keep mutator data alive during cleanup
        items=[([client.mk_eve_item(datas=[eve_d1])], eve_mutated_item_id)],
        attrs={eve_roll_attr_id: (0.8, 1.2), eve_absolute_attr_id: (0.8, 1.2)})
    client.mk_eve_mutator(
        datas=[eve_d2],
        id_=eve_mutator_id,
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_roll_attr_id: (0.8, 1.2), eve_absolute_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_roll_attr_id: {consts.ApiAttrMutation.roll: 0.7},
        eve_absolute_attr_id: {consts.ApiAttrMutation.absolute: 54}}))
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    with check_no_field():
        api_item.attrs  # pylint: disable=W0104
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - on first source lib couldn't interpret absolute values, so defaults are exposed
    api_item.update()
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_roll_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_roll_attr_id].absolute == approx(54)
    assert api_item.mutation.attrs[eve_absolute_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_absolute_attr_id].absolute == approx(50)
    assert api_item.attrs[eve_roll_attr_id].base == approx(54)
    assert api_item.attrs[eve_absolute_attr_id].base == approx(50)


def test_no_mutation_range(client, consts):
    # Check that absolute values are discarded when mutation range is not defined
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_roll_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_absolute_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_base_item_id = client.mk_eve_item(
        datas=[eve_d1, eve_d2],
        attrs={eve_roll_attr_id: 50, eve_absolute_attr_id: 50})
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
        attrs={eve_roll_attr_id: (0.8, 1.2), eve_absolute_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_roll_attr_id: {consts.ApiAttrMutation.roll: 0.7},
        eve_absolute_attr_id: {consts.ApiAttrMutation.absolute: 54}}))
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    assert api_item.attrs[eve_roll_attr_id].base == approx(50)
    assert api_item.attrs[eve_absolute_attr_id].base == approx(50)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - on first source lib couldn't interpret absolute values, so defaults are exposed
    api_item.update()
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_roll_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_roll_attr_id].absolute == approx(54)
    assert api_item.mutation.attrs[eve_absolute_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_absolute_attr_id].absolute == approx(50)
    assert api_item.attrs[eve_roll_attr_id].base == approx(54)
    assert api_item.attrs[eve_absolute_attr_id].base == approx(50)


def test_zero_mutation_range(client, consts):
    # Check that absolute values are discarded when mutation range has zero width
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_roll_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_absolute_low_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_absolute_mid_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_absolute_high_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_base_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2], attrs={
        eve_roll_attr_id: 50,
        eve_absolute_low_attr_id: 50,
        eve_absolute_mid_attr_id: 50,
        eve_absolute_high_attr_id: 50})
    eve_mutated_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2])
    eve_mutator_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_mutator(
        datas=[eve_d1],
        id_=eve_mutator_id,
        # Valid input or output item is needed just to keep mutator data alive during cleanup
        items=[([client.mk_eve_item(datas=[eve_d1])], eve_mutated_item_id)],
        attrs={
            eve_roll_attr_id: (1.08, 1.08),
            eve_absolute_low_attr_id: (0.92, 0.92),
            eve_absolute_mid_attr_id: (1, 1),
            eve_absolute_high_attr_id: (1.08, 1.08)})
    client.mk_eve_mutator(
        datas=[eve_d2],
        id_=eve_mutator_id,
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={
            eve_roll_attr_id: (0.8, 1.2),
            eve_absolute_low_attr_id: (0.8, 1.2),
            eve_absolute_mid_attr_id: (0.8, 1.2),
            eve_absolute_high_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_roll_attr_id: {consts.ApiAttrMutation.roll: 0.7},
        eve_absolute_low_attr_id: {consts.ApiAttrMutation.absolute: 54},
        eve_absolute_mid_attr_id: {consts.ApiAttrMutation.absolute: 54},
        eve_absolute_high_attr_id: {consts.ApiAttrMutation.absolute: 54}}))
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    assert api_item.attrs[eve_roll_attr_id].base == approx(50)
    assert api_item.attrs[eve_absolute_low_attr_id].base == approx(50)
    assert api_item.attrs[eve_absolute_mid_attr_id].base == approx(50)
    assert api_item.attrs[eve_absolute_high_attr_id].base == approx(50)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - on first source lib couldn't interpret absolute values, so defaults are exposed
    api_item.update()
    assert len(api_item.mutation.attrs) == 4
    assert api_item.mutation.attrs[eve_roll_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_roll_attr_id].absolute == approx(54)
    assert api_item.mutation.attrs[eve_absolute_low_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_absolute_low_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_absolute_mid_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_absolute_mid_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_absolute_high_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_absolute_high_attr_id].absolute == approx(50)
    assert api_item.attrs[eve_roll_attr_id].base == approx(54)
    assert api_item.attrs[eve_absolute_low_attr_id].base == approx(50)
    assert api_item.attrs[eve_absolute_mid_attr_id].base == approx(50)
    assert api_item.attrs[eve_absolute_high_attr_id].base == approx(50)


def test_zero_base_value(client, consts):
    # Check that absolute values are discarded when base value is zero
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_roll_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_absolute_low_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_absolute_mid_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_absolute_high_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_base_item_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(datas=[eve_d1], id_=eve_base_item_id, attrs={
        eve_roll_attr_id: 0,
        eve_absolute_low_attr_id: 0,
        eve_absolute_mid_attr_id: 0,
        eve_absolute_high_attr_id: 0})
    client.mk_eve_item(datas=[eve_d2], id_=eve_base_item_id, attrs={
        eve_roll_attr_id: 50,
        eve_absolute_low_attr_id: 50,
        eve_absolute_mid_attr_id: 50,
        eve_absolute_high_attr_id: 50})
    eve_mutated_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2])
    eve_mutator_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_mutator(
        datas=[eve_d1],
        id_=eve_mutator_id,
        # Valid input or output item is needed just to keep mutator data alive during cleanup
        items=[([client.mk_eve_item(datas=[eve_d1])], eve_mutated_item_id)],
        attrs={
            eve_roll_attr_id: (0.8, 1.2),
            eve_absolute_low_attr_id: (0.8, 1.2),
            eve_absolute_mid_attr_id: (0.8, 1.2),
            eve_absolute_high_attr_id: (0.8, 1.2)})
    client.mk_eve_mutator(
        datas=[eve_d2],
        id_=eve_mutator_id,
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={
            eve_roll_attr_id: (0.8, 1.2),
            eve_absolute_low_attr_id: (0.8, 1.2),
            eve_absolute_mid_attr_id: (0.8, 1.2),
            eve_absolute_high_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_roll_attr_id: {consts.ApiAttrMutation.roll: 0.7},
        eve_absolute_low_attr_id: {consts.ApiAttrMutation.absolute: -3},
        eve_absolute_mid_attr_id: {consts.ApiAttrMutation.absolute: 0},
        eve_absolute_high_attr_id: {consts.ApiAttrMutation.absolute: 6}}))
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    assert api_item.attrs[eve_roll_attr_id].base == approx(0)
    assert api_item.attrs[eve_absolute_low_attr_id].base == approx(0)
    assert api_item.attrs[eve_absolute_mid_attr_id].base == approx(0)
    assert api_item.attrs[eve_absolute_high_attr_id].base == approx(0)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - on first source lib couldn't interpret absolute values, so defaults are exposed
    api_item.update()
    assert len(api_item.mutation.attrs) == 4
    assert api_item.mutation.attrs[eve_roll_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_roll_attr_id].absolute == approx(54)
    assert api_item.mutation.attrs[eve_absolute_low_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_absolute_low_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_absolute_mid_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_absolute_mid_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_absolute_high_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_absolute_high_attr_id].absolute == approx(50)
    assert api_item.attrs[eve_roll_attr_id].base == approx(54)
    assert api_item.attrs[eve_absolute_low_attr_id].base == approx(50)
    assert api_item.attrs[eve_absolute_mid_attr_id].base == approx(50)
    assert api_item.attrs[eve_absolute_high_attr_id].base == approx(50)


def test_item_type_id(client):
    # Check that base item type ID is used
    eve_base_item_id = client.mk_eve_item()
    # Valid input or output item is needed just to keep mutator data alive during cleanup
    eve_mutator_id = client.mk_eve_mutator(items=[([client.mk_eve_item()], client.mk_eve_item())])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id, mutation=eve_mutator_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
