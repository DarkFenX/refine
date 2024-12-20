from tests import approx, check_no_field


def test_rolls_range(client, consts):
    # Check processing of roll values - within range and out of range
    eve_lower_attr_id = client.mk_eve_attr()
    eve_within_attr_id = client.mk_eve_attr()
    eve_higher_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(
        attrs={eve_lower_attr_id: 100, eve_within_attr_id: 100, eve_higher_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attributes={eve_lower_attr_id: (0.8, 1.2), eve_within_attr_id: (0.8, 1.2), eve_higher_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id, mutation=eve_mutator_id)
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 3
    assert api_item.mutation.attrs[eve_lower_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_lower_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_within_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_within_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_higher_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_higher_attr_id].absolute == approx(100)
    assert api_item.attrs[eve_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_higher_attr_id].base == approx(100)
    # Action
    api_item.change_mod(mutation={
        eve_lower_attr_id: {consts.ApiAttrMutation.roll: -5},
        eve_within_attr_id: {consts.ApiAttrMutation.roll: 0.3},
        eve_higher_attr_id: {consts.ApiAttrMutation.roll: 128}})
    # Verification
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


def test_absolute_base_attr_value(client, consts):
    # Check what is used as base attribute value for converting absolute value into roll
    eve_base_attr_id = client.mk_eve_attr()
    eve_overlap_attr_id = client.mk_eve_attr()
    eve_mutated_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_base_attr_id: 50, eve_overlap_attr_id: 70})
    eve_mutated_item_id = client.mk_eve_item(attrs={eve_overlap_attr_id: 80, eve_mutated_attr_id: 100})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attributes={eve_base_attr_id: (0.8, 1.2), eve_overlap_attr_id: (0.8, 1.2), eve_mutated_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id, mutation=eve_mutator_id)
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 3
    assert api_item.mutation.attrs[eve_base_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_base_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_overlap_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_overlap_attr_id].absolute == approx(80)
    assert api_item.mutation.attrs[eve_mutated_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_mutated_attr_id].absolute == approx(100)
    assert api_item.attrs[eve_base_attr_id].base == approx(50)
    assert api_item.attrs[eve_overlap_attr_id].base == approx(80)
    assert api_item.attrs[eve_mutated_attr_id].base == approx(100)
    # Action
    api_item.change_mod(mutation={
        eve_base_attr_id: {consts.ApiAttrMutation.absolute: 55},
        eve_overlap_attr_id: {consts.ApiAttrMutation.absolute: 75},
        eve_mutated_attr_id: {consts.ApiAttrMutation.absolute: 115}})
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 3
    assert api_item.mutation.attrs[eve_base_attr_id].roll == approx(0.75)
    assert api_item.mutation.attrs[eve_base_attr_id].absolute == approx(55)
    # For overlapping values, mutated item values should be taken; we check it here via roll value,
    # which is below 0.5 if base value is 80 instead of 70
    assert api_item.mutation.attrs[eve_overlap_attr_id].roll == approx(0.34375)
    assert api_item.mutation.attrs[eve_overlap_attr_id].absolute == approx(75)
    assert api_item.mutation.attrs[eve_mutated_attr_id].roll == approx(0.875)
    assert api_item.mutation.attrs[eve_mutated_attr_id].absolute == approx(115)
    assert api_item.attrs[eve_base_attr_id].base == approx(55)
    assert api_item.attrs[eve_overlap_attr_id].base == approx(75)
    assert api_item.attrs[eve_mutated_attr_id].base == approx(115)


def test_absolute_value_range(client, consts):
    # Check processing of absolute values - within range and out of range
    eve_lower_attr_id = client.mk_eve_attr()
    eve_within_attr_id = client.mk_eve_attr()
    eve_higher_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(
        attrs={eve_lower_attr_id: 100, eve_within_attr_id: 100, eve_higher_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attributes={eve_lower_attr_id: (0.8, 1.2), eve_within_attr_id: (0.8, 1.2), eve_higher_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id, mutation=eve_mutator_id)
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 3
    assert api_item.mutation.attrs[eve_lower_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_lower_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_within_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_within_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_higher_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_higher_attr_id].absolute == approx(100)
    assert api_item.attrs[eve_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_higher_attr_id].base == approx(100)
    # Action
    api_item.change_mod(mutation={
        eve_lower_attr_id: {consts.ApiAttrMutation.absolute: -53},
        eve_within_attr_id: {consts.ApiAttrMutation.absolute: 92},
        eve_higher_attr_id: {consts.ApiAttrMutation.absolute: 1009}})
    # Verification
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
