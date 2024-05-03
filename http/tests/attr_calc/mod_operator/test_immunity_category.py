from pytest import approx


def setup_immunity_test(client, consts, cat_id):
    return setup_immunity_test_ext(client=client, consts=consts, affector1_cat_id=cat_id, affector2_cat_id=cat_id)


def setup_immunity_test_ext(client, consts, affector1_cat_id, affector2_cat_id):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr(stackable=False)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_affector1 = client.mk_eve_item(
        cat_id=affector1_cat_id,
        attrs={eve_affector_attr.id: 50},
        eff_ids=[eve_effect.id])
    eve_item_affector2 = client.mk_eve_item(
        cat_id=affector2_cat_id,
        attrs={eve_affector_attr.id: 100},
        eff_ids=[eve_effect.id])
    eve_item_affectee = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item_affector1 = api_fit.add_rig(type_id=eve_item_affector1.id)
    api_item_affector2 = api_fit.add_rig(type_id=eve_item_affector2.id)
    api_item_affectee = api_fit.set_ship(type_id=eve_item_affectee.id)
    api_item_affectee.update()
    return (
        api_item_affectee.attrs[eve_affectee_attr.id].dogma,
        api_item_affectee.mods[eve_affectee_attr.id],
        api_item_affector1,
        api_item_affector2)


def test_ship(client, consts):
    attr_val, attr_mods, api_item_affector1, api_item_affector2 = setup_immunity_test(
        client, consts, cat_id=consts.EveItemCat.ship)
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one().penalized is False
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one().penalized is False


def test_charge(client, consts):
    attr_val, attr_mods, api_item_affector1, api_item_affector2 = setup_immunity_test(
        client, consts, cat_id=consts.EveItemCat.charge)
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one().penalized is False
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one().penalized is False


def test_skill(client, consts):
    attr_val, attr_mods, api_item_affector1, api_item_affector2 = setup_immunity_test(
        client, consts, cat_id=consts.EveItemCat.skill)
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one().penalized is False
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one().penalized is False


def test_implant(client, consts):
    attr_val, attr_mods, api_item_affector1, api_item_affector2 = setup_immunity_test(
        client, consts, cat_id=consts.EveItemCat.implant)
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one().penalized is False
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one().penalized is False


def test_subsystem(client, consts):
    attr_val, attr_mods, api_item_affector1, api_item_affector2 = setup_immunity_test(
        client, consts, cat_id=consts.EveItemCat.subsystem)
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one().penalized is False
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one().penalized is False


def test_mixed(client, consts):
    attr_val, attr_mods, api_item_affector1, api_item_affector2 = setup_immunity_test_ext(
        client, consts, affector1_cat_id=consts.EveItemCat.charge, affector2_cat_id=consts.EveItemCat.implant)
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one().penalized is False
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one().penalized is False


def test_with_not_immune(client, consts):
    attr_val, attr_mods, api_item_affector1, api_item_affector2 = setup_immunity_test_ext(
        client, consts, affector1_cat_id=consts.EveItemCat.charge, affector2_cat_id=consts.EveItemCat.module)
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one().penalized is False
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one().penalized is True
