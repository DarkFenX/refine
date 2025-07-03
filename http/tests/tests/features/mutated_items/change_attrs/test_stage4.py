from tests import approx, check_no_field, muta_abs_to_api, muta_roll_to_api


def test_rolls_range(client):
    # Check processing of roll values - within range and out of range
    eve_add_lower_attr_id = client.mk_eve_attr()
    eve_add_within_attr_id = client.mk_eve_attr()
    eve_add_higher_attr_id = client.mk_eve_attr()
    eve_change_lower_attr_id = client.mk_eve_attr()
    eve_change_within_attr_id = client.mk_eve_attr()
    eve_change_higher_attr_id = client.mk_eve_attr()
    eve_remove_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={
        eve_add_lower_attr_id: 100,
        eve_add_within_attr_id: 100,
        eve_add_higher_attr_id: 100,
        eve_change_lower_attr_id: 100,
        eve_change_within_attr_id: 100,
        eve_change_higher_attr_id: 100,
        eve_remove_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)], attrs={
        eve_add_lower_attr_id: (0.8, 1.2),
        eve_add_within_attr_id: (0.8, 1.2),
        eve_add_higher_attr_id: (0.8, 1.2),
        eve_change_lower_attr_id: (0.8, 1.2),
        eve_change_within_attr_id: (0.8, 1.2),
        eve_change_higher_attr_id: (0.8, 1.2),
        eve_remove_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_change_lower_attr_id: muta_roll_to_api(val=111),
        eve_change_within_attr_id: muta_roll_to_api(val=0.6),
        eve_change_higher_attr_id: muta_roll_to_api(val=-8),
        eve_remove_attr_id: muta_roll_to_api(val=0.8)}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 7
    assert api_item.mutation.attrs[eve_add_lower_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_lower_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_add_within_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_within_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_add_higher_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_higher_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_change_lower_attr_id].roll == approx(1)
    assert api_item.mutation.attrs[eve_change_lower_attr_id].absolute == approx(120)
    assert api_item.mutation.attrs[eve_change_within_attr_id].roll == approx(0.6)
    assert api_item.mutation.attrs[eve_change_within_attr_id].absolute == approx(104)
    assert api_item.mutation.attrs[eve_change_higher_attr_id].roll == approx(0)
    assert api_item.mutation.attrs[eve_change_higher_attr_id].absolute == approx(80)
    assert api_item.mutation.attrs[eve_remove_attr_id].roll == approx(0.8)
    assert api_item.mutation.attrs[eve_remove_attr_id].absolute == approx(112)
    assert api_item.attrs[eve_add_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_higher_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_lower_attr_id].base == approx(120)
    assert api_item.attrs[eve_change_within_attr_id].base == approx(104)
    assert api_item.attrs[eve_change_higher_attr_id].base == approx(80)
    assert api_item.attrs[eve_remove_attr_id].base == approx(112)
    # Action
    api_item.change_module(mutation={
        eve_add_lower_attr_id: muta_roll_to_api(val=-5),
        eve_add_within_attr_id: muta_roll_to_api(val=0.3),
        eve_add_higher_attr_id: muta_roll_to_api(val=128),
        eve_change_lower_attr_id: muta_roll_to_api(val=-60),
        eve_change_within_attr_id: muta_roll_to_api(val=0.1),
        eve_change_higher_attr_id: muta_roll_to_api(val=1.1),
        eve_remove_attr_id: None})
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


