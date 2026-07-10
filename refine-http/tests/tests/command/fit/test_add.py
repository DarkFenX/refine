from fw import check_no_field


def test_error_params_malformed(client):
    client.create_sources()
    api_sol = client.create_sol()
    # Verification
    api_sol.create_fit(
        fit_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    api_sol.create_fit(
        item_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    api_sol.update()
    with check_no_field():
        api_sol.fits  # noqa: B018
