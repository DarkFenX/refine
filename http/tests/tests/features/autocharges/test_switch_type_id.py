from fw import Effect, approx, check_no_field


def test_specified_same(client, consts):
    # Autocharges which have the same type ID
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_autocharge_id = client.mk_eve_item(attrs={eve_attr_id: 50})
    eve_fighter1_id = client.mk_eve_fighter(
        attrs={eve_autocharge_attr_id: eve_autocharge_id},
        eff_ids=[eve_effect_id])
    eve_fighter2_id = client.mk_eve_fighter(
        attrs={eve_autocharge_attr_id: eve_autocharge_id},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter1_id)
    # Verification
    api_fighter.update()
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[api_effect_id]
    assert api_autocharge.type_id == eve_autocharge_id
    assert api_autocharge.attrs[eve_attr_id].modified == approx(50)
    # Action
    api_fighter.change_fighter(type_id=eve_fighter2_id)
    # Verification
    api_fighter.update()
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[api_effect_id]
    assert api_autocharge.type_id == eve_autocharge_id
    assert api_autocharge.attrs[eve_attr_id].modified == approx(50)


def test_specified_different(client, consts):
    # Autocharges which have different type IDs
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_autocharge1_id = client.mk_eve_item(attrs={eve_attr1_id: 50})
    eve_autocharge2_id = client.mk_eve_item(attrs={eve_attr2_id: 70})
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_autocharge_attr_id: eve_autocharge1_id}, eff_ids=[eve_effect_id])
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_autocharge_attr_id: eve_autocharge2_id}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter1_id)
    # Verification
    api_fighter.update()
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[api_effect_id]
    assert api_autocharge.type_id == eve_autocharge1_id
    assert api_autocharge.attrs[eve_attr1_id].modified == approx(50)
    # Action
    api_fighter.change_fighter(type_id=eve_fighter2_id)
    # Verification
    api_fighter.update()
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[api_effect_id]
    assert api_autocharge.type_id == eve_autocharge2_id
    assert api_autocharge.attrs[eve_attr2_id].modified == approx(70)


def test_valid_to_invalid_reference_to_valid(client, consts):
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_autocharge1_id = client.mk_eve_item(attrs={eve_attr_id: 50})
    eve_autocharge2_id = client.alloc_item_id()
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_autocharge_attr_id: eve_autocharge1_id}, eff_ids=[eve_effect_id])
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_autocharge_attr_id: eve_autocharge2_id}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter1_id)
    # Verification
    api_fighter.update()
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[api_effect_id]
    assert api_autocharge.type_id == eve_autocharge1_id
    assert api_autocharge.attrs[eve_attr_id].modified == approx(50)
    # Action
    api_fighter.change_fighter(type_id=eve_fighter2_id)
    # Verification
    api_fighter.update()
    with check_no_field():
        api_fighter.autocharges  # noqa: B018
    # Action
    api_fighter.change_fighter(type_id=eve_fighter1_id)
    # Verification
    api_fighter.update()
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[api_effect_id]
    assert api_autocharge.type_id == eve_autocharge1_id
    assert api_autocharge.attrs[eve_attr_id].modified == approx(50)


def test_valid_to_no_reference_to_valid(client, consts):
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_autocharge1_id = client.mk_eve_item(attrs={eve_attr_id: 50})
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_autocharge_attr_id: eve_autocharge1_id}, eff_ids=[eve_effect_id])
    eve_fighter2_id = client.mk_eve_fighter(eff_ids=[eve_effect_id])
    client.create_sources()
    api_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter1_id)
    # Verification
    api_fighter.update()
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[api_effect_id]
    assert api_autocharge.type_id == eve_autocharge1_id
    assert api_autocharge.attrs[eve_attr_id].modified == approx(50)
    # Action
    api_fighter.change_fighter(type_id=eve_fighter2_id)
    # Verification
    api_fighter.update()
    with check_no_field():
        api_fighter.autocharges  # noqa: B018
    # Action
    api_fighter.change_fighter(type_id=eve_fighter1_id)
    # Verification
    api_fighter.update()
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[api_effect_id]
    assert api_autocharge.type_id == eve_autocharge1_id
    assert api_autocharge.attrs[eve_attr_id].modified == approx(50)