def test_absolute_base_attr_value(client):
    # Check what is used as base attribute value for converting absolute value into roll
    eve_add_base_attr_id = client.mk_eve_attr()
    eve_add_overlap_attr_id = client.mk_eve_attr()
    eve_add_mutated_attr_id = client.mk_eve_attr()
    eve_change_base_attr_id = client.mk_eve_attr()
    eve_change_overlap_attr_id = client.mk_eve_attr()
    eve_change_mutated_attr_id = client.mk_eve_attr()
    eve_remove_base_attr_id = client.mk_eve_attr()
    eve_remove_overlap_attr_id = client.mk_eve_attr()
    eve_remove_mutated_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={
        eve_add_base_attr_id: 50,
        eve_add_overlap_attr_id: 70,
        eve_change_base_attr_id: 50,
        eve_change_overlap_attr_id: 70,
        eve_remove_base_attr_id: 50,
        eve_remove_overlap_attr_id: 70})
    eve_mutated_item_id = client.mk_eve_item(attrs={
        eve_add_overlap_attr_id: 80,
        eve_add_mutated_attr_id: 100,
        eve_change_overlap_attr_id: 80,
        eve_change_mutated_attr_id: 100,
        eve_remove_overlap_attr_id: 80,
        eve_remove_mutated_attr_id: 100})
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)], attrs={
        eve_add_base_attr_id: (0.8, 1.2),
        eve_add_overlap_attr_id: (0.8, 1.2),
        eve_add_mutated_attr_id: (0.8, 1.2),
        eve_change_base_attr_id: (0.8, 1.2),
        eve_change_overlap_attr_id: (0.8, 1.2),
        eve_change_mutated_attr_id: (0.8, 1.2),
        eve_remove_base_attr_id: (0.8, 1.2),
        eve_remove_overlap_attr_id: (0.8, 1.2),
        eve_remove_mutated_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_change_base_attr_id: muta_abs_to_api(val=55),
        eve_change_overlap_attr_id: muta_abs_to_api(val=85),
        eve_change_mutated_attr_id: muta_abs_to_api(val=115),
        eve_remove_base_attr_id: muta_abs_to_api(val=55),
        eve_remove_overlap_attr_id: muta_abs_to_api(val=75),
        eve_remove_mutated_attr_id: muta_abs_to_api(val=115)}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 9
    assert api_item.mutation.attrs[eve_add_base_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_base_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_add_overlap_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_overlap_attr_id].absolute == approx(80)
    assert api_item.mutation.attrs[eve_add_mutated_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_mutated_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_change_base_attr_id].roll == approx(0.75)
    assert api_item.mutation.attrs[eve_change_base_attr_id].absolute == approx(55)
    assert api_item.mutation.attrs[eve_change_overlap_attr_id].roll == approx(0.65625)
    assert api_item.mutation.attrs[eve_change_overlap_attr_id].absolute == approx(85)
    assert api_item.mutation.attrs[eve_change_mutated_attr_id].roll == approx(0.875)
    assert api_item.mutation.attrs[eve_change_mutated_attr_id].absolute == approx(115)
    assert api_item.mutation.attrs[eve_remove_base_attr_id].roll == approx(0.75)
    assert api_item.mutation.attrs[eve_remove_base_attr_id].absolute == approx(55)
    assert api_item.mutation.attrs[eve_remove_overlap_attr_id].roll == approx(0.34375)
    assert api_item.mutation.attrs[eve_remove_overlap_attr_id].absolute == approx(75)
    assert api_item.mutation.attrs[eve_remove_mutated_attr_id].roll == approx(0.875)
    assert api_item.mutation.attrs[eve_remove_mutated_attr_id].absolute == approx(115)
    assert api_item.attrs[eve_add_base_attr_id].base == approx(50)
    assert api_item.attrs[eve_add_overlap_attr_id].base == approx(80)
    assert api_item.attrs[eve_add_mutated_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_base_attr_id].base == approx(55)
    assert api_item.attrs[eve_change_overlap_attr_id].base == approx(85)
    assert api_item.attrs[eve_change_mutated_attr_id].base == approx(115)
    assert api_item.attrs[eve_remove_base_attr_id].base == approx(55)
    assert api_item.attrs[eve_remove_overlap_attr_id].base == approx(75)
    assert api_item.attrs[eve_remove_mutated_attr_id].base == approx(115)
    # Action
    api_item.change_module(mutation={
        eve_add_base_attr_id: muta_abs_to_api(val=55),
        eve_add_overlap_attr_id: muta_abs_to_api(val=85),
        eve_add_mutated_attr_id: muta_abs_to_api(val=115),
        eve_change_base_attr_id: muta_abs_to_api(val=45),
        eve_change_overlap_attr_id: muta_abs_to_api(val=75),
        eve_change_mutated_attr_id: muta_abs_to_api(val=85),
        eve_remove_base_attr_id: None,
        eve_remove_overlap_attr_id: None,
        eve_remove_mutated_attr_id: None})
    # Verification - for overlapping values, mutated item values should be taken, we check it
    # indirectly via roll values
    api_item.update()
    assert len(api_item.mutation.attrs) == 9
    assert api_item.mutation.attrs[eve_add_base_attr_id].roll == approx(0.75)
    assert api_item.mutation.attrs[eve_add_base_attr_id].absolute == approx(55)
    assert api_item.mutation.attrs[eve_add_overlap_attr_id].roll == approx(0.65625)
    assert api_item.mutation.attrs[eve_add_overlap_attr_id].absolute == approx(85)
    assert api_item.mutation.attrs[eve_add_mutated_attr_id].roll == approx(0.875)
    assert api_item.mutation.attrs[eve_add_mutated_attr_id].absolute == approx(115)
    assert api_item.mutation.attrs[eve_change_base_attr_id].roll == approx(0.25)
    assert api_item.mutation.attrs[eve_change_base_attr_id].absolute == approx(45)
    assert api_item.mutation.attrs[eve_change_overlap_attr_id].roll == approx(0.34375)
    assert api_item.mutation.attrs[eve_change_overlap_attr_id].absolute == approx(75)
    assert api_item.mutation.attrs[eve_change_mutated_attr_id].roll == approx(0.125)
    assert api_item.mutation.attrs[eve_change_mutated_attr_id].absolute == approx(85)
    assert api_item.mutation.attrs[eve_remove_base_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_remove_base_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_remove_overlap_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_remove_overlap_attr_id].absolute == approx(80)
    assert api_item.mutation.attrs[eve_remove_mutated_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_remove_mutated_attr_id].absolute == approx(100)
    assert api_item.attrs[eve_add_base_attr_id].base == approx(55)
    assert api_item.attrs[eve_add_overlap_attr_id].base == approx(85)
    assert api_item.attrs[eve_add_mutated_attr_id].base == approx(115)
    assert api_item.attrs[eve_change_base_attr_id].base == approx(45)
    assert api_item.attrs[eve_change_overlap_attr_id].base == approx(75)
    assert api_item.attrs[eve_change_mutated_attr_id].base == approx(85)
    assert api_item.attrs[eve_remove_base_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_overlap_attr_id].base == approx(80)
    assert api_item.attrs[eve_remove_mutated_attr_id].base == approx(100)


def test_absolute_value_range(client):
    # Check processing of absolute values - within range and out of range
    eve_add_lower_attr_id = client.mk_eve_attr()
    eve_add_within_attr_id = client.mk_eve_attr()
    eve_add_higher_attr_id = client.mk_eve_attr()
    eve_change_lower_attr_id = client.mk_eve_attr()
    eve_change_within_attr_id = client.mk_eve_attr()
    eve_change_higher_attr_id = client.mk_eve_attr()
    eve_remove_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={
        eve_add_lower_attr_id: 100,
        eve_add_within_attr_id: 100,
        eve_add_higher_attr_id: 100,
        eve_change_lower_attr_id: 100,
        eve_change_within_attr_id: 100,
        eve_change_higher_attr_id: 100,
        eve_remove_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)], attrs={
        eve_add_lower_attr_id: (0.8, 1.2),
        eve_add_within_attr_id: (0.8, 1.2),
        eve_add_higher_attr_id: (0.8, 1.2),
        eve_change_lower_attr_id: (0.8, 1.2),
        eve_change_within_attr_id: (0.8, 1.2),
        eve_change_higher_attr_id: (0.8, 1.2),
        eve_remove_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_change_lower_attr_id: muta_abs_to_api(val=260),
        eve_change_within_attr_id: muta_abs_to_api(val=104),
        eve_change_higher_attr_id: muta_abs_to_api(val=0.5),
        eve_remove_attr_id: muta_abs_to_api(val=112)}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 7
    assert api_item.mutation.attrs[eve_add_lower_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_lower_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_add_within_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_within_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_add_higher_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_higher_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_change_lower_attr_id].roll == approx(1)
    assert api_item.mutation.attrs[eve_change_lower_attr_id].absolute == approx(120)
    assert api_item.mutation.attrs[eve_change_within_attr_id].roll == approx(0.6)
    assert api_item.mutation.attrs[eve_change_within_attr_id].absolute == approx(104)
    assert api_item.mutation.attrs[eve_change_higher_attr_id].roll == approx(0)
    assert api_item.mutation.attrs[eve_change_higher_attr_id].absolute == approx(80)
    assert api_item.mutation.attrs[eve_remove_attr_id].roll == approx(0.8)
    assert api_item.mutation.attrs[eve_remove_attr_id].absolute == approx(112)
    assert api_item.attrs[eve_add_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_higher_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_lower_attr_id].base == approx(120)
    assert api_item.attrs[eve_change_within_attr_id].base == approx(104)
    assert api_item.attrs[eve_change_higher_attr_id].base == approx(80)
    assert api_item.attrs[eve_remove_attr_id].base == approx(112)
    # Action
    api_item.change_module(mutation={
        eve_add_lower_attr_id: muta_abs_to_api(val=-502),
        eve_add_within_attr_id: muta_abs_to_api(val=92),
        eve_add_higher_attr_id: muta_abs_to_api(val=1001),
        eve_change_lower_attr_id: muta_abs_to_api(val=0),
        eve_change_within_attr_id: muta_abs_to_api(val=84),
        eve_change_higher_attr_id: muta_abs_to_api(val=130),
        eve_remove_attr_id: None})
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


