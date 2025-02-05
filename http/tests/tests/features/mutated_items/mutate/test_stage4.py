from tests import approx, check_no_field


def test_overwrite(client, consts):
    # Check results of overwriting one mutation by another
    eve_first_attr_id = client.mk_eve_attr()
    eve_second_attr_id = client.mk_eve_attr()
    eve_both_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(
        attrs={eve_first_attr_id: 100, eve_second_attr_id: 100, eve_both_attr_id: 100})
    eve_mutated_item1_id = client.mk_eve_item()
    eve_mutated_item2_id = client.mk_eve_item()
    eve_mutator1_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item1_id)],
        attrs={eve_first_attr_id: (0.8, 1.2), eve_both_attr_id: (0.7, 1.1)})
    eve_mutator2_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item2_id)],
        attrs={eve_second_attr_id: (0.8, 1.2), eve_both_attr_id: (0.9, 1.3)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id, mutation=(eve_mutator1_id, {
        eve_first_attr_id: {consts.ApiAttrMutation.roll: 0.7},
        eve_both_attr_id: {consts.ApiAttrMutation.roll: 0.3}}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item1_id
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_first_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_first_attr_id].absolute == approx(108)
    assert api_item.mutation.attrs[eve_both_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_both_attr_id].absolute == approx(82)
    assert api_item.attrs[eve_first_attr_id].base == approx(108)
    assert api_item.attrs[eve_second_attr_id].base == approx(100)
    assert api_item.attrs[eve_both_attr_id].base == approx(82)
    # Action
    api_item.change_mod(mutation=(eve_mutator2_id, {
        eve_second_attr_id: {consts.ApiAttrMutation.roll: 0.6},
        eve_both_attr_id: {consts.ApiAttrMutation.roll: 0.3}}))
    # Verification - first mutation only attribute is back to base value, attribute present on both
    # mutations has to update value against new mutation range, second mutation only attribute has
    # to expose mutated value
    api_item.update()
    assert api_item.type_id == eve_mutated_item2_id
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_second_attr_id].roll == approx(0.6)
    assert api_item.mutation.attrs[eve_second_attr_id].absolute == approx(104)
    assert api_item.mutation.attrs[eve_both_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_both_attr_id].absolute == approx(102)
    assert api_item.attrs[eve_first_attr_id].base == approx(100)
    assert api_item.attrs[eve_second_attr_id].base == approx(104)
    assert api_item.attrs[eve_both_attr_id].base == approx(102)


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
        attrs={eve_lower_attr_id: (0.8, 1.2), eve_within_attr_id: (0.8, 1.2), eve_higher_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id)
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_higher_attr_id].base == approx(100)
    # Action
    api_item.change_mod(mutation=(eve_mutator_id, {
        eve_lower_attr_id: {consts.ApiAttrMutation.roll: -5},
        eve_within_attr_id: {consts.ApiAttrMutation.roll: 0.3},
        eve_higher_attr_id: {consts.ApiAttrMutation.roll: 128}}))
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
        attrs={eve_base_attr_id: (0.8, 1.2), eve_overlap_attr_id: (0.8, 1.2), eve_mutated_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id)
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert len(api_item.attrs) == 2
    assert api_item.attrs[eve_base_attr_id].base == approx(50)
    assert api_item.attrs[eve_overlap_attr_id].base == approx(70)
    # Action
    api_item.change_mod(mutation=(eve_mutator_id, {
        eve_base_attr_id: {consts.ApiAttrMutation.absolute: 55},
        eve_overlap_attr_id: {consts.ApiAttrMutation.absolute: 75},
        eve_mutated_attr_id: {consts.ApiAttrMutation.absolute: 115}}))
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
        attrs={eve_lower_attr_id: (0.8, 1.2), eve_within_attr_id: (0.8, 1.2), eve_higher_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id)
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_higher_attr_id].base == approx(100)
    # Action
    api_item.change_mod(mutation=(eve_mutator_id, {
        eve_lower_attr_id: {consts.ApiAttrMutation.absolute: -53},
        eve_within_attr_id: {consts.ApiAttrMutation.absolute: 92},
        eve_higher_attr_id: {consts.ApiAttrMutation.absolute: 1009}}))
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


