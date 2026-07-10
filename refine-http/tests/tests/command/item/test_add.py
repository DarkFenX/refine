from fw import check_no_field


def test_error_params_malformed(client):
    eve_sw_effect_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    # Verification
    api_sol.add_sw_effect(
        type_id=eve_sw_effect_id,
        item_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    api_sol.update()
    with check_no_field():
        api_sol.sw_effects  # noqa: B018