def test_no_base_item(client):
    # Check that absolute mutations are accepted for items w/o base item using mutated item
    # attribute values
    eve_add_roll_attr_id = client.mk_eve_attr()
    eve_add_absolute_attr_id = client.mk_eve_attr()
    eve_change_roll_attr_id = client.mk_eve_attr()
    eve_change_absolute_attr_id = client.mk_eve_attr()
    eve_remove_roll_attr_id = client.mk_eve_attr()
    eve_remove_absolute_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.alloc_item_id()
    eve_mutated_item_id = client.mk_eve_item(attrs={
        eve_add_roll_attr_id: 50,
        eve_add_absolute_attr_id: 50,
        eve_change_roll_attr_id: 50,
        eve_change_absolute_attr_id: 50,
        eve_remove_roll_attr_id: 50,
        eve_remove_absolute_attr_id: 50})
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)], attrs={
        eve_add_roll_attr_id: (0.8, 1.2),
        eve_add_absolute_attr_id: (0.8, 1.2),
        eve_change_roll_attr_id: (0.8, 1.2),
        eve_change_absolute_attr_id: (0.8, 1.2),
        eve_remove_roll_attr_id: (0.8, 1.2),
        eve_remove_absolute_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_change_roll_attr_id: muta_roll_to_api(val=0.7),
        eve_change_absolute_attr_id: muta_abs_to_api(val=52),
        eve_remove_roll_attr_id: muta_roll_to_api(val=0.8),
        eve_remove_absolute_attr_id: muta_abs_to_api(val=55)}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 6
    assert api_item.mutation.attrs[eve_add_roll_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_roll_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_add_absolute_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_absolute_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].absolute == approx(54)
    assert api_item.mutation.attrs[eve_change_absolute_attr_id].roll == approx(0.6)
    assert api_item.mutation.attrs[eve_change_absolute_attr_id].absolute == approx(52)
    assert api_item.mutation.attrs[eve_remove_roll_attr_id].roll == approx(0.8)
    assert api_item.mutation.attrs[eve_remove_roll_attr_id].absolute == approx(56)
    assert api_item.mutation.attrs[eve_remove_absolute_attr_id].roll == approx(0.75)
    assert api_item.mutation.attrs[eve_remove_absolute_attr_id].absolute == approx(55)
    assert api_item.attrs[eve_add_roll_attr_id].base == approx(50)
    assert api_item.attrs[eve_add_absolute_attr_id].base == approx(50)
    assert api_item.attrs[eve_change_roll_attr_id].base == approx(54)
    assert api_item.attrs[eve_change_absolute_attr_id].base == approx(52)
    assert api_item.attrs[eve_remove_roll_attr_id].base == approx(56)
    assert api_item.attrs[eve_remove_absolute_attr_id].base == approx(55)
    # Action
    api_item.change_module(mutation={
        eve_add_roll_attr_id: muta_roll_to_api(val=0.9),
        eve_add_absolute_attr_id: muta_abs_to_api(val=59),
        eve_change_roll_attr_id: muta_roll_to_api(val=0.3),
        eve_change_absolute_attr_id: muta_abs_to_api(val=48),
        eve_remove_roll_attr_id: None,
        eve_remove_absolute_attr_id: None})
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 6
    assert api_item.mutation.attrs[eve_add_roll_attr_id].roll == approx(0.9)
    assert api_item.mutation.attrs[eve_add_roll_attr_id].absolute == approx(58)
    assert api_item.mutation.attrs[eve_add_absolute_attr_id].roll == approx(0.95)
    assert api_item.mutation.attrs[eve_add_absolute_attr_id].absolute == approx(59)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].absolute == approx(46)
    assert api_item.mutation.attrs[eve_change_absolute_attr_id].roll == approx(0.4)
    assert api_item.mutation.attrs[eve_change_absolute_attr_id].absolute == approx(48)
    assert api_item.mutation.attrs[eve_remove_roll_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_remove_roll_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_remove_absolute_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_remove_absolute_attr_id].absolute == approx(50)
    assert api_item.attrs[eve_add_roll_attr_id].base == approx(58)
    assert api_item.attrs[eve_add_absolute_attr_id].base == approx(59)
    assert api_item.attrs[eve_change_roll_attr_id].base == approx(46)
    assert api_item.attrs[eve_change_absolute_attr_id].base == approx(48)
    assert api_item.attrs[eve_remove_roll_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_absolute_attr_id].base == approx(50)


