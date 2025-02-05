from tests import check_no_field


def test_character(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_char(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.not_loaded_item])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_char = api_fit.set_char(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.not_loaded_item])
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_char.id]
