from pytest import approx


def test_absent_attr_combination(client, consts):
    # Check how calculator reacts to affector attribute which is absent
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_abs_attr = client.mk_eve_attr()
    eve_invalid_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_abs_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_valid_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_invalid_mod, eve_valid_mod])
    eve_item = client.mk_eve_item(
        attrs={eve_src_attr.id: 1.5, eve_tgt_attr.id: 100},
        eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_rig(type_id=eve_item.id)
    # Invalid source value shouldn't screw whole calculation process
    assert api_item.update().attrs[eve_tgt_attr.id].dogma == approx(150)
