

def test_error_params_malformed(client):
    eve_sw_effect_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id, state=True)
    # Verification
    api_sw_effect.change_sw_effect(
        state=False,
        item_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    assert api_sw_effect.update().state is True