def test_no_base_value(client):
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
    eve_mutator_id = client.mk_eve_mutator(
        datas=[eve_d1, eve_d2],
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
        eve_change_roll_attr_id: muta_roll_to_api(val=0.3),
        eve_change_absolute_attr_id: muta_abs_to_api(val=46),
        eve_remove_attr_id: muta_roll_to_api(val=0.2)}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 0
    with check_no_field():
        api_item.attrs  # noqa: B018
    # Action
    api_item.change_module(mutation={
        eve_add_roll_attr_id: muta_roll_to_api(val=0.7),
        eve_add_absolute_attr_id: muta_abs_to_api(val=54),
        eve_change_roll_attr_id: muta_roll_to_api(val=0.4),
        eve_change_absolute_attr_id: muta_abs_to_api(val=45),
        eve_remove_attr_id: None})
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


def test_no_mutation_range(client):
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
        items=[([eve_base_item_id], eve_mutated_item_id)])
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
        eve_change_roll_attr_id: muta_roll_to_api(val=0.3),
        eve_change_absolute_attr_id: muta_abs_to_api(val=46),
        eve_remove_attr_id: muta_roll_to_api(val=0.2)}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 0
    assert api_item.attrs[eve_add_roll_attr_id].base == approx(50)
    assert api_item.attrs[eve_add_absolute_attr_id].base == approx(50)
    assert api_item.attrs[eve_change_roll_attr_id].base == approx(50)
    assert api_item.attrs[eve_change_absolute_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_attr_id].base == approx(50)
    # Action
    api_item.change_module(mutation={
        eve_add_roll_attr_id: muta_roll_to_api(val=0.7),
        eve_add_absolute_attr_id: muta_abs_to_api(val=54),
        eve_change_roll_attr_id: muta_roll_to_api(val=0.4),
        eve_change_absolute_attr_id: muta_abs_to_api(val=45),
        eve_remove_attr_id: None})
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 0
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


def test_zero_mutation_range(client):
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
        items=[([eve_base_item_id], eve_mutated_item_id)],
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
        eve_change_roll_attr_id: muta_roll_to_api(val=0.7),
        eve_change_absolute_low_attr_id: muta_abs_to_api(val=-54),
        eve_change_absolute_mid_attr_id: muta_abs_to_api(val=-54),
        eve_change_absolute_high_attr_id: muta_abs_to_api(val=-54),
        eve_remove_roll_attr_id: muta_roll_to_api(val=0.4),
        eve_remove_absolute_low_attr_id: muta_abs_to_api(val=54),
        eve_remove_absolute_mid_attr_id: muta_abs_to_api(val=54),
        eve_remove_absolute_high_attr_id: muta_abs_to_api(val=54)}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 12
    assert api_item.mutation.attrs[eve_add_roll_attr_id].roll is None
    assert api_item.mutation.attrs[eve_add_roll_attr_id].absolute == approx(54)
    assert api_item.mutation.attrs[eve_add_absolute_low_attr_id].roll is None
    assert api_item.mutation.attrs[eve_add_absolute_low_attr_id].absolute == approx(46)
    assert api_item.mutation.attrs[eve_add_absolute_mid_attr_id].roll is None
    assert api_item.mutation.attrs[eve_add_absolute_mid_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_add_absolute_high_attr_id].roll is None
    assert api_item.mutation.attrs[eve_add_absolute_high_attr_id].absolute == approx(54)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].absolute == approx(-46)
    assert api_item.mutation.attrs[eve_change_absolute_low_attr_id].roll is None
    assert api_item.mutation.attrs[eve_change_absolute_low_attr_id].absolute == approx(-46)
    assert api_item.mutation.attrs[eve_change_absolute_mid_attr_id].roll is None
    assert api_item.mutation.attrs[eve_change_absolute_mid_attr_id].absolute == approx(-50)
    assert api_item.mutation.attrs[eve_change_absolute_high_attr_id].roll is None
    assert api_item.mutation.attrs[eve_change_absolute_high_attr_id].absolute == approx(-54)
    assert api_item.mutation.attrs[eve_remove_roll_attr_id].roll == approx(0.4)
    assert api_item.mutation.attrs[eve_remove_roll_attr_id].absolute == approx(54)
    assert api_item.mutation.attrs[eve_remove_absolute_low_attr_id].roll is None
    assert api_item.mutation.attrs[eve_remove_absolute_low_attr_id].absolute == approx(46)
    assert api_item.mutation.attrs[eve_remove_absolute_mid_attr_id].roll is None
    assert api_item.mutation.attrs[eve_remove_absolute_mid_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_remove_absolute_high_attr_id].roll is None
    assert api_item.mutation.attrs[eve_remove_absolute_high_attr_id].absolute == approx(54)
    assert api_item.attrs[eve_add_roll_attr_id].base == approx(54)
    assert api_item.attrs[eve_add_absolute_low_attr_id].base == approx(46)
    assert api_item.attrs[eve_add_absolute_mid_attr_id].base == approx(50)
    assert api_item.attrs[eve_add_absolute_high_attr_id].base == approx(54)
    assert api_item.attrs[eve_change_roll_attr_id].base == approx(-46)
    assert api_item.attrs[eve_change_absolute_low_attr_id].base == approx(-46)
    assert api_item.attrs[eve_change_absolute_mid_attr_id].base == approx(-50)
    assert api_item.attrs[eve_change_absolute_high_attr_id].base == approx(-54)
    assert api_item.attrs[eve_remove_roll_attr_id].base == approx(54)
    assert api_item.attrs[eve_remove_absolute_low_attr_id].base == approx(46)
    assert api_item.attrs[eve_remove_absolute_mid_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_absolute_high_attr_id].base == approx(54)
    # Action
    api_item.change_module(mutation={
        eve_add_roll_attr_id: muta_roll_to_api(val=0.7),
        eve_add_absolute_low_attr_id: muta_abs_to_api(val=50),
        eve_add_absolute_mid_attr_id: muta_abs_to_api(val=50),
        eve_add_absolute_high_attr_id: muta_abs_to_api(val=50),
        eve_change_roll_attr_id: muta_roll_to_api(val=0.2),
        eve_change_absolute_low_attr_id: muta_abs_to_api(val=-50),
        eve_change_absolute_mid_attr_id: muta_abs_to_api(val=-50),
        eve_change_absolute_high_attr_id: muta_abs_to_api(val=-50),
        eve_remove_roll_attr_id: None,
        eve_remove_absolute_low_attr_id: None,
        eve_remove_absolute_mid_attr_id: None,
        eve_remove_absolute_high_attr_id: None})
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 12
    assert api_item.mutation.attrs[eve_add_roll_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_add_roll_attr_id].absolute == approx(54)
    assert api_item.mutation.attrs[eve_add_absolute_low_attr_id].roll is None
    assert api_item.mutation.attrs[eve_add_absolute_low_attr_id].absolute == approx(46)
    assert api_item.mutation.attrs[eve_add_absolute_mid_attr_id].roll is None
    assert api_item.mutation.attrs[eve_add_absolute_mid_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_add_absolute_high_attr_id].roll is None
    assert api_item.mutation.attrs[eve_add_absolute_high_attr_id].absolute == approx(54)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].roll == approx(0.2)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].absolute == approx(-46)
    assert api_item.mutation.attrs[eve_change_absolute_low_attr_id].roll is None
    assert api_item.mutation.attrs[eve_change_absolute_low_attr_id].absolute == approx(-46)
    assert api_item.mutation.attrs[eve_change_absolute_mid_attr_id].roll is None
    assert api_item.mutation.attrs[eve_change_absolute_mid_attr_id].absolute == approx(-50)
    assert api_item.mutation.attrs[eve_change_absolute_high_attr_id].roll is None
    assert api_item.mutation.attrs[eve_change_absolute_high_attr_id].absolute == approx(-54)
    assert api_item.mutation.attrs[eve_remove_roll_attr_id].roll is None
    assert api_item.mutation.attrs[eve_remove_roll_attr_id].absolute == approx(54)
    assert api_item.mutation.attrs[eve_remove_absolute_low_attr_id].roll is None
    assert api_item.mutation.attrs[eve_remove_absolute_low_attr_id].absolute == approx(46)
    assert api_item.mutation.attrs[eve_remove_absolute_mid_attr_id].roll is None
    assert api_item.mutation.attrs[eve_remove_absolute_mid_attr_id].absolute == approx(50)
    assert api_item.mutation.attrs[eve_remove_absolute_high_attr_id].roll is None
    assert api_item.mutation.attrs[eve_remove_absolute_high_attr_id].absolute == approx(54)
    assert api_item.attrs[eve_add_roll_attr_id].base == approx(54)
    assert api_item.attrs[eve_add_absolute_low_attr_id].base == approx(46)
    assert api_item.attrs[eve_add_absolute_mid_attr_id].base == approx(50)
    assert api_item.attrs[eve_add_absolute_high_attr_id].base == approx(54)
    assert api_item.attrs[eve_change_roll_attr_id].base == approx(-46)
    assert api_item.attrs[eve_change_absolute_low_attr_id].base == approx(-46)
    assert api_item.attrs[eve_change_absolute_mid_attr_id].base == approx(-50)
    assert api_item.attrs[eve_change_absolute_high_attr_id].base == approx(-54)
    assert api_item.attrs[eve_remove_roll_attr_id].base == approx(54)
    assert api_item.attrs[eve_remove_absolute_low_attr_id].base == approx(46)
    assert api_item.attrs[eve_remove_absolute_mid_attr_id].base == approx(50)
    assert api_item.attrs[eve_remove_absolute_high_attr_id].base == approx(54)
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


