from tests import Effect, approx, check_no_field


def test_autocharge(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_charge_id = client.mk_eve_item(attrs={eve_attr_id: 10})
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_autocharge_attr_id: eve_charge_id}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Check default upon addition
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[api_effect_id]
    assert isinstance(api_autocharge.id, str)
    with check_no_field():
        api_autocharge.kind  # noqa: B018
    with check_no_field():
        api_autocharge.attrs  # noqa: B018
    api_autocharge_id = api_autocharge.id
    # ID only
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.id)
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[api_effect_id]
    assert api_autocharge.id == api_autocharge_id
    with check_no_field():
        api_autocharge.kind  # noqa: B018
    with check_no_field():
        api_autocharge.attrs  # noqa: B018
    api_autocharge.update(item_info_mode=consts.ApiItemInfoMode.id)
    assert api_autocharge.id == api_autocharge_id
    with check_no_field():
        api_autocharge.kind  # noqa: B018
    with check_no_field():
        api_autocharge.attrs  # noqa: B018
    # Partial
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.partial)
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[api_effect_id]
    assert api_autocharge.id == api_autocharge_id
    assert api_autocharge.kind == consts.ApiItemKind.autocharge
    with check_no_field():
        api_autocharge.attrs  # noqa: B018
    api_autocharge.update(item_info_mode=consts.ApiItemInfoMode.partial)
    assert api_autocharge.id == api_autocharge_id
    assert api_autocharge.kind == consts.ApiItemKind.autocharge
    with check_no_field():
        api_autocharge.attrs  # noqa: B018
    # Full
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.full)
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[api_effect_id]
    assert api_autocharge.id == api_autocharge_id
    assert api_autocharge.kind == consts.ApiItemKind.autocharge
    assert api_autocharge.attrs[eve_attr_id].dogma == approx(10)
    api_autocharge.update(item_info_mode=consts.ApiItemInfoMode.full)
    assert api_autocharge.id == api_autocharge_id
    assert api_autocharge.kind == consts.ApiItemKind.autocharge
    assert api_autocharge.attrs[eve_attr_id].dogma == approx(10)


def test_invalid_reference(client, consts):
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_charge_id = client.alloc_item_id()
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_autocharge_attr_id: eve_charge_id}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Check default upon addition
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    with check_no_field():
        api_fighter.autocharges  # noqa: B018
    # ID only
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.id)
    with check_no_field():
        api_fighter.autocharges  # noqa: B018
    # Partial
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.partial)
    with check_no_field():
        api_fighter.autocharges  # noqa: B018
    # Full
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.full)
    with check_no_field():
        api_fighter.autocharges  # noqa: B018


def test_no_reference(client, consts):
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_fighter_id = client.mk_eve_fighter(eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Check default upon addition
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    with check_no_field():
        api_fighter.autocharges  # noqa: B018
    # ID only
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.id)
    with check_no_field():
        api_fighter.autocharges  # noqa: B018
    # Partial
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.partial)
    with check_no_field():
        api_fighter.autocharges  # noqa: B018
    # Full
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.full)
    with check_no_field():
        api_fighter.autocharges  # noqa: B018
