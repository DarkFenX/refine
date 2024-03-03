from pytest import approx


def get_dogma_value(
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
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr(stackable=False)
    # Pre-assignment
    eve_mod_pre_ass = client.mk_eve_effect_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.ship,
        op=consts.ModOp.pre_assign,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect_pre_ass = client.mk_eve_effect(mod_info=[eve_mod_pre_ass])
    eve_item_src_pre_ass = client.mk_eve_item(
        attrs={eve_src_attr.id: val_pre_ass},
        eff_ids=[eve_effect_pre_ass.id])
    # Pre-multiplication
    eve_mod_pre_mul = client.mk_eve_effect_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.ship,
        op=consts.ModOp.pre_mul,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect_pre_mul = client.mk_eve_effect(mod_info=[eve_mod_pre_mul])
    eve_item_src_pre_mul = client.mk_eve_item(
        attrs={eve_src_attr.id: val_pre_mul},
        eff_ids=[eve_effect_pre_mul.id])
    # Pre-division
    eve_mod_pre_div = client.mk_eve_effect_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.ship,
        op=consts.ModOp.pre_div,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect_pre_div = client.mk_eve_effect(mod_info=[eve_mod_pre_div])
    eve_item_src_pre_div = client.mk_eve_item(
        attrs={eve_src_attr.id: val_pre_div},
        eff_ids=[eve_effect_pre_div.id])
    # Addition
    eve_mod_mod_add = client.mk_eve_effect_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.ship,
        op=consts.ModOp.mod_add,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect_mod_add = client.mk_eve_effect(mod_info=[eve_mod_mod_add])
    eve_item_src_mod_add = client.mk_eve_item(
        attrs={eve_src_attr.id: val_mod_add},
        eff_ids=[eve_effect_mod_add.id])
    # Subtraction
    eve_mod_mod_sub = client.mk_eve_effect_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.ship,
        op=consts.ModOp.mod_sub,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect_mod_sub = client.mk_eve_effect(mod_info=[eve_mod_mod_sub])
    eve_item_src_mod_sub = client.mk_eve_item(
        attrs={eve_src_attr.id: val_mod_sub},
        eff_ids=[eve_effect_mod_sub.id])
    # Post-multiplication
    eve_mod_post_mul = client.mk_eve_effect_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.ship,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect_post_mul = client.mk_eve_effect(mod_info=[eve_mod_post_mul])
    eve_item_src_post_mul = client.mk_eve_item(
        attrs={eve_src_attr.id: val_post_mul},
        eff_ids=[eve_effect_post_mul.id])
    # Post-division
    eve_mod_post_div = client.mk_eve_effect_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.ship,
        op=consts.ModOp.post_div,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect_post_div = client.mk_eve_effect(mod_info=[eve_mod_post_div])
    eve_item_src_post_div = client.mk_eve_item(
        attrs={eve_src_attr.id: val_post_div},
        eff_ids=[eve_effect_post_div.id])
    # Post-percent
    eve_mod_post_perc = client.mk_eve_effect_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.ship,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect_post_perc = client.mk_eve_effect(mod_info=[eve_mod_post_perc])
    eve_item_src_post_perc = client.mk_eve_item(
        attrs={eve_src_attr.id: val_post_perc},
        eff_ids=[eve_effect_post_perc.id])
    # Post-assignment
    eve_mod_post_ass = client.mk_eve_effect_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.ship,
        op=consts.ModOp.post_assign,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect_post_ass = client.mk_eve_effect(mod_info=[eve_mod_post_ass])
    eve_item_src_post_ass = client.mk_eve_item(
        attrs={eve_src_attr.id: val_post_ass},
        eff_ids=[eve_effect_post_ass.id])
    eve_item_tgt = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.add_rig(type_id=eve_item_src_pre_ass.id)
    api_fit.add_rig(type_id=eve_item_src_pre_mul.id)
    api_fit.add_rig(type_id=eve_item_src_pre_div.id)
    api_fit.add_rig(type_id=eve_item_src_mod_add.id)
    api_fit.add_rig(type_id=eve_item_src_mod_sub.id)
    api_fit.add_rig(type_id=eve_item_src_post_mul.id)
    api_fit.add_rig(type_id=eve_item_src_post_div.id)
    api_fit.add_rig(type_id=eve_item_src_post_perc.id)
    if val_post_ass:
        api_fit.add_rig(type_id=eve_item_src_post_ass.id)
    api_item_tgt = api_fit.set_ship(type_id=eve_item_tgt.id)
    return api_item_tgt.update().attrs[eve_tgt_attr.id].dogma


def test_almost_all_in(client, consts):
    val_pre_ass = 5
    val_pre_mul = 50
    val_pre_div = 0.5
    val_mod_add = 10
    val_mod_sub = 63
    val_post_mul = 1.35
    val_post_div = 2.7
    val_post_perc = 15
    value = get_dogma_value(
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
        val_post_ass=0)
    expected_value = (
            (val_pre_ass * val_pre_mul / val_pre_div + val_mod_add - val_mod_sub)
            * val_post_mul / val_post_div * (1 + val_post_perc / 100))
    assert value == approx(expected_value)


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
    value = get_dogma_value(
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
    assert value == approx(val_post_ass)