def test_zero_base_value(client):
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
    eve_mutator_id = client.mk_eve_mutator(
        datas=[eve_d1, eve_d2],
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
        eve_change_roll_attr_id: muta_roll_to_api(val=0.7),
        eve_change_absolute_low_attr_id: muta_abs_to_api(val=0),
        eve_change_absolute_mid_attr_id: muta_abs_to_api(val=0),
        eve_change_absolute_high_attr_id: muta_abs_to_api(val=0),
        eve_remove_roll_attr_id: muta_roll_to_api(val=0.4),
        eve_remove_absolute_low_attr_id: muta_abs_to_api(val=-3),
        eve_remove_absolute_mid_attr_id: muta_abs_to_api(val=0),
        eve_remove_absolute_high_attr_id: muta_abs_to_api(val=6)}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 12
    assert api_item.mutation.attrs[eve_add_roll_attr_id].roll is None
    assert api_item.mutation.attrs[eve_add_roll_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_add_absolute_low_attr_id].roll is None
    assert api_item.mutation.attrs[eve_add_absolute_low_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_add_absolute_mid_attr_id].roll is None
    assert api_item.mutation.attrs[eve_add_absolute_mid_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_add_absolute_high_attr_id].roll is None
    assert api_item.mutation.attrs[eve_add_absolute_high_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_change_absolute_low_attr_id].roll is None
    assert api_item.mutation.attrs[eve_change_absolute_low_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_change_absolute_mid_attr_id].roll is None
    assert api_item.mutation.attrs[eve_change_absolute_mid_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_change_absolute_high_attr_id].roll is None
    assert api_item.mutation.attrs[eve_change_absolute_high_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_remove_roll_attr_id].roll == approx(0.4)
    assert api_item.mutation.attrs[eve_remove_roll_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_remove_absolute_low_attr_id].roll is None
    assert api_item.mutation.attrs[eve_remove_absolute_low_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_remove_absolute_mid_attr_id].roll is None
    assert api_item.mutation.attrs[eve_remove_absolute_mid_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_remove_absolute_high_attr_id].roll is None
    assert api_item.mutation.attrs[eve_remove_absolute_high_attr_id].absolute == approx(0)
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
        eve_add_roll_attr_id: muta_roll_to_api(val=0.7),
        eve_add_absolute_low_attr_id: muta_abs_to_api(val=-8),
        eve_add_absolute_mid_attr_id: muta_abs_to_api(val=0),
        eve_add_absolute_high_attr_id: muta_abs_to_api(val=2),
        eve_change_roll_attr_id: muta_roll_to_api(val=0.2),
        eve_change_absolute_low_attr_id: muta_abs_to_api(val=-3),
        eve_change_absolute_mid_attr_id: muta_abs_to_api(val=0),
        eve_change_absolute_high_attr_id: muta_abs_to_api(val=6),
        eve_remove_roll_attr_id: None,
        eve_remove_absolute_low_attr_id: None,
        eve_remove_absolute_mid_attr_id: None,
        eve_remove_absolute_high_attr_id: None})
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 12
    assert api_item.mutation.attrs[eve_add_roll_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_add_roll_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_add_absolute_low_attr_id].roll is None
    assert api_item.mutation.attrs[eve_add_absolute_low_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_add_absolute_mid_attr_id].roll is None
    assert api_item.mutation.attrs[eve_add_absolute_mid_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_add_absolute_high_attr_id].roll is None
    assert api_item.mutation.attrs[eve_add_absolute_high_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].roll == approx(0.2)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_change_absolute_low_attr_id].roll is None
    assert api_item.mutation.attrs[eve_change_absolute_low_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_change_absolute_mid_attr_id].roll is None
    assert api_item.mutation.attrs[eve_change_absolute_mid_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_change_absolute_high_attr_id].roll is None
    assert api_item.mutation.attrs[eve_change_absolute_high_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_remove_roll_attr_id].roll is None
    assert api_item.mutation.attrs[eve_remove_roll_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_remove_absolute_low_attr_id].roll is None
    assert api_item.mutation.attrs[eve_remove_absolute_low_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_remove_absolute_mid_attr_id].roll is None
    assert api_item.mutation.attrs[eve_remove_absolute_mid_attr_id].absolute == approx(0)
    assert api_item.mutation.attrs[eve_remove_absolute_high_attr_id].roll is None
    assert api_item.mutation.attrs[eve_remove_absolute_high_attr_id].absolute == approx(0)
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


