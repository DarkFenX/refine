from fw import approx


def test_default_val(client, consts):
    eve_affector_attr_id = client.mk_eve_attr(def_val=20)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affectee_item = api_fit.set_ship(type_id=eve_affectee_item_id)
    api_affector_item = api_fit.add_rig(type_id=eve_affector_item_id)
    # Fetch affectee item attributes just to trigger base value fetch of the affector attribute
    api_affectee_item.update()
    assert api_affectee_item.attrs[eve_affectee_attr_id].dogma == approx(120)
    api_mod = api_affectee_item.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(20)
    assert api_mod.applied_val == approx(20)
    assert api_mod.affectors.one().item_id == api_affector_item.id
    assert api_mod.affectors.one().attr_id == eve_affector_attr_id
    # Check that value was stored on the affector item
    api_affector_item.update()
    assert api_affector_item.attrs[eve_affector_attr_id].base == approx(20)
    assert api_affector_item.attrs[eve_affector_attr_id].dogma == approx(20)
    assert api_affector_item.attrs[eve_affector_attr_id].extra == approx(20)