def test_no_base_item(client, consts):
    # Check that absolute mutations are accepted for items w/o base item using mutated item
    # attribute values
    eve_roll_attr_id = client.mk_eve_attr()
    eve_absolute_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.alloc_item_id()
    eve_mutated_item_id = client.mk_eve_item(attrs={eve_roll_attr_id: 50, eve_absolute_attr_id: 50})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_roll_attr_id: (0.8, 1.2), eve_absolute_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id)
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    with check_no_field():
        api_item.attrs  # noqa: B018
    # Action
    api_item.change_mod(mutation=(eve_mutator_id, {
        eve_roll_attr_id: {consts.ApiAttrMutation.roll: 0.7},
        eve_absolute_attr_id: {consts.ApiAttrMutation.absolute: 52}}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_roll_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_roll_attr_id].absolute == approx(54)
    assert api_item.mutation.attrs[eve_absolute_attr_id].roll == approx(0.6)
    assert api_item.mutation.attrs[eve_absolute_attr_id].absolute == approx(52)
    assert api_item.attrs[eve_roll_attr_id].base == approx(54)
    assert api_item.attrs[eve_absolute_attr_id].base == approx(52)


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
    eve_mutator_id = client.mk_eve_mutator(
        datas=[eve_d1, eve_d2],
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_roll_attr_id: (0.8, 1.2), eve_absolute_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id)
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    with check_no_field():
        api_item.attrs  # noqa: B018
    # Action
    api_item.change_mod(mutation=(eve_mutator_id, {
        eve_roll_attr_id: {consts.ApiAttrMutation.roll: 0.7},
        eve_absolute_attr_id: {consts.ApiAttrMutation.absolute: 54}}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 0
    with check_no_field():
        api_item.attrs  # noqa: B018
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - since there were no base attribute values on first source, attribute mutations
    # defined via absolute value were discarded. However, on second source roll and absolute value
    # are still exposed, but without mutation applied
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
        items=[([eve_base_item_id], eve_mutated_item_id)])
    client.mk_eve_mutator(
        datas=[eve_d2],
        id_=eve_mutator_id,
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_roll_attr_id: (0.8, 1.2), eve_absolute_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id)
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_roll_attr_id].base == approx(50)
    assert api_item.attrs[eve_absolute_attr_id].base == approx(50)
    # Action
    api_item.change_mod(mutation=(eve_mutator_id, {
        eve_roll_attr_id: {consts.ApiAttrMutation.roll: 0.7},
        eve_absolute_attr_id: {consts.ApiAttrMutation.absolute: 54}}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 0
    assert api_item.attrs[eve_roll_attr_id].base == approx(50)
    assert api_item.attrs[eve_absolute_attr_id].base == approx(50)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - since there was no mutation range on first source, attribute mutations defined
    # via absolute value were discarded. However, on second source absolute value is still exposed,
    # but without mutation applied
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
        items=[([eve_base_item_id], eve_mutated_item_id)],
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
    api_item = api_fit.add_mod(type_id=eve_base_item_id)
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_roll_attr_id].base == approx(50)
    assert api_item.attrs[eve_absolute_low_attr_id].base == approx(50)
    assert api_item.attrs[eve_absolute_mid_attr_id].base == approx(50)
    assert api_item.attrs[eve_absolute_high_attr_id].base == approx(50)
    # Action
    api_item.change_mod(mutation=(eve_mutator_id, {
        eve_roll_attr_id: {consts.ApiAttrMutation.roll: 0.7},
        eve_absolute_low_attr_id: {consts.ApiAttrMutation.absolute: 54},
        eve_absolute_mid_attr_id: {consts.ApiAttrMutation.absolute: 54},
        eve_absolute_high_attr_id: {consts.ApiAttrMutation.absolute: 54}}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 4
    assert api_item.mutation.attrs[eve_roll_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_roll_attr_id].absolute == approx(54)
    assert api_item.mutation.attrs[eve_absolute_low_attr_id].roll is None
    assert api_item.mutation.attrs[eve_absolute_low_attr_id].absolute == approx(46)
    assert api_item.mutation.attrs[eve_absolute_mid_attr_id].roll is None
    assert api_item.mutation.attrs[eve_absolute_mid_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_absolute_high_attr_id].roll is None
    assert api_item.mutation.attrs[eve_absolute_high_attr_id].absolute == approx(54)
    assert api_item.attrs[eve_roll_attr_id].base == approx(54)
    # Mutations requests were discarded - which will be verified later; here, something is still
    # returned, because it's just base value put onto range
    assert api_item.attrs[eve_absolute_low_attr_id].base == approx(46)
    assert api_item.attrs[eve_absolute_mid_attr_id].base == approx(50)
    assert api_item.attrs[eve_absolute_high_attr_id].base == approx(54)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - since mutation ranges had zero width on first source, attribute mutations
    # defined via absolute value were discarded. However, on second source absolute value are still
    # exposed, but without mutation applied
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
    eve_mutator_id = client.mk_eve_mutator(
        datas=[eve_d1, eve_d2],
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={
            eve_roll_attr_id: (0.8, 1.2),
            eve_absolute_low_attr_id: (0.8, 1.2),
            eve_absolute_mid_attr_id: (0.8, 1.2),
            eve_absolute_high_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id)
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_roll_attr_id].base == approx(0)
    assert api_item.attrs[eve_absolute_low_attr_id].base == approx(0)
    assert api_item.attrs[eve_absolute_mid_attr_id].base == approx(0)
    assert api_item.attrs[eve_absolute_high_attr_id].base == approx(0)
    # Action
    api_item.change_mod(mutation=(eve_mutator_id, {
        eve_roll_attr_id: {consts.ApiAttrMutation.roll: 0.7},
        eve_absolute_low_attr_id: {consts.ApiAttrMutation.absolute: -3},
        eve_absolute_mid_attr_id: {consts.ApiAttrMutation.absolute: 0},
        eve_absolute_high_attr_id: {consts.ApiAttrMutation.absolute: 6}}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 4
    assert api_item.mutation.attrs[eve_roll_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_roll_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_absolute_low_attr_id].roll is None
    assert api_item.mutation.attrs[eve_absolute_low_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_absolute_mid_attr_id].roll is None
    assert api_item.mutation.attrs[eve_absolute_mid_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_absolute_high_attr_id].roll is None
    assert api_item.mutation.attrs[eve_absolute_high_attr_id].absolute == approx(0)
    assert api_item.attrs[eve_roll_attr_id].base == approx(0)
    assert api_item.attrs[eve_absolute_low_attr_id].base == approx(0)
    assert api_item.attrs[eve_absolute_mid_attr_id].base == approx(0)
    assert api_item.attrs[eve_absolute_high_attr_id].base == approx(0)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - since base value was zero on first source, attribute mutations defined via
    # absolute value were discarded. However, on second source absolute value are still exposed, but
    # without mutation applied
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


def test_base_out_of_range(client):
    # Check which value is exposed if base value is out of range (as of 2024-12-11, a thing for
    # decayed disruptor rolls)
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr1_id: 100, eve_attr2_id: 100})
    eve_mutated_item_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_attr1_id: (1.1, 1.3), eve_attr2_id: (0.7, 0.9)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id)
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_attr1_id].base == approx(100)
    assert api_item.attrs[eve_attr2_id].base == approx(100)
    # Action
    api_item.change_mod(mutation=eve_mutator_id)
    # Verification - all the values are put onto mutation range
    api_item.update()
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_attr1_id].roll == approx(0)
    assert api_item.mutation.attrs[eve_attr1_id].absolute == approx(110)
    assert api_item.mutation.attrs[eve_attr2_id].roll == approx(1)
    assert api_item.mutation.attrs[eve_attr2_id].absolute == approx(90)
    assert api_item.attrs[eve_attr1_id].base == approx(110)
    assert api_item.attrs[eve_attr2_id].base == approx(90)


def test_modification(client, consts):
    # Check that mutated value is used as base for source and target of modifications
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_base_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20, eve_affectee_attr_id: 200})
    eve_mutated_item_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_affector_attr_id: (0.8, 1.2), eve_affectee_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id)
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_affector_attr_id].base == approx(20)
    assert api_item.attrs[eve_affector_attr_id].dogma == approx(20)
    assert api_item.attrs[eve_affectee_attr_id].base == approx(200)
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(200)
    # Action
    api_item.change_mod(mutation=(eve_mutator_id, {
        eve_affector_attr_id: {consts.ApiAttrMutation.roll: 0.2},
        eve_affectee_attr_id: {consts.ApiAttrMutation.roll: 0.8}}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_affector_attr_id].roll == approx(0.2)
    assert api_item.mutation.attrs[eve_affector_attr_id].absolute == approx(17.6)
    assert api_item.mutation.attrs[eve_affectee_attr_id].roll == approx(0.8)
    assert api_item.mutation.attrs[eve_affectee_attr_id].absolute == approx(224)
    assert api_item.attrs[eve_affector_attr_id].base == approx(17.6)
    assert api_item.attrs[eve_affector_attr_id].dogma == approx(17.6)
    assert api_item.attrs[eve_affectee_attr_id].base == approx(224)
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(263.424)


def test_item_type_id(client):
    # Check that mutated item type ID is used
    eve_base_item_id = client.mk_eve_item()
    eve_mutated_item_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    # Action
    api_item.change_mod(mutation=eve_mutator_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id


def test_item_group(client, consts):
    # Check that mutated item group is used
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr1_id = client.mk_eve_attr()
    eve_affectee_attr2_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp1_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr1_id)
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp2_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr2_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod1, eve_mod2])
    eve_implant_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_base_item_id = client.mk_eve_item(
        grp_id=eve_grp1_id,
        attrs={eve_affectee_attr1_id: 100, eve_affectee_attr2_id: 100})
    eve_mutated_item_id = client.mk_eve_item(grp_id=eve_grp2_id)
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_affectee_attr1_id: (0.8, 1.2), eve_affectee_attr2_id: (0.8, 1.2)})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_item = api_fit.add_mod(type_id=eve_base_item_id)
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_affectee_attr1_id].base == approx(100)
    assert api_item.attrs[eve_affectee_attr1_id].dogma == approx(120)  # Modified
    assert api_item.attrs[eve_affectee_attr2_id].base == approx(100)
    assert api_item.attrs[eve_affectee_attr2_id].dogma == approx(100)  # Not modified
    # Action
    api_item.change_mod(mutation=(eve_mutator_id, {
        eve_affectee_attr1_id: {consts.ApiAttrMutation.roll: 0.2},
        eve_affectee_attr2_id: {consts.ApiAttrMutation.roll: 0.8}}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_affectee_attr1_id].roll == approx(0.2)
    assert api_item.mutation.attrs[eve_affectee_attr1_id].absolute == approx(88)
    assert api_item.mutation.attrs[eve_affectee_attr2_id].roll == approx(0.8)
    assert api_item.mutation.attrs[eve_affectee_attr2_id].absolute == approx(112)
    assert api_item.attrs[eve_affectee_attr1_id].base == approx(88)
    assert api_item.attrs[eve_affectee_attr1_id].dogma == approx(88)  # Not modified
    assert api_item.attrs[eve_affectee_attr2_id].base == approx(112)
    assert api_item.attrs[eve_affectee_attr2_id].dogma == approx(134.4)  # Modified


def test_item_category(client, consts):
    # Check that mutated item category is used
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(stackable=False)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_base_item_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id])
    eve_mutated_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, eff_ids=[eve_effect_id])
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)], attrs={})
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_item1 = api_fit.add_mod(type_id=eve_base_item_id)
    api_item2 = api_fit.add_mod(type_id=eve_base_item_id)
    # Verification - base item is not from stacking penalty immune category, so calculation is
    # stacking penalized.
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr_id].dogma == approx(140.85888)
    # Action
    api_item1.change_mod(mutation=eve_mutator_id)
    api_item2.change_mod(mutation=eve_mutator_id)
    # Verification - value is 144 because change is non-penalized thanks to implant category of
    # mutated item.
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr_id].dogma == approx(144)