def test_base_out_of_range(client):
    eve_add_lower_attr_id = client.mk_eve_attr()
    eve_add_higher_attr_id = client.mk_eve_attr()
    eve_change_lower_attr_id = client.mk_eve_attr()
    eve_change_higher_attr_id = client.mk_eve_attr()
    eve_remove_lower_attr_id = client.mk_eve_attr()
    eve_remove_higher_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={
        eve_add_lower_attr_id: 100,
        eve_add_higher_attr_id: 100,
        eve_change_lower_attr_id: 100,
        eve_change_higher_attr_id: 100,
        eve_remove_lower_attr_id: 100,
        eve_remove_higher_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)], attrs={
        eve_add_lower_attr_id: (0.7, 0.9),
        eve_add_higher_attr_id: (1.1, 1.3),
        eve_change_lower_attr_id: (0.7, 0.9),
        eve_change_higher_attr_id: (1.1, 1.3),
        eve_remove_lower_attr_id: (0.7, 0.9),
        eve_remove_higher_attr_id: (1.1, 1.3)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_change_lower_attr_id: muta_roll_to_api(val=0.7),
        eve_change_higher_attr_id: muta_roll_to_api(val=0.3),
        eve_remove_lower_attr_id: muta_roll_to_api(val=0.7),
        eve_remove_higher_attr_id: muta_roll_to_api(val=0.3)}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 6
    assert api_item.mutation.attrs[eve_add_lower_attr_id].roll == approx(1)
    assert api_item.mutation.attrs[eve_add_lower_attr_id].absolute == approx(90)
    assert api_item.mutation.attrs[eve_add_higher_attr_id].roll == approx(0)
    assert api_item.mutation.attrs[eve_add_higher_attr_id].absolute == approx(110)
    assert api_item.mutation.attrs[eve_change_lower_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_change_lower_attr_id].absolute == approx(84)
    assert api_item.mutation.attrs[eve_change_higher_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_change_higher_attr_id].absolute == approx(116)
    assert api_item.mutation.attrs[eve_remove_lower_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_remove_lower_attr_id].absolute == approx(84)
    assert api_item.mutation.attrs[eve_remove_higher_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_remove_higher_attr_id].absolute == approx(116)
    assert api_item.attrs[eve_add_lower_attr_id].base == approx(90)
    assert api_item.attrs[eve_add_higher_attr_id].base == approx(110)
    assert api_item.attrs[eve_change_lower_attr_id].base == approx(84)
    assert api_item.attrs[eve_change_higher_attr_id].base == approx(116)
    assert api_item.attrs[eve_remove_lower_attr_id].base == approx(84)
    assert api_item.attrs[eve_remove_higher_attr_id].base == approx(116)
    # Action
    api_item.change_module(mutation={
        eve_add_lower_attr_id: muta_roll_to_api(val=0.7),
        eve_add_higher_attr_id: muta_roll_to_api(val=0.3),
        eve_change_lower_attr_id: muta_roll_to_api(val=0.8),
        eve_change_higher_attr_id: muta_roll_to_api(val=0.2),
        eve_remove_lower_attr_id: None,
        eve_remove_higher_attr_id: None})
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 6
    assert api_item.mutation.attrs[eve_add_lower_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_add_lower_attr_id].absolute == approx(84)
    assert api_item.mutation.attrs[eve_add_higher_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_add_higher_attr_id].absolute == approx(116)
    assert api_item.mutation.attrs[eve_change_lower_attr_id].roll == approx(0.8)
    assert api_item.mutation.attrs[eve_change_lower_attr_id].absolute == approx(86)
    assert api_item.mutation.attrs[eve_change_higher_attr_id].roll == approx(0.2)
    assert api_item.mutation.attrs[eve_change_higher_attr_id].absolute == approx(114)
    assert api_item.mutation.attrs[eve_remove_lower_attr_id].roll == approx(1)
    assert api_item.mutation.attrs[eve_remove_lower_attr_id].absolute == approx(90)
    assert api_item.mutation.attrs[eve_remove_higher_attr_id].roll == approx(0)
    assert api_item.mutation.attrs[eve_remove_higher_attr_id].absolute == approx(110)
    assert api_item.attrs[eve_add_lower_attr_id].base == approx(84)
    assert api_item.attrs[eve_add_higher_attr_id].base == approx(116)
    assert api_item.attrs[eve_change_lower_attr_id].base == approx(86)
    assert api_item.attrs[eve_change_higher_attr_id].base == approx(114)
    assert api_item.attrs[eve_remove_lower_attr_id].base == approx(90)
    assert api_item.attrs[eve_remove_higher_attr_id].base == approx(110)


