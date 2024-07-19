from tests import approx


def test_default_val(client, consts):
    eve_affector_attr = client.mk_eve_attr(def_val=20)
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 100}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.set_char(type_id=eve_item.id)
    api_item.update()
    assert api_item.attrs[eve_affectee_attr.id].dogma == approx(120)
    api_mod = api_item.mods[eve_affectee_attr.id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(20)
    assert api_mod.applied_val == approx(20)
    assert api_mod.affectors.one().item_id == api_item.id
    assert api_mod.affectors.one().attr_id == eve_affector_attr.id
