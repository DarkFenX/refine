from tests import approx


def test_unaffected(client, consts):
    # Currently there are no effects used by EVE which affect multiple items filtered by "other"
    # domain (i.e. when affecting item references ship via "other" reference), so we don't support
    # it either
    eve_skill = client.mk_eve_item()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_srq,
        dom=consts.EveModDom.other,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 100}, srqs={eve_skill.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_affector_item.id)
    api_affectee_item1 = api_fit.add_rig(type_id=eve_affectee_item.id)
    api_affectee_item2 = api_fit.add_implant(type_id=eve_affectee_item.id)
    assert api_affectee_item1.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    assert api_affectee_item2.update().attrs[eve_affectee_attr.id].dogma == approx(100)
