from tests import approx


def setup_penalization_test(*, client, consts, stackable):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(stackable=stackable)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_affector1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 10}, eff_ids=[eve_effect_id])
    eve_item_affector2_id = client.mk_eve_item(attrs={eve_affector_attr_id: -20}, eff_ids=[eve_effect_id])
    eve_item_affector3_id = client.mk_eve_item(attrs={eve_affector_attr_id: 53.02}, eff_ids=[eve_effect_id])
    # Addition of 0 is considered insignificant, and won't be exposed as modification
    eve_item_affector4_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0}, eff_ids=[eve_effect_id])
    eve_item_affectee_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item_affector1 = api_fit.add_rig(type_id=eve_item_affector1_id)
    api_item_affector2 = api_fit.add_rig(type_id=eve_item_affector2_id)
    api_item_affector3 = api_fit.add_rig(type_id=eve_item_affector3_id)
    api_fit.add_rig(type_id=eve_item_affector4_id)
    api_item_affectee = api_fit.set_ship(type_id=eve_item_affectee_id)
    api_item_affectee.update()
    return (
        api_item_affectee.attrs[eve_affectee_attr_id].dogma,
        api_item_affectee.mods[eve_affectee_attr_id],
        api_item_affector1,
        api_item_affector2,
        api_item_affector3)


def test_non_penalized(client, consts):
    attr_val, attr_mods, api_item_affector1, api_item_affector2, api_item_affector3 = setup_penalization_test(
        client=client, consts=consts, stackable=True)
    # Verification
    assert attr_val == approx(143.02)
    assert len(attr_mods) == 3
    api_mod1 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one()
    assert api_mod1.op == consts.ApiModOp.mod_add
    assert api_mod1.initial_val == approx(10)
    assert api_mod1.stacking_mult is None
    assert api_mod1.applied_val == approx(10)
    api_mod2 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one()
    assert api_mod2.op == consts.ApiModOp.mod_add
    assert api_mod2.initial_val == approx(-20)
    assert api_mod2.stacking_mult is None
    assert api_mod2.initial_val == approx(-20)
    api_mod3 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector3.id).one()
    assert api_mod3.op == consts.ApiModOp.mod_add
    assert api_mod3.initial_val == approx(53.02)
    assert api_mod3.stacking_mult is None
    assert api_mod3.initial_val == approx(53.02)


def test_penalized(client, consts):
    attr_val, attr_mods, api_item_affector1, api_item_affector2, api_item_affector3 = setup_penalization_test(
        client=client, consts=consts, stackable=False)
    # Verification
    assert attr_val == approx(143.02)
    assert len(attr_mods) == 3
    api_mod1 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one()
    assert api_mod1.op == consts.ApiModOp.mod_add
    assert api_mod1.initial_val == approx(10)
    assert api_mod1.stacking_mult is None
    assert api_mod1.applied_val == approx(10)
    api_mod2 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one()
    assert api_mod2.op == consts.ApiModOp.mod_add
    assert api_mod2.initial_val == approx(-20)
    assert api_mod2.stacking_mult is None
    assert api_mod2.initial_val == approx(-20)
    api_mod3 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector3.id).one()
    assert api_mod3.op == consts.ApiModOp.mod_add
    assert api_mod3.initial_val == approx(53.02)
    assert api_mod3.stacking_mult is None
    assert api_mod3.initial_val == approx(53.02)
