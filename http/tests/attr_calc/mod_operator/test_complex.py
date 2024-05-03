from pytest import approx


def setup_test(
    client,
    consts,
    val_pre_ass,
    val_pre_mul,
    val_pre_div,
    val_mod_add,
    val_mod_sub,
    val_post_mul,
    val_post_div,
    val_post_perc,
    val_post_ass,
):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr(stackable=False)
    # Pre-assignment
    eve_mod_pre_ass = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect_pre_ass = client.mk_eve_effect(mod_info=[eve_mod_pre_ass])
    eve_item_affector_pre_ass = client.mk_eve_item(
        attrs={eve_affector_attr.id: val_pre_ass},
        eff_ids=[eve_effect_pre_ass.id])
    # Pre-multiplication
    eve_mod_pre_mul = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.pre_mul,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect_pre_mul = client.mk_eve_effect(mod_info=[eve_mod_pre_mul])
    eve_item_affector_pre_mul = client.mk_eve_item(
        attrs={eve_affector_attr.id: val_pre_mul},
        eff_ids=[eve_effect_pre_mul.id])
    # Pre-division
    eve_mod_pre_div = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.pre_div,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect_pre_div = client.mk_eve_effect(mod_info=[eve_mod_pre_div])
    eve_item_affector_pre_div = client.mk_eve_item(
        attrs={eve_affector_attr.id: val_pre_div},
        eff_ids=[eve_effect_pre_div.id])
    # Addition
    eve_mod_mod_add = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect_mod_add = client.mk_eve_effect(mod_info=[eve_mod_mod_add])
    eve_item_affector_mod_add = client.mk_eve_item(
        attrs={eve_affector_attr.id: val_mod_add},
        eff_ids=[eve_effect_mod_add.id])
    # Subtraction
    eve_mod_mod_sub = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.mod_sub,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect_mod_sub = client.mk_eve_effect(mod_info=[eve_mod_mod_sub])
    eve_item_affector_mod_sub = client.mk_eve_item(
        attrs={eve_affector_attr.id: val_mod_sub},
        eff_ids=[eve_effect_mod_sub.id])
    # Post-multiplication
    eve_mod_post_mul = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect_post_mul = client.mk_eve_effect(mod_info=[eve_mod_post_mul])
    eve_item_affector_post_mul = client.mk_eve_item(
        attrs={eve_affector_attr.id: val_post_mul},
        eff_ids=[eve_effect_post_mul.id])
    # Post-division
    eve_mod_post_div = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_div,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect_post_div = client.mk_eve_effect(mod_info=[eve_mod_post_div])
    eve_item_affector_post_div = client.mk_eve_item(
        attrs={eve_affector_attr.id: val_post_div},
        eff_ids=[eve_effect_post_div.id])
    # Post-percent
    eve_mod_post_perc = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect_post_perc = client.mk_eve_effect(mod_info=[eve_mod_post_perc])
    eve_item_affector_post_perc = client.mk_eve_item(
        attrs={eve_affector_attr.id: val_post_perc},
        eff_ids=[eve_effect_post_perc.id])
    # Post-assignment
    eve_mod_post_ass = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect_post_ass = client.mk_eve_effect(mod_info=[eve_mod_post_ass])
    eve_item_affector_post_ass = client.mk_eve_item(
        attrs={eve_affector_attr.id: val_post_ass},
        eff_ids=[eve_effect_post_ass.id])
    eve_item_affectee = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_item_affector_pre_ass.id)
    api_fit.add_rig(type_id=eve_item_affector_pre_mul.id)
    api_fit.add_rig(type_id=eve_item_affector_pre_div.id)
    api_fit.add_rig(type_id=eve_item_affector_mod_add.id)
    api_fit.add_rig(type_id=eve_item_affector_mod_sub.id)
    api_fit.add_rig(type_id=eve_item_affector_post_mul.id)
    api_fit.add_rig(type_id=eve_item_affector_post_div.id)
    api_fit.add_rig(type_id=eve_item_affector_post_perc.id)
    if val_post_ass:
        api_fit.add_rig(type_id=eve_item_affector_post_ass.id)
    api_item_affectee = api_fit.set_ship(type_id=eve_item_affectee.id)
    api_item_affectee.update()
    return api_item_affectee.attrs[eve_affectee_attr.id].dogma, api_item_affectee.mods[eve_affectee_attr.id]


def test_almost_all_in(client, consts):
    val_pre_ass = 5
    val_pre_mul = 50
    val_pre_div = 0.5
    val_mod_add = 10
    val_mod_sub = 63
    val_post_mul = 1.35
    val_post_div = 2.7
    val_post_perc = 15
    attr_value, attr_mods = setup_test(
        client=client,
        consts=consts,
        val_pre_ass=val_pre_ass,
        val_pre_mul=val_pre_mul,
        val_pre_div=val_pre_div,
        val_mod_add=val_mod_add,
        val_mod_sub=val_mod_sub,
        val_post_mul=val_post_mul,
        val_post_div=val_post_div,
        val_post_perc=val_post_perc,
        val_post_ass=None)
    expected_value = (
            (val_pre_ass * val_pre_mul / val_pre_div + val_mod_add - val_mod_sub)
            * val_post_mul / val_post_div * (1 + val_post_perc / 100))
    assert attr_value == approx(expected_value)
    assert attr_mods.find_by_op(op=consts.ApiModOp.pre_assign).one().val == approx(val_pre_ass)
    assert attr_mods.find_by_op(op=consts.ApiModOp.pre_mul).one().val == approx(val_pre_mul)
    assert attr_mods.find_by_op(op=consts.ApiModOp.pre_div).one().val == approx(val_pre_div)
    assert attr_mods.find_by_op(op=consts.ApiModOp.mod_add).one().val == approx(val_mod_add)
    assert attr_mods.find_by_op(op=consts.ApiModOp.mod_sub).one().val == approx(val_mod_sub)
    assert attr_mods.find_by_op(op=consts.ApiModOp.post_mul).one().val == approx(val_post_mul)
    assert attr_mods.find_by_op(op=consts.ApiModOp.post_div).one().val == approx(val_post_div)
    assert attr_mods.find_by_op(op=consts.ApiModOp.post_percent).one().val == approx(val_post_perc)


def test_all_in(client, consts):
    val_pre_ass = 5
    val_pre_mul = 50
    val_pre_div = 0.5
    val_mod_add = 10
    val_mod_sub = 63
    val_post_mul = 1.35
    val_post_div = 2.7
    val_post_perc = 15
    val_post_ass = 68
    attr_value, attr_mods = setup_test(
        client=client,
        consts=consts,
        val_pre_ass=val_pre_ass,
        val_pre_mul=val_pre_mul,
        val_pre_div=val_pre_div,
        val_mod_add=val_mod_add,
        val_mod_sub=val_mod_sub,
        val_post_mul=val_post_mul,
        val_post_div=val_post_div,
        val_post_perc=val_post_perc,
        val_post_ass=val_post_ass)
    assert attr_value == approx(val_post_ass)
    # When there is a post-assignment, all modifications before it are considered insignificant
    assert attr_mods.one().val == approx(val_post_ass)
    assert attr_mods.one().op == consts.ApiModOp.post_assign