def test_modification_incoming(client, consts):
    # Check that changing mutated value correctly triggers recalculation
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_add_attr_id = client.mk_eve_attr()
    eve_affectee_change_attr_id = client.mk_eve_attr()
    eve_affectee_remove_attr_id = client.mk_eve_attr()
    eve_add_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_add_attr_id)
    eve_change_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_change_attr_id)
    eve_remove_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_remove_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_add_mod, eve_change_mod, eve_remove_mod])
    eve_base_item_id = client.mk_eve_item(attrs={
        eve_affector_attr_id: 20,
        eve_affectee_add_attr_id: 200,
        eve_affectee_change_attr_id: 200,
        eve_affectee_remove_attr_id: 200})
    eve_mutated_item_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)], attrs={
        eve_affectee_add_attr_id: (0.5, 0.9),
        eve_affectee_change_attr_id: (0.8, 1.2),
        eve_affectee_remove_attr_id: (1.1, 1.5)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_affectee_change_attr_id: muta_roll_to_api(val=0.2),
        eve_affectee_remove_attr_id: muta_roll_to_api(val=0.8)}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 3
    assert api_item.mutation.attrs[eve_affectee_add_attr_id].roll == approx(1)
    assert api_item.mutation.attrs[eve_affectee_add_attr_id].absolute == approx(180)
    assert api_item.mutation.attrs[eve_affectee_change_attr_id].roll == approx(0.2)
    assert api_item.mutation.attrs[eve_affectee_change_attr_id].absolute == approx(176)
    assert api_item.mutation.attrs[eve_affectee_remove_attr_id].roll == approx(0.8)
    assert api_item.mutation.attrs[eve_affectee_remove_attr_id].absolute == approx(284)
    assert api_item.attrs[eve_affector_attr_id].base == approx(20)
    assert api_item.attrs[eve_affector_attr_id].dogma == approx(20)
    assert api_item.attrs[eve_affectee_add_attr_id].base == approx(180)
    assert api_item.attrs[eve_affectee_add_attr_id].dogma == approx(216)
    assert api_item.attrs[eve_affectee_change_attr_id].base == approx(176)
    assert api_item.attrs[eve_affectee_change_attr_id].dogma == approx(211.2)
    assert api_item.attrs[eve_affectee_remove_attr_id].base == approx(284)
    assert api_item.attrs[eve_affectee_remove_attr_id].dogma == approx(340.8)
    # Action
    api_item.change_module(mutation={
        eve_affectee_add_attr_id: muta_roll_to_api(val=0.9),
        eve_affectee_change_attr_id: muta_roll_to_api(val=0.3),
        eve_affectee_remove_attr_id: None})
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 3
    assert api_item.mutation.attrs[eve_affectee_add_attr_id].roll == approx(0.9)
    assert api_item.mutation.attrs[eve_affectee_add_attr_id].absolute == approx(172)
    assert api_item.mutation.attrs[eve_affectee_change_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_affectee_change_attr_id].absolute == approx(184)
    assert api_item.mutation.attrs[eve_affectee_remove_attr_id].roll == approx(0)
    assert api_item.mutation.attrs[eve_affectee_remove_attr_id].absolute == approx(220)
    assert api_item.attrs[eve_affector_attr_id].base == approx(20)
    assert api_item.attrs[eve_affector_attr_id].dogma == approx(20)
    assert api_item.attrs[eve_affectee_add_attr_id].base == approx(172)
    assert api_item.attrs[eve_affectee_add_attr_id].dogma == approx(206.4)
    assert api_item.attrs[eve_affectee_change_attr_id].base == approx(184)
    assert api_item.attrs[eve_affectee_change_attr_id].dogma == approx(220.8)
    assert api_item.attrs[eve_affectee_remove_attr_id].base == approx(220)
    assert api_item.attrs[eve_affectee_remove_attr_id].dogma == approx(264)


