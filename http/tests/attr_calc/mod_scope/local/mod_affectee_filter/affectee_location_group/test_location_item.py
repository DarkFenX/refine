from tests import approx


def test_unaffected(client, consts):
    # Currently there are no effects used by EVE which affect multiple items filtered by "self"
    # location (i.e. when location is item, not, say, ship), so we don't support it either
    eve_grp_id = client.mk_eve_item_group()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.item,
        grp=eve_grp_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_affector_item_id)
    api_affectee_item1 = api_fit.add_rig(type_id=eve_affectee_item_id)
    api_affectee_item2 = api_fit.add_implant(type_id=eve_affectee_item_id)
    assert api_affectee_item1.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_affectee_item2.update().attrs[eve_affectee_attr_id].dogma == approx(100)
