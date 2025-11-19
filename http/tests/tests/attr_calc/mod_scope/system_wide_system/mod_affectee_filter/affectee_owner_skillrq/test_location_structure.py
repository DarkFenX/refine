from tests import approx


def test_unaffected(client, consts):
    # EVE does not use effects with "structureID" location with owner modifiable filter, so it's an
    # undefined behavior. Reefast just discards this modification as invalid
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.struct,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 100}, srqs={eve_skill_id: 1})
    eve_char_item_id = client.mk_eve_item()
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_item_id)
    api_fit.set_ship(type_id=eve_struct_id)
    api_sol.add_sw_effect(type_id=eve_affector_item_id)
    api_affectee_item = api_fit.add_drone(type_id=eve_affectee_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)
