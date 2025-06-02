from tests import approx, check_no_field


def test_from_stage1(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item1_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_base_item2_id = client.mk_eve_item(attrs={eve_attr_id: 200})
    eve_mutator_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(
        type_id=eve_base_item1_id,
        mutation=(eve_mutator_id, {eve_attr_id: {consts.ApiAttrMutation.roll: 0.3}}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item1_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_attr_id].base == approx(100)
    # Action
    api_item.change_module(type_id=eve_base_item2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item2_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_attr_id].base == approx(200)
    # Action
    api_item.change_module(type_id=eve_base_item1_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item1_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_attr_id].base == approx(100)