def test_item_skillreqs(client, consts):
    # Check that mutated item skill requirements are used
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr1_id = client.mk_eve_attr()
    eve_affectee_attr2_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_srq,
        loc=consts.EveModLoc.ship,
        srq=eve_skill1_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr1_id)
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_srq,
        loc=consts.EveModLoc.ship,
        srq=eve_skill2_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr2_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod1, eve_mod2])
    eve_implant_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_base_item_id = client.mk_eve_item(
        attrs={eve_affectee_attr1_id: 100, eve_affectee_attr2_id: 100},
        srqs={eve_skill1_id: 1})
    eve_mutated_item_id = client.mk_eve_item(srqs={eve_skill2_id: 1})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_affectee_attr1_id: (0.8, 1.2), eve_affectee_attr2_id: (0.8, 1.2)})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_item = api_fit.add_mod(type_id=eve_base_item_id)
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_affectee_attr1_id].base == approx(100)
    assert api_item.attrs[eve_affectee_attr1_id].dogma == approx(120)  # Modified
    assert api_item.attrs[eve_affectee_attr2_id].base == approx(100)
    assert api_item.attrs[eve_affectee_attr2_id].dogma == approx(100)  # Not modified
    # Action
    api_item.change_mod(mutation=(eve_mutator_id, {
        eve_affectee_attr1_id: {consts.ApiAttrMutation.roll: 0.2},
        eve_affectee_attr2_id: {consts.ApiAttrMutation.roll: 0.8}}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_affectee_attr1_id].roll == approx(0.2)
    assert api_item.mutation.attrs[eve_affectee_attr1_id].absolute == approx(88)
    assert api_item.mutation.attrs[eve_affectee_attr2_id].roll == approx(0.8)
    assert api_item.mutation.attrs[eve_affectee_attr2_id].absolute == approx(112)
    assert api_item.attrs[eve_affectee_attr1_id].base == approx(88)
    assert api_item.attrs[eve_affectee_attr1_id].dogma == approx(88)  # Not modified
    assert api_item.attrs[eve_affectee_attr2_id].base == approx(112)
    assert api_item.attrs[eve_affectee_attr2_id].dogma == approx(134.4)  # Modified


def test_item_effects(client, consts):
    # Check that mutated item effects are used
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr1_id = client.mk_eve_attr()
    eve_affectee_attr2_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr1_id)
    eve_effect1_id = client.mk_eve_effect(mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr2_id)
    eve_effect2_id = client.mk_eve_effect(mod_info=[eve_mod2])
    eve_base_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect1_id])
    eve_mutated_item_id = client.mk_eve_item(eff_ids=[eve_effect2_id])
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)], attrs={})
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr1_id: 100, eve_affectee_attr2_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_item = api_fit.add_mod(type_id=eve_base_item_id)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr1_id].dogma == approx(120)  # Modified
    assert api_ship.attrs[eve_affectee_attr2_id].dogma == approx(100)  # Not modified
    # Action
    api_item.change_mod(mutation=eve_mutator_id)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr1_id].dogma == approx(100)  # Not modified
    assert api_ship.attrs[eve_affectee_attr2_id].dogma == approx(120)  # Modified


