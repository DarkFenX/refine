from tests import approx


def test_op_preassign(client, consts):
    eve_chance_attr = client.mk_eve_attr()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(chance_attr_id=eve_chance_attr.id, mod_info=[eve_mod])
    eve_booster = client.mk_eve_item(attrs={eve_chance_attr.id: 0.4, eve_affector_attr.id: 25}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_booster = api_fit.add_booster(type_id=eve_booster.id)
    api_booster.update(item_info_mode=consts.ApiItemInfoMode.full)
    api_side = api_booster.side_effects[eve_effect.id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str is None


def test_op_postassign(client, consts):
    eve_chance_attr = client.mk_eve_attr()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(chance_attr_id=eve_chance_attr.id, mod_info=[eve_mod])
    eve_booster = client.mk_eve_item(attrs={eve_chance_attr.id: 0.4, eve_affector_attr.id: 25}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_booster = api_fit.add_booster(type_id=eve_booster.id)
    api_booster.update(item_info_mode=consts.ApiItemInfoMode.full)
    api_side = api_booster.side_effects[eve_effect.id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str is None