def test_modification_outgoing(client, consts):
    # Check that changing mutated value correctly triggers recalculation
    eve_affector_add_attr_id = client.mk_eve_attr()
    eve_affector_change_attr_id = client.mk_eve_attr()
    eve_affector_remove_attr_id = client.mk_eve_attr()
    eve_affectee_add_attr_id = client.mk_eve_attr()
    eve_affectee_change_attr_id = client.mk_eve_attr()
    eve_affectee_remove_attr_id = client.mk_eve_attr()
    eve_add_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_add_attr_id,
        affectee_attr_id=eve_affectee_add_attr_id)
    eve_change_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_change_attr_id,
        affectee_attr_id=eve_affectee_change_attr_id)
    eve_remove_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_remove_attr_id,
        affectee_attr_id=eve_affectee_remove_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_add_mod, eve_change_mod, eve_remove_mod])
    eve_base_item_id = client.mk_eve_item(attrs={
        eve_affector_add_attr_id: 20,
        eve_affector_change_attr_id: 20,
        eve_affector_remove_attr_id: 20,
        eve_affectee_add_attr_id: 200,
        eve_affectee_change_attr_id: 200,
        eve_affectee_remove_attr_id: 200})
    eve_mutated_item_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)], attrs={
        eve_affector_add_attr_id: (0.5, 0.9),
        eve_affector_change_attr_id: (0.8, 1.2),
        eve_affector_remove_attr_id: (1.1, 1.5)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_affector_change_attr_id: muta_roll_to_api(val=0.2),
        eve_affector_remove_attr_id: muta_roll_to_api(val=0.8)}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 3
    assert api_item.mutation.attrs[eve_affector_add_attr_id].roll == approx(1)
    assert api_item.mutation.attrs[eve_affector_add_attr_id].absolute == approx(18)
    assert api_item.mutation.attrs[eve_affector_change_attr_id].roll == approx(0.2)
    assert api_item.mutation.attrs[eve_affector_change_attr_id].absolute == approx(17.6)
    assert api_item.mutation.attrs[eve_affector_remove_attr_id].roll == approx(0.8)
    assert api_item.mutation.attrs[eve_affector_remove_attr_id].absolute == approx(28.4)
    assert api_item.attrs[eve_affector_add_attr_id].base == approx(18)
    assert api_item.attrs[eve_affector_add_attr_id].dogma == approx(18)
    assert api_item.attrs[eve_affector_change_attr_id].base == approx(17.6)
    assert api_item.attrs[eve_affector_change_attr_id].dogma == approx(17.6)
    assert api_item.attrs[eve_affector_remove_attr_id].base == approx(28.4)
    assert api_item.attrs[eve_affector_remove_attr_id].dogma == approx(28.4)
    assert api_item.attrs[eve_affectee_add_attr_id].base == approx(200)
    assert api_item.attrs[eve_affectee_add_attr_id].dogma == approx(236)
    assert api_item.attrs[eve_affectee_change_attr_id].base == approx(200)
    assert api_item.attrs[eve_affectee_change_attr_id].dogma == approx(235.2)
    assert api_item.attrs[eve_affectee_remove_attr_id].base == approx(200)
    assert api_item.attrs[eve_affectee_remove_attr_id].dogma == approx(256.8)
    # Action
    api_item.change_module(mutation={
        eve_affector_add_attr_id: muta_roll_to_api(val=0.9),
        eve_affector_change_attr_id: muta_roll_to_api(val=0.3),
        eve_affector_remove_attr_id: None})
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 3
    assert api_item.mutation.attrs[eve_affector_add_attr_id].roll == approx(0.9)
    assert api_item.mutation.attrs[eve_affector_add_attr_id].absolute == approx(17.2)
    assert api_item.mutation.attrs[eve_affector_change_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_affector_change_attr_id].absolute == approx(18.4)
    assert api_item.mutation.attrs[eve_affector_remove_attr_id].roll == approx(0)
    assert api_item.mutation.attrs[eve_affector_remove_attr_id].absolute == approx(22)
    assert api_item.attrs[eve_affector_add_attr_id].base == approx(17.2)
    assert api_item.attrs[eve_affector_add_attr_id].dogma == approx(17.2)
    assert api_item.attrs[eve_affector_change_attr_id].base == approx(18.4)
    assert api_item.attrs[eve_affector_change_attr_id].dogma == approx(18.4)
    assert api_item.attrs[eve_affector_remove_attr_id].base == approx(22)
    assert api_item.attrs[eve_affector_remove_attr_id].dogma == approx(22)
    assert api_item.attrs[eve_affectee_add_attr_id].base == approx(200)
    assert api_item.attrs[eve_affectee_add_attr_id].dogma == approx(234.4)
    assert api_item.attrs[eve_affectee_change_attr_id].base == approx(200)
    assert api_item.attrs[eve_affectee_change_attr_id].dogma == approx(236.8)
    assert api_item.attrs[eve_affectee_remove_attr_id].base == approx(200)
    assert api_item.attrs[eve_affectee_remove_attr_id].dogma == approx(244)


def test_drone(client):
    eve_add_roll_attr_id = client.mk_eve_attr()
    eve_add_absolute_attr_id = client.mk_eve_attr()
    eve_change_roll_attr_id = client.mk_eve_attr()
    eve_change_absolute_attr_id = client.mk_eve_attr()
    eve_remove_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={
        eve_add_roll_attr_id: 100,
        eve_add_absolute_attr_id: 100,
        eve_change_roll_attr_id: 100,
        eve_change_absolute_attr_id: 100,
        eve_remove_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)], attrs={
        eve_add_roll_attr_id: (0.8, 1.2),
        eve_add_absolute_attr_id: (0.8, 1.2),
        eve_change_roll_attr_id: (0.8, 1.2),
        eve_change_absolute_attr_id: (0.8, 1.2),
        eve_remove_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_drone(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_change_roll_attr_id: muta_roll_to_api(val=0.6),
        eve_change_absolute_attr_id: muta_abs_to_api(val=111),
        eve_remove_attr_id: muta_roll_to_api(val=0.8)}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 5
    assert api_item.mutation.attrs[eve_add_roll_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_roll_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_add_absolute_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_add_absolute_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].roll == approx(0.6)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].absolute == approx(104)
    assert api_item.mutation.attrs[eve_change_absolute_attr_id].roll == approx(0.775)
    assert api_item.mutation.attrs[eve_change_absolute_attr_id].absolute == approx(111)
    assert api_item.mutation.attrs[eve_remove_attr_id].roll == approx(0.8)
    assert api_item.mutation.attrs[eve_remove_attr_id].absolute == approx(112)
    assert api_item.attrs[eve_add_roll_attr_id].base == approx(100)
    assert api_item.attrs[eve_add_absolute_attr_id].base == approx(100)
    assert api_item.attrs[eve_change_roll_attr_id].base == approx(104)
    assert api_item.attrs[eve_change_absolute_attr_id].base == approx(111)
    assert api_item.attrs[eve_remove_attr_id].base == approx(112)
    # Action
    api_item.change_drone(mutation={
        eve_add_roll_attr_id: muta_roll_to_api(val=0.3),
        eve_add_absolute_attr_id: muta_abs_to_api(val=105),
        eve_change_roll_attr_id: muta_roll_to_api(val=0.55),
        eve_change_absolute_attr_id: muta_abs_to_api(val=109),
        eve_remove_attr_id: None})
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 5
    assert api_item.mutation.attrs[eve_add_roll_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_add_roll_attr_id].absolute == approx(92)
    assert api_item.mutation.attrs[eve_add_absolute_attr_id].roll == approx(0.625)
    assert api_item.mutation.attrs[eve_add_absolute_attr_id].absolute == approx(105)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].roll == approx(0.55)
    assert api_item.mutation.attrs[eve_change_roll_attr_id].absolute == approx(102)
    assert api_item.mutation.attrs[eve_change_absolute_attr_id].roll == approx(0.725)
    assert api_item.mutation.attrs[eve_change_absolute_attr_id].absolute == approx(109)
    assert api_item.mutation.attrs[eve_remove_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_remove_attr_id].absolute == approx(100)
    assert api_item.attrs[eve_add_roll_attr_id].base == approx(92)
    assert api_item.attrs[eve_add_absolute_attr_id].base == approx(105)
    assert api_item.attrs[eve_change_roll_attr_id].base == approx(102)
    assert api_item.attrs[eve_change_absolute_attr_id].base == approx(109)
    assert api_item.attrs[eve_remove_attr_id].base == approx(100)
