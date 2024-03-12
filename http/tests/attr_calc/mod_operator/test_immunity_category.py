from pytest import approx


def setup_immunity_test(client, consts, cat_id):
    return setup_immunity_test_ext(client=client, consts=consts, src1_cat_id=cat_id, src2_cat_id=cat_id)


def setup_immunity_test_ext(client, consts, src1_cat_id, src2_cat_id):
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr(stackable=False)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_src1 = client.mk_eve_item(cat_id=src1_cat_id, attrs={eve_src_attr.id: 50}, eff_ids=[eve_effect.id])
    eve_item_src2 = client.mk_eve_item(cat_id=src2_cat_id, attrs={eve_src_attr.id: 100}, eff_ids=[eve_effect.id])
    eve_item_tgt = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item_src1 = api_fit.add_rig(type_id=eve_item_src1.id)
    api_item_src2 = api_fit.add_rig(type_id=eve_item_src2.id)
    api_item_tgt = api_fit.set_ship(type_id=eve_item_tgt.id)
    api_item_tgt.update()
    return (
        api_item_tgt.attrs[eve_tgt_attr.id].dogma,
        api_item_tgt.mods[eve_tgt_attr.id],
        api_item_src1,
        api_item_src2)


def test_ship(client, consts):
    attr_val, attr_mods, api_item_src1, api_item_src2 = setup_immunity_test(
        client, consts, cat_id=consts.EveItemCat.ship)
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_src_item(src_item_id=api_item_src1.id).one().penalized is False
    assert attr_mods.find_by_src_item(src_item_id=api_item_src2.id).one().penalized is False


def test_charge(client, consts):
    attr_val, attr_mods, api_item_src1, api_item_src2 = setup_immunity_test(
        client, consts, cat_id=consts.EveItemCat.charge)
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_src_item(src_item_id=api_item_src1.id).one().penalized is False
    assert attr_mods.find_by_src_item(src_item_id=api_item_src2.id).one().penalized is False


def test_skill(client, consts):
    attr_val, attr_mods, api_item_src1, api_item_src2 = setup_immunity_test(
        client, consts, cat_id=consts.EveItemCat.skill)
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_src_item(src_item_id=api_item_src1.id).one().penalized is False
    assert attr_mods.find_by_src_item(src_item_id=api_item_src2.id).one().penalized is False


def test_implant(client, consts):
    attr_val, attr_mods, api_item_src1, api_item_src2 = setup_immunity_test(
        client, consts, cat_id=consts.EveItemCat.implant)
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_src_item(src_item_id=api_item_src1.id).one().penalized is False
    assert attr_mods.find_by_src_item(src_item_id=api_item_src2.id).one().penalized is False


def test_subsystem(client, consts):
    attr_val, attr_mods, api_item_src1, api_item_src2 = setup_immunity_test(
        client, consts, cat_id=consts.EveItemCat.subsystem)
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_src_item(src_item_id=api_item_src1.id).one().penalized is False
    assert attr_mods.find_by_src_item(src_item_id=api_item_src2.id).one().penalized is False


def test_mixed(client, consts):
    attr_val, attr_mods, api_item_src1, api_item_src2 = setup_immunity_test_ext(
        client, consts, src1_cat_id=consts.EveItemCat.charge, src2_cat_id=consts.EveItemCat.implant)
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_src_item(src_item_id=api_item_src1.id).one().penalized is False
    assert attr_mods.find_by_src_item(src_item_id=api_item_src2.id).one().penalized is False


def test_with_not_immune(client, consts):
    attr_val, attr_mods, api_item_src1, api_item_src2 = setup_immunity_test_ext(
        client, consts, src1_cat_id=consts.EveItemCat.charge, src2_cat_id=consts.EveItemCat.module)
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_src_item(src_item_id=api_item_src1.id).one().penalized is False
    assert attr_mods.find_by_src_item(src_item_id=api_item_src2.id).one().penalized is True
