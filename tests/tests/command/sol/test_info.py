from fw import check_no_field


def test_sol_sol(client, consts):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.batch() as api_sol_batch:
        api_sol_info1 = api_sol_batch.get_sol_info(sol_mode=consts.ApiSolInfoMode.full)
        api_sol_info2 = api_sol_batch.get_sol_info(sol_mode=consts.ApiSolInfoMode.id)
    # Verification
    assert api_fit.id in api_sol_info1.fits
    with check_no_field():
        api_sol_info2.fits  # ruff:ignore[useless-expression]


def test_sol_fleet_override(client, consts):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fleet1 = api_sol.create_fleet(fit_ids=[api_fit1.id])
    api_fleet2 = api_sol.create_fleet(fit_ids=[api_fit2.id])
    with api_sol.batch() as api_sol_batch:
        api_sol_info = api_sol_batch.get_sol_info(
            sol_mode=consts.ApiSolInfoMode.full,
            fleet_mode=(consts.ApiFleetInfoMode.id, [(consts.ApiFleetInfoMode.full, [api_fleet1.id])]))
    # Verification
    assert api_fit1.id in api_sol_info.fleets[api_fleet1.id].fits
    api_fleet2_info = api_sol_info.fleets[api_fleet2.id]
    with check_no_field():
        api_fleet2_info.fits  # ruff:ignore[useless-expression]


def test_sol_fleet_override_backref(client, consts):
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_fit1 = api_sol_batch.create_fit()
        api_fit2 = api_sol_batch.create_fit()
        api_fleet1 = api_sol_batch.create_fleet(fit_ids=[api_fit1.id])
        api_fleet2 = api_sol_batch.create_fleet(fit_ids=[api_fit2.id])
        api_sol_info = api_sol_batch.get_sol_info(
            sol_mode=consts.ApiSolInfoMode.full,
            fleet_mode=(consts.ApiFleetInfoMode.id, [(consts.ApiFleetInfoMode.full, ['#0', '#9', api_fleet1.id])]))
    # Verification
    assert api_fit1.id in api_sol_info.fleets[api_fleet1.id].fits
    api_fleet2_info = api_sol_info.fleets[api_fleet2.id]
    with check_no_field():
        api_fleet2_info.fits  # ruff:ignore[useless-expression]