def test_item_default_effect(client, consts):
    # Check that mutated item effects are used
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr1_id = client.mk_eve_attr()
    eve_affectee_attr2_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr1_id)
    eve_effect1_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr2_id)
    eve_effect2_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod2])
    eve_base_item_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect1_id, eve_effect2_id],
        defeff_id=eve_effect1_id)
    eve_mutated_item_id = client.mk_eve_item(eff_ids=[eve_effect1_id, eve_effect2_id], defeff_id=eve_effect2_id)
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)], attrs={})
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr1_id: 100, eve_affectee_attr2_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_item = api_fit.add_mod(type_id=eve_base_item_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr1_id].dogma == approx(120)  # Modified
    assert api_ship.attrs[eve_affectee_attr2_id].dogma == approx(100)  # Not modified
    # Action
    api_item.change_mod(mutation=eve_mutator_id)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr1_id].dogma == approx(100)  # Not modified
    assert api_ship.attrs[eve_affectee_attr2_id].dogma == approx(120)  # Modified


def test_drone(client, consts):
    eve_roll_attr_id = client.mk_eve_attr()
    eve_absolute_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_roll_attr_id: 100, eve_absolute_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_roll_attr_id: (0.8, 1.2), eve_absolute_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_drone(type_id=eve_base_item_id)
    # Verification
    api_item.update()
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_roll_attr_id].base == approx(100)
    assert api_item.attrs[eve_absolute_attr_id].base == approx(100)
    # Action
    api_item.change_drone(mutation=(eve_mutator_id, {
        eve_roll_attr_id: {consts.ApiAttrMutation.roll: 0.3},
        eve_absolute_attr_id: {consts.ApiAttrMutation.absolute: 105}}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_roll_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_roll_attr_id].absolute == approx(92)
    assert api_item.mutation.attrs[eve_absolute_attr_id].roll == approx(0.625)
    assert api_item.mutation.attrs[eve_absolute_attr_id].absolute == approx(105)
    assert api_item.attrs[eve_roll_attr_id].base == approx(92)
    assert api_item.attrs[eve_absolute_attr_id].base == approx(105)
