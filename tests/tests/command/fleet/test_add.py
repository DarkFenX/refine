from fw import check_no_field


def test_error_params_malformed(client):
    client.create_sources()
    api_sol = client.create_sol()
    # Action & verification
    api_sol.create_fleet(
        fleet_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    api_sol.update()
    with check_no_field():
        api_sol.fleets  # ruff:ignore[useless-expression]


def test_fit_duplicate(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id, api_fit.id])
    # Verification
    assert api_fleet.update().fits.keys() == {api_fit.id}
    assert api_fit.update().fleet_id == api_fleet.id
