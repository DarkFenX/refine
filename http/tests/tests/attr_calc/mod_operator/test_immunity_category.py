from fw import approx


def setup_immunity_test(*, client, consts, cat_id):
    return setup_immunity_test_ext(
        client=client,
        consts=consts,
        affector1_cat_id=cat_id,
        affector2_cat_id=cat_id)


def setup_immunity_test_ext(*, client, consts, affector1_cat_id, affector2_cat_id):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(stackable=False)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_affector1_id = client.mk_eve_item(
        cat_id=affector1_cat_id,
        attrs={eve_affector_attr_id: 50},
        eff_ids=[eve_effect_id])
    eve_item_affector2_id = client.mk_eve_item(
        cat_id=affector2_cat_id,
        attrs={eve_affector_attr_id: 100},
        eff_ids=[eve_effect_id])
    eve_item_affectee_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item_affector1 = api_fit.add_rig(type_id=eve_item_affector1_id)
    api_item_affector2 = api_fit.add_rig(type_id=eve_item_affector2_id)
    api_item_affectee = api_fit.set_ship(type_id=eve_item_affectee_id)
    api_item_affectee.update()
    return (
        api_item_affectee.attrs[eve_affectee_attr_id].dogma,
        api_item_affectee.mods[eve_affectee_attr_id],
        api_item_affector1,
        api_item_affector2)


def test_ship(client, consts):
    attr_val, attr_mods, api_item_affector1, api_item_affector2 = setup_immunity_test(
        client=client, consts=consts, cat_id=consts.EveItemCat.ship)
    # Verification
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one().stacking_mult is None
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one().stacking_mult is None


def test_charge(client, consts):
    attr_val, attr_mods, api_item_affector1, api_item_affector2 = setup_immunity_test(
        client=client, consts=consts, cat_id=consts.EveItemCat.charge)
    # Verification
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one().stacking_mult is None
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one().stacking_mult is None


def test_skill(client, consts):
    attr_val, attr_mods, api_item_affector1, api_item_affector2 = setup_immunity_test(
        client=client, consts=consts, cat_id=consts.EveItemCat.skill)
    # Verification
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one().stacking_mult is None
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one().stacking_mult is None


def test_implant(client, consts):
    attr_val, attr_mods, api_item_affector1, api_item_affector2 = setup_immunity_test(
        client=client, consts=consts, cat_id=consts.EveItemCat.implant)
    # Verification
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one().stacking_mult is None
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one().stacking_mult is None


def test_subsystem(client, consts):
    attr_val, attr_mods, api_item_affector1, api_item_affector2 = setup_immunity_test(
        client=client, consts=consts, cat_id=consts.EveItemCat.subsystem)
    # Verification
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one().stacking_mult is None
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one().stacking_mult is None


def test_mixed(client, consts):
    attr_val, attr_mods, api_item_affector1, api_item_affector2 = setup_immunity_test_ext(
        client=client,
        consts=consts,
        affector1_cat_id=consts.EveItemCat.charge,
        affector2_cat_id=consts.EveItemCat.implant)
    # Verification
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one().stacking_mult is None
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one().stacking_mult is None


def test_with_not_immune(client, consts):
    attr_val, attr_mods, api_item_affector1, api_item_affector2 = setup_immunity_test_ext(
        client=client,
        consts=consts,
        affector1_cat_id=consts.EveItemCat.charge,
        affector2_cat_id=consts.EveItemCat.module)
    # Verification
    assert attr_val == approx(300)
    assert len(attr_mods) == 2
    assert attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one().stacking_mult is None
    assert attr_mods.find_by_affector_item(
        affector_item_id=api_item_affector2.id).one().stacking_mult == approx(consts.PenaltyStr.p1)
