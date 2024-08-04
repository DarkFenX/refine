from tests import approx, check_no_field


def test_no_side_effects(client, consts):
    # Imitate just primary booster effect
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_booster = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Check default upon addition
    api_booster = api_fit.add_booster(type_id=eve_booster.id)
    assert isinstance(api_booster.id, str)
    with check_no_field():
        api_booster.kind  # pylint: disable=W0104
    with check_no_field():
        api_booster.side_effects  # pylint: disable=W0104
    with check_no_field():
        api_booster.attrs  # pylint: disable=W0104
    api_booster_id = api_booster.id
    # ID only
    api_booster.update(item_info_mode=consts.ApiItemInfoMode.id)
    assert api_booster.id == api_booster_id
    with check_no_field():
        api_booster.kind  # pylint: disable=W0104
    with check_no_field():
        api_booster.side_effects  # pylint: disable=W0104
    with check_no_field():
        api_booster.attrs  # pylint: disable=W0104
    # Partial
    api_booster.update(item_info_mode=consts.ApiItemInfoMode.partial)
    assert api_booster.id == api_booster_id
    assert api_booster.kind == consts.ApiItemKind.booster
    with check_no_field():
        api_booster.side_effects  # pylint: disable=W0104
    with check_no_field():
        api_booster.attrs  # pylint: disable=W0104
    # Full
    api_booster.update(item_info_mode=consts.ApiItemInfoMode.full)
    assert api_booster.id == api_booster_id
    assert api_booster.kind == consts.ApiItemKind.booster
    with check_no_field():
        api_booster.side_effects  # pylint: disable=W0104
    assert len(api_booster.attrs) == 1
    assert api_booster.attrs[eve_affector_attr.id].extra == approx(20)


