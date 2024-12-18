"""
Stages is just a short description of how much data was available for the mutation:
- stage 1: mutator is not available;
- stage 2: mutator is available, but mutated item ID cannot be found for the base item ID;
- stage 3: mutator and mutated item ID are available, but mutated item is not available;
- stage 4: all the data was available.
"""

from tests import approx, check_no_field


def test_from_unmutated(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attributes={eve_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    # Action
    api_item.change_mod(mutation=None)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    # Action
    api_item.change_mod(mutation=eve_mutator_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(100)
    assert api_item.attrs[eve_attr_id].base == approx(100)


def test_from_stage1(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutator_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(
        type_id=eve_base_item_id,
        mutation=(eve_mutator_id, {eve_attr_id: {consts.ApiAttrMutation.roll: 0.6}}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    # Action
    api_item.change_mod(mutation=None)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    # Action
    api_item.change_mod(mutation=eve_mutator_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104


def test_from_stage2(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([client.mk_eve_item()], eve_mutated_item_id)],
        attributes={eve_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(
        type_id=eve_base_item_id,
        mutation=(eve_mutator_id, {eve_attr_id: {consts.ApiAttrMutation.roll: 0.6}}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    # Action
    api_item.change_mod(mutation=None)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    # Action
    api_item.change_mod(mutation=eve_mutator_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104


def test_from_stage3(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.alloc_item_id()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attributes={eve_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(
        type_id=eve_base_item_id,
        mutation=(eve_mutator_id, {eve_attr_id: {consts.ApiAttrMutation.roll: 0.6}}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    # Action
    api_item.change_mod(mutation=None)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    # Action
    api_item.change_mod(mutation=eve_mutator_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104


def test_from_stage4(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attributes={eve_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(
        type_id=eve_base_item_id,
        mutation=(eve_mutator_id, {eve_attr_id: {consts.ApiAttrMutation.roll: 0.6}}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.6)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(104)
    assert api_item.attrs[eve_attr_id].base == approx(104)
    # Action
    api_item.change_mod(mutation=None)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # pylint: disable=W0104
    # Action
    api_item.change_mod(mutation=eve_mutator_id)
    # Verification - after mutating item again, all the old mutations should be gone
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(100)
    assert api_item.attrs[eve_attr_id].base == approx(100)
