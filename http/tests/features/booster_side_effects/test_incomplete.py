from tests import approx, check_no_field


def test_unloaded_item(client):
    eve_chance_attr_id = client.mk_eve_attr()
    eve_booster_id = client.alloc_item_id()
    eve_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id)
    # Create item with the effect just to make sure it is not getting removed during cleanup
    client.mk_eve_item(eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    api_booster.update()
    with check_no_field():
        api_booster.side_effects  # pylint: disable=W0104
    # Action
    api_booster.change_booster(side_effects={eve_effect_id: True}, status_code=409)
    # Verification
    api_booster.update()
    with check_no_field():
        api_booster.side_effects  # pylint: disable=W0104


def test_no_chance_val(client, consts):
    eve_chance_attr_id = client.alloc_attr_id()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod])
    eve_booster_id = client.mk_eve_item(attrs={eve_affector_attr_id: 25}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    api_booster.update()
    with check_no_field():
        api_booster.side_effects  # pylint: disable=W0104
    # Action
    api_booster.change_booster(side_effects={eve_effect_id: True})
    # Verification
    api_booster.update()
    with check_no_field():
        api_booster.side_effects  # pylint: disable=W0104


def test_no_str_val(client, consts):
    eve_chance_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.alloc_attr_id()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod])
    eve_booster_id = client.mk_eve_item(attrs={eve_chance_attr_id: 0.4}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str is None
    # Action
    api_booster.change_booster(side_effects={eve_effect_id: True})
    # Verification
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str is None