def test_with_side_effects(client, consts):
    eve_primary_affector_attr = client.mk_eve_attr()
    eve_primary_affectee_attr = client.mk_eve_attr()
    eve_side1_chance_attr = client.mk_eve_attr()
    eve_side1_affector_attr = client.mk_eve_attr()
    eve_side1_affectee_attr = client.mk_eve_attr()
    eve_side2_chance_attr = client.mk_eve_attr()
    eve_side2_affector_attr = client.mk_eve_attr()
    eve_side2_affectee_attr = client.mk_eve_attr()
    eve_primary_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_primary_affector_attr.id,
        affectee_attr_id=eve_primary_affectee_attr.id)
    eve_primary_effect = client.mk_eve_effect(mod_info=[eve_primary_mod])
    eve_side1_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_side1_affector_attr.id,
        affectee_attr_id=eve_side1_affectee_attr.id)
    eve_side1_effect = client.mk_eve_effect(chance_attr_id=eve_side1_chance_attr.id, mod_info=[eve_side1_mod])
    eve_side2_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_side2_affector_attr.id,
        affectee_attr_id=eve_side2_affectee_attr.id)
    eve_side2_effect = client.mk_eve_effect(chance_attr_id=eve_side2_chance_attr.id, mod_info=[eve_side2_mod])
    eve_booster = client.mk_eve_item(
        attrs={
            eve_primary_affector_attr.id: 20,
            eve_side1_chance_attr.id: 0.4, eve_side1_affector_attr.id: 25,
            eve_side2_chance_attr.id: 0.2, eve_side2_affector_attr.id: 10},
        eff_ids=[eve_primary_effect.id, eve_side1_effect.id, eve_side2_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Check default upon addition
    api_booster = api_fit.add_booster(type_id=eve_booster.id, side_effects={eve_side2_effect.id: True})
    assert isinstance(api_booster.id, str)
    with check_no_field():
        api_booster.kind  # pylint: disable=W0104
    with check_no_field():
        api_booster.side_effects  # pylint: disable=W0104
    with check_no_field():
        api_booster.attrs  # pylint: disable=W0104
    api_booster_id = api_booster.id
    # ID only
    api_booster.update(item_info_mode=consts.ApiItemInfoMode.id)
    assert api_booster.id == api_booster_id
    with check_no_field():
        api_booster.kind  # pylint: disable=W0104
    with check_no_field():
        api_booster.side_effects  # pylint: disable=W0104
    with check_no_field():
        api_booster.attrs  # pylint: disable=W0104
    # Partial
    api_booster.update(item_info_mode=consts.ApiItemInfoMode.partial)
    assert api_booster.id == api_booster_id
    assert api_booster.kind == consts.ApiItemKind.booster
    assert len(api_booster.side_effects) == 2
    api_side1 = api_booster.side_effects[eve_side1_effect.id]
    assert api_side1.chance == approx(0.4)
    assert api_side1.status is False
    assert api_side1.str.op == consts.ApiSideEffectOp.perc
    assert api_side1.str.val == approx(25)
    api_side2 = api_booster.side_effects[eve_side2_effect.id]
    assert api_side2.chance == approx(0.2)
    assert api_side2.status is True
    assert api_side2.str.op == consts.ApiSideEffectOp.perc
    assert api_side2.str.val == approx(10)
    with check_no_field():
        api_booster.attrs  # pylint: disable=W0104
    # Full
    api_booster.update(item_info_mode=consts.ApiItemInfoMode.full)
    assert api_booster.id == api_booster_id
    assert api_booster.kind == consts.ApiItemKind.booster
    assert len(api_booster.side_effects) == 2
    api_side1 = api_booster.side_effects[eve_side1_effect.id]
    assert api_side1.chance == approx(0.4)
    assert api_side1.status is False
    assert api_side1.str.op == consts.ApiSideEffectOp.perc
    assert api_side1.str.val == approx(25)
    api_side2 = api_booster.side_effects[eve_side2_effect.id]
    assert api_side2.chance == approx(0.2)
    assert api_side2.status is True
    assert api_side2.str.op == consts.ApiSideEffectOp.perc
    assert api_side2.str.val == approx(10)
    assert len(api_booster.attrs) == 5
    assert api_booster.attrs[eve_primary_affector_attr.id].extra == approx(20)
    assert api_booster.attrs[eve_side1_chance_attr.id].extra == approx(0.4)
    assert api_booster.attrs[eve_side1_affector_attr.id].extra == approx(25)
    assert api_booster.attrs[eve_side2_chance_attr.id].extra == approx(0.2)
    assert api_booster.attrs[eve_side2_affector_attr.id].extra == approx(10)


def test_strength_matching(client, consts):
    eve_grp1 = client.mk_eve_item_group()
    eve_grp2 = client.mk_eve_item_group()
    eve_chance_attr = client.mk_eve_attr()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.ship,
        grp=eve_grp1.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.ship,
        grp=eve_grp2.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(chance_attr_id=eve_chance_attr.id, mod_info=[eve_mod1, eve_mod2])
    eve_ship = client.mk_eve_ship()
    eve_booster = client.mk_eve_item(
        attrs={eve_chance_attr.id: 0.4, eve_affector_attr.id: 25},
        eff_ids=[eve_effect.id])
    eve_module1 = client.mk_eve_item(grp_id=eve_grp1.id, attrs={eve_affectee_attr.id: 100})
    eve_module2 = client.mk_eve_item(grp_id=eve_grp2.id, attrs={eve_affectee_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_module1 = api_fit.add_mod(type_id=eve_module1.id)
    api_module2 = api_fit.add_mod(type_id=eve_module2.id)
    api_booster = api_fit.add_booster(type_id=eve_booster.id)
    # Verification
    assert api_module1.update().attrs[eve_affectee_attr.id].extra == approx(100)
    assert api_module2.update().attrs[eve_affectee_attr.id].extra == approx(200)
    api_side = api_booster.update().side_effects[eve_effect.id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
    # Action
    api_booster.change_booster(side_effects={eve_effect.id: True})
    # Verification
    assert api_module1.update().attrs[eve_affectee_attr.id].extra == approx(125)
    assert api_module2.update().attrs[eve_affectee_attr.id].extra == approx(250)
    api_side = api_booster.update().side_effects[eve_effect.id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)


def test_strength_mismatch_op(client, consts):
    eve_grp1 = client.mk_eve_item_group()
    eve_grp2 = client.mk_eve_item_group()
    eve_chance_attr = client.mk_eve_attr()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.ship,
        grp=eve_grp1.id,
        op=consts.EveModOp.pre_mul,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.ship,
        grp=eve_grp2.id,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(chance_attr_id=eve_chance_attr.id, mod_info=[eve_mod1, eve_mod2])
    eve_ship = client.mk_eve_ship()
    eve_booster = client.mk_eve_item(
        attrs={eve_chance_attr.id: 0.4, eve_affector_attr.id: 1.25},
        eff_ids=[eve_effect.id])
    eve_module1 = client.mk_eve_item(grp_id=eve_grp1.id, attrs={eve_affectee_attr.id: 100})
    eve_module2 = client.mk_eve_item(grp_id=eve_grp2.id, attrs={eve_affectee_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_module1 = api_fit.add_mod(type_id=eve_module1.id)
    api_module2 = api_fit.add_mod(type_id=eve_module2.id)
    api_booster = api_fit.add_booster(type_id=eve_booster.id)
    # Verification
    assert api_module1.update().attrs[eve_affectee_attr.id].extra == approx(100)
    assert api_module2.update().attrs[eve_affectee_attr.id].extra == approx(200)
    api_side = api_booster.update().side_effects[eve_effect.id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str is None
    # Action
    api_booster.change_booster(side_effects={eve_effect.id: True})
    # Verification
    assert api_module1.update().attrs[eve_affectee_attr.id].extra == approx(125)
    assert api_module2.update().attrs[eve_affectee_attr.id].extra == approx(250)
    api_side = api_booster.update().side_effects[eve_effect.id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str is None


def test_strength_mismatch_attr(client, consts):
    eve_grp1 = client.mk_eve_item_group()
    eve_grp2 = client.mk_eve_item_group()
    eve_chance_attr = client.mk_eve_attr()
    eve_affector_attr1 = client.mk_eve_attr()
    eve_affector_attr2 = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.ship,
        grp=eve_grp1.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr1.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.ship,
        grp=eve_grp2.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr2.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(chance_attr_id=eve_chance_attr.id, mod_info=[eve_mod1, eve_mod2])
    eve_ship = client.mk_eve_ship()
    eve_booster = client.mk_eve_item(
        attrs={eve_chance_attr.id: 0.4, eve_affector_attr1.id: 25, eve_affector_attr2.id: 25},
        eff_ids=[eve_effect.id])
    eve_module1 = client.mk_eve_item(grp_id=eve_grp1.id, attrs={eve_affectee_attr.id: 100})
    eve_module2 = client.mk_eve_item(grp_id=eve_grp2.id, attrs={eve_affectee_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_module1 = api_fit.add_mod(type_id=eve_module1.id)
    api_module2 = api_fit.add_mod(type_id=eve_module2.id)
    api_booster = api_fit.add_booster(type_id=eve_booster.id)
    # Verification
    assert api_module1.update().attrs[eve_affectee_attr.id].extra == approx(100)
    assert api_module2.update().attrs[eve_affectee_attr.id].extra == approx(200)
    api_side = api_booster.update().side_effects[eve_effect.id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str is None
    # Action
    api_booster.change_booster(side_effects={eve_effect.id: True})
    # Verification
    assert api_module1.update().attrs[eve_affectee_attr.id].extra == approx(125)
    assert api_module2.update().attrs[eve_affectee_attr.id].extra == approx(250)
    api_side = api_booster.update().side_effects[eve_effect.id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str is None