def test_sol_fit_override(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_item1 = api_fit1.add_implant(type_id=eve_item_id)
    api_fit2.add_implant(type_id=eve_item_id)
    with api_sol.batch() as api_sol_batch:
        api_sol_info = api_sol_batch.get_sol_info(
            sol_mode=consts.ApiSolInfoMode.full,
            fit_mode=(consts.ApiFitInfoMode.id, [(consts.ApiFitInfoMode.full, [api_fit1.id])]))
    # Verification
    assert api_item1.id in api_sol_info.fits[api_fit1.id].implants
    api_fit2_info = api_sol_info.fits[api_fit2.id]
    with check_no_field():
        api_fit2_info.implants  # ruff:ignore[useless-expression]


def test_sol_fit_override_backref(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_fit1 = api_sol_batch.create_fit()
        api_fit2 = api_sol_batch.create_fit()
        api_item1 = api_sol_batch.add_implant(fit_id=api_fit1.id, type_id=eve_item_id)
        api_sol_batch.add_implant(fit_id=api_fit2.id, type_id=eve_item_id)
        api_sol_info = api_sol_batch.get_sol_info(
            sol_mode=consts.ApiSolInfoMode.full,
            fit_mode=(consts.ApiFitInfoMode.id, [(consts.ApiFitInfoMode.full, ['#2', '#9', api_fit1.id])]))
    # Verification
    assert api_item1.id in api_sol_info.fits[api_fit1.id].implants
    api_fit2_info = api_sol_info.fits[api_fit2.id]
    with check_no_field():
        api_fit2_info.implants  # ruff:ignore[useless-expression]


def test_sol_item_override(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item1 = api_fit.add_implant(type_id=eve_item_id)
    api_item2 = api_fit.add_implant(type_id=eve_item_id)
    with api_sol.batch() as api_sol_batch:
        api_sol_info = api_sol_batch.get_sol_info(
            sol_mode=consts.ApiSolInfoMode.full,
            fit_mode=consts.ApiFitInfoMode.full,
            item_mode=(consts.ApiItemInfoMode.id, [(consts.ApiItemInfoMode.partial, [api_item2.id])]))
    # Verification
    api_fit_info = api_sol_info.fits[api_fit.id]
    api_item1_info = api_fit_info.implants[api_item1.id]
    with check_no_field():
        api_item1_info.type_id  # ruff:ignore[useless-expression]
    api_item2_info = api_fit_info.implants[api_item2.id]
    assert api_item2_info.type_id == eve_item_id


def test_sol_item_override_backref(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_fit = api_sol_batch.create_fit()
        api_item1 = api_sol_batch.add_implant(fit_id=api_fit.id, type_id=eve_item_id)
        api_item2 = api_sol_batch.add_implant(fit_id=api_fit.id, type_id=eve_item_id)
        api_sol_info = api_sol_batch.get_sol_info(
            sol_mode=consts.ApiSolInfoMode.full,
            fit_mode=consts.ApiFitInfoMode.full,
            item_mode=(consts.ApiItemInfoMode.id, [(consts.ApiItemInfoMode.partial, ['#0', '#9', api_item2.id])]))
    # Verification
    api_fit_info = api_sol_info.fits[api_fit.id]
    api_item1_info = api_fit_info.implants[api_item1.id]
    with check_no_field():
        api_item1_info.type_id  # ruff:ignore[useless-expression]
    api_item2_info = api_fit_info.implants[api_item2.id]
    assert api_item2_info.type_id == eve_item_id


def test_fleet_fleet(client, consts):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    with api_sol.batch() as api_sol_batch:
        api_fleet_info1 = api_sol_batch.get_fleet_info(fleet_id=api_fleet.id, fleet_mode=consts.ApiFleetInfoMode.full)
        api_fleet_info2 = api_sol_batch.get_fleet_info(fleet_id=api_fleet.id, fleet_mode=consts.ApiFleetInfoMode.id)
    # Verification
    assert api_fit.id in api_fleet_info1.fits
    with check_no_field():
        api_fleet_info2.fits  # ruff:ignore[useless-expression]


def test_fleet_fleet_backref(client, consts):
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_fit = api_sol_batch.create_fit()
        api_fleet = api_sol_batch.create_fleet(fit_ids=[api_fit.id])
        api_fleet_info1 = api_sol_batch.get_fleet_info(fleet_id=api_fleet.id, fleet_mode=consts.ApiFleetInfoMode.full)
        api_fleet_info2 = api_sol_batch.get_fleet_info(fleet_id=api_fleet.id, fleet_mode=consts.ApiFleetInfoMode.id)
    # Verification
    assert api_fit.id in api_fleet_info1.fits
    with check_no_field():
        api_fleet_info2.fits  # ruff:ignore[useless-expression]


def test_fleet_fleet_backref_error_range(client, consts):
    client.create_sources()
    api_sol = client.create_sol()
    # Verification
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #2 does not have results recorded',
            'cmd_index': 1,
    }) as api_sol_batch:
        api_sol_batch.create_fleet()
        api_sol_batch.get_fleet_info(fleet_id='#2', fleet_mode=consts.ApiFleetInfoMode.full)


def test_fleet_fleet_backref_error_kind(client, consts):
    client.create_sources()
    api_sol = client.create_sol()
    # Verification
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #0 exists, but does not have fleet ID info',
            'cmd_index': 1,
    }) as api_sol_batch:
        api_sol_batch.create_fit()
        api_sol_batch.get_fleet_info(fleet_id='#0', fleet_mode=consts.ApiFleetInfoMode.full)


def test_fit_fit(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item_id)
    with api_sol.batch() as api_sol_batch:
        api_fit_info1 = api_sol_batch.get_fit_info(fit_id=api_fit.id, fit_mode=consts.ApiFitInfoMode.full)
        api_fit_info2 = api_sol_batch.get_fit_info(fit_id=api_fit.id, fit_mode=consts.ApiFitInfoMode.id)
    # Verification
    assert api_item.id in api_fit_info1.implants
    with check_no_field():
        api_fit_info2.implants  # ruff:ignore[useless-expression]


def test_fit_fit_backref(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_fit = api_sol_batch.create_fit()
        api_item = api_sol_batch.add_implant(fit_id=api_fit.id, type_id=eve_item_id)
        api_fit_info1 = api_sol_batch.get_fit_info(fit_id=api_fit.id, fit_mode=consts.ApiFitInfoMode.full)
        api_fit_info2 = api_sol_batch.get_fit_info(fit_id=api_fit.id, fit_mode=consts.ApiFitInfoMode.id)
    # Verification
    assert api_item.id in api_fit_info1.implants
    with check_no_field():
        api_fit_info2.implants  # ruff:ignore[useless-expression]


def test_fit_fit_backref_error_range(client, consts):
    client.create_sources()
    api_sol = client.create_sol()
    # Verification
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #2 does not have results recorded',
            'cmd_index': 1,
    }) as api_sol_batch:
        api_sol_batch.create_fit()
        api_sol_batch.get_fit_info(fit_id='#2', fit_mode=consts.ApiFitInfoMode.full)


