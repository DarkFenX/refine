from pytest import approx


def test_affector_addition_removal(client, consts):
    # Check that effects are applied/removed when fit-wide effect is added/removed
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_attr1.id,
        affectee_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_fw_effect = client.mk_eve_item(attrs={eve_attr1.id: 5}, eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_ship()
    eve_rig = client.mk_eve_item(attrs={eve_attr2.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(7.5)
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect.id)
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(37.5)
    api_fw_effect.remove()
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(7.5)


def test_affector_state_change(client, consts):
    # Check that effects are applied/removed when fit-wide effect is enabled/disabled
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_attr1.id,
        affectee_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_fw_effect = client.mk_eve_item(attrs={eve_attr1.id: 5}, eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_ship()
    eve_rig = client.mk_eve_item(attrs={eve_attr2.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect.id, state=False)
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(7.5)
    api_fw_effect.change_fw_effect(state=True)
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(37.5)
    api_fw_effect.change_fw_effect(state=False)
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(7.5)
