from tests import approx


def test_debuff_rr_ship(client, consts):
    eve_affectee_attr = client.mk_eve_attr(stackable=True)
    client.mk_eve_buff(
        id_=consts.EveBuff.remote_repair_impedance,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.debuff_lance, cat_id=consts.EveEffCat.active)
    eve_affector_module = client.mk_eve_item(eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 1.0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_fit1 = api_sol.create_fit()
    api_affector_module1 = api_affector_fit1.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module1.change_mod(add_projs=[api_affectee_ship.id])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(0.5)
    api_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(-50)
    assert api_mod.stacking_mult is None
    assert api_mod.initial_val == approx(-50)
    assert api_mod.affectors.one().item_id == api_affector_module1.id
    assert api_mod.affectors.one().attr_id is None
    assert api_mod.affectors.one().hardcoded == approx(-50)
    api_affector_fit2 = api_sol.create_fit()
    api_affector_module2 = api_affector_fit2.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module2.change_mod(add_projs=[api_affectee_ship.id])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(0.5)
    api_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(-50)
    assert api_mod.stacking_mult is None
    assert api_mod.initial_val == approx(-50)
    assert api_mod.affectors.one().item_id in (api_affector_module1.id, api_affector_module2.id)
    assert api_mod.affectors.one().attr_id is None
    assert api_mod.affectors.one().hardcoded == approx(-50)
