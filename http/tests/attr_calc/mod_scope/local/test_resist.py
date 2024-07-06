from pytest import approx


def test_unresisted(client, consts):
    # Engine assumes local effects cannot be resisted
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_resist_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(resist_attr_id=eve_resist_attr.id, mod_info=[eve_mod])
    eve_rig = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 100, eve_resist_attr.id: 0.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_rig(type_id=eve_rig.id)
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(120)