def test_fit_fit_backref_error_kind(client, consts):
    client.create_sources()
    api_sol = client.create_sol()
    # Verification
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #0 exists, but does not have fit ID info',
            'cmd_index': 1,
    }) as api_sol_batch:
        api_sol_batch.create_fleet()
        api_sol_batch.get_fit_info(fit_id='#0', fit_mode=consts.ApiFitInfoMode.full)


def test_fit_item_override(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item1 = api_fit.add_implant(type_id=eve_item_id)
    api_item2 = api_fit.add_implant(type_id=eve_item_id)
    with api_sol.batch() as api_sol_batch:
        api_fit_info = api_sol_batch.get_fit_info(
            fit_id=api_fit.id,
            fit_mode=consts.ApiFitInfoMode.full,
            item_mode=(consts.ApiItemInfoMode.id, [(consts.ApiItemInfoMode.partial, [api_item2.id])]))
    # Verification
    api_item1_info = api_fit_info.implants[api_item1.id]
    with check_no_field():
        api_item1_info.type_id  # ruff:ignore[useless-expression]
    api_item2_info = api_fit_info.implants[api_item2.id]
    assert api_item2_info.type_id == eve_item_id


def test_fit_item_override_backref(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_fit = api_sol_batch.create_fit()
        api_item1 = api_sol_batch.add_implant(fit_id=api_fit.id, type_id=eve_item_id)
        api_item2 = api_sol_batch.add_implant(fit_id=api_fit.id, type_id=eve_item_id)
        api_fit_info = api_sol_batch.get_fit_info(
            fit_id=api_fit.id,
            fit_mode=consts.ApiFitInfoMode.full,
            item_mode=(consts.ApiItemInfoMode.id, [(consts.ApiItemInfoMode.partial, ['#0', '#9', api_item2.id])]))
    # Verification
    api_item1_info = api_fit_info.implants[api_item1.id]
    with check_no_field():
        api_item1_info.type_id  # ruff:ignore[useless-expression]
    api_item2_info = api_fit_info.implants[api_item2.id]
    assert api_item2_info.type_id == eve_item_id


def test_item_item(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item_id)
    with api_sol.batch() as api_sol_batch:
        api_item_info1 = api_sol_batch.get_item_info(item_id=api_item.id, item_mode=consts.ApiItemInfoMode.partial)
        api_item_info2 = api_sol_batch.get_item_info(item_id=api_item.id, item_mode=consts.ApiItemInfoMode.id)
    # Verification
    assert api_item_info1.type_id == eve_item_id
    with check_no_field():
        api_item_info2.type_id  # ruff:ignore[useless-expression]


def test_item_item_backref(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_fit = api_sol_batch.create_fit()
        api_item = api_sol_batch.add_implant(fit_id=api_fit.id, type_id=eve_item_id)
        api_item_info1 = api_sol_batch.get_item_info(item_id=api_item.id, item_mode=consts.ApiItemInfoMode.partial)
        api_item_info2 = api_sol_batch.get_item_info(item_id=api_item.id, item_mode=consts.ApiItemInfoMode.id)
    # Verification
    assert api_item_info1.type_id == eve_item_id
    with check_no_field():
        api_item_info2.type_id  # ruff:ignore[useless-expression]


def test_item_item_backref_error_range(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #2 does not have results recorded',
            'cmd_index': 1,
    }) as api_sol_batch:
        api_sol_batch.add_implant(fit_id=api_fit.id, type_id=eve_item_id)
        api_sol_batch.get_item_info(item_id='#2', item_mode=consts.ApiItemInfoMode.partial)


def test_item_item_backref_error_kind(client, consts):
    client.create_sources()
    api_sol = client.create_sol()
    # Verification
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #0 exists, but does not have item ID info',
            'cmd_index': 1,
    }) as api_sol_batch:
        api_sol_batch.create_fit()
        api_sol_batch.get_item_info(item_id='#0', item_mode=consts.ApiItemInfoMode.partial)
