from fw import check_no_field


def test_error_params_malformed(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Action & verification
    api_fleet.change(
        rm_fit_ids=[api_fit.id],
        fleet_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    assert api_fit.id in api_fleet.update().fits


def test_add_fit_member(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_fit = api_sol.create_fit(fleet_id=api_fleet.id)
    # Action
    api_fleet.change(add_fit_ids=[api_fit.id])
    # Verification
    assert api_fleet.update().fits.keys() == {api_fit.id}
    assert api_fit.update().fleet_id == api_fleet.id


def test_add_fit_duplicate(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_fit = api_sol.create_fit()
    # Action
    api_fleet.change(add_fit_ids=[api_fit.id, api_fit.id])
    # Verification
    assert api_fleet.update().fits.keys() == {api_fit.id}
    assert api_fit.update().fleet_id == api_fleet.id


def test_add_fit_other_fleet(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet1 = api_sol.create_fleet()
    api_fleet2 = api_sol.create_fleet()
    api_fit = api_sol.create_fit(fleet_id=api_fleet2.id)
    # Action
    api_fleet1.change(add_fit_ids=[api_fit.id])
    # Verification - fit leaves its fleet
    assert api_fleet1.update().fits.keys() == {api_fit.id}
    api_fleet2.update()
    with check_no_field():
        api_fleet2.fits  # ruff:ignore[useless-expression]
    assert api_fit.update().fleet_id == api_fleet1.id


def test_rm_fit_no_fleet(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit1.id])
    # Action
    api_fleet.change(rm_fit_ids=[api_fit2.id])
    # Verification
    assert api_fleet.update().fits.keys() == {api_fit1.id}
    api_fit2.update()
    with check_no_field():
        api_fit2.fleet_id  # ruff:ignore[useless-expression]


def test_rm_fit_other_fleet(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fleet1 = api_sol.create_fleet(fit_ids=[api_fit1.id])
    api_fleet2 = api_sol.create_fleet(fit_ids=[api_fit2.id])
    # Action
    api_fleet1.change(rm_fit_ids=[api_fit2.id])
    # Verification - fit stays in its fleet
    assert api_fleet1.update().fits.keys() == {api_fit1.id}
    assert api_fleet2.update().fits.keys() == {api_fit2.id}
    assert api_fit2.update().fleet_id == api_fleet2.id
