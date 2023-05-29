from pytest import approx


def get_dogma_value(client, consts, cat_id):
    return get_dogma_value_ext(client=client, consts=consts, src1_cat_id=cat_id, src2_cat_id=cat_id)


def get_dogma_value_ext(client, consts, src1_cat_id, src2_cat_id):
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr(stackable=False)
    eve_modifier = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.ship,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_modifier])
    eve_item_src1 = client.mk_eve_item(cat_id=src1_cat_id, attrs={eve_src_attr.id: 50}, eff_ids=[eve_effect.id])
    eve_item_src2 = client.mk_eve_item(cat_id=src2_cat_id, attrs={eve_src_attr.id: 100}, eff_ids=[eve_effect.id])
    eve_item_tgt = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.add_rig(type_id=eve_item_src1.id)
    api_fit.add_rig(type_id=eve_item_src2.id)
    api_item_tgt = api_fit.set_ship(type_id=eve_item_tgt.id)
    return api_item_tgt.update().attr_vals[eve_tgt_attr.id].dogma


def test_ship(client, consts):
    value = get_dogma_value(client, consts, cat_id=consts.ItemCat.ship)
    assert value == approx(300)


def test_charge(client, consts):
    value = get_dogma_value(client, consts, cat_id=consts.ItemCat.charge)
    assert value == approx(300)


def test_skill(client, consts):
    value = get_dogma_value(client, consts, cat_id=consts.ItemCat.skill)
    assert value == approx(300)


def test_implant(client, consts):
    value = get_dogma_value(client, consts, cat_id=consts.ItemCat.implant)
    assert value == approx(300)


def test_subsystem(client, consts):
    value = get_dogma_value(client, consts, cat_id=consts.ItemCat.subsystem)
    assert value == approx(300)


def test_mixed(client, consts):
    value = get_dogma_value_ext(client, consts, src1_cat_id=consts.ItemCat.charge, src2_cat_id=consts.ItemCat.implant)
    assert value == approx(300)


def test_with_not_immune(client, consts):
    value = get_dogma_value_ext(client, consts, src1_cat_id=consts.ItemCat.charge, src2_cat_id=consts.ItemCat.module)
    assert value == approx(300)
