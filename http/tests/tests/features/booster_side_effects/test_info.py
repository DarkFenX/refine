from tests import approx, check_no_field


def test_no_side_effects(client, consts):
    # Imitate just primary booster effect
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_booster_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Check default upon addition
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    assert isinstance(api_booster.id, str)
    with check_no_field():
        api_booster.kind  # noqa: B018
    with check_no_field():
        api_booster.side_effects  # noqa: B018
    with check_no_field():
        api_booster.attrs  # noqa: B018
    api_booster_id = api_booster.id
    # ID only
    api_booster.update(item_info_mode=consts.ApiItemInfoMode.id)
    assert api_booster.id == api_booster_id
    with check_no_field():
        api_booster.kind  # noqa: B018
    with check_no_field():
        api_booster.side_effects  # noqa: B018
    with check_no_field():
        api_booster.attrs  # noqa: B018
    # Partial
    api_booster.update(item_info_mode=consts.ApiItemInfoMode.partial)
    assert api_booster.id == api_booster_id
    assert api_booster.kind == consts.ApiItemKind.booster
    with check_no_field():
        api_booster.side_effects  # noqa: B018
    with check_no_field():
        api_booster.attrs  # noqa: B018
    # Full
    api_booster.update(item_info_mode=consts.ApiItemInfoMode.full)
    assert api_booster.id == api_booster_id
    assert api_booster.kind == consts.ApiItemKind.booster
    with check_no_field():
        api_booster.side_effects  # noqa: B018
    assert len(api_booster.attrs) == 1
    assert api_booster.attrs[eve_affector_attr_id].extra == approx(20)


def test_with_side_effects(client, consts):
    eve_primary_affector_attr_id = client.mk_eve_attr()
    eve_primary_affectee_attr_id = client.mk_eve_attr()
    eve_side1_chance_attr_id = client.mk_eve_attr()
    eve_side1_affector_attr_id = client.mk_eve_attr()
    eve_side1_affectee_attr_id = client.mk_eve_attr()
    eve_side2_chance_attr_id = client.mk_eve_attr()
    eve_side2_affector_attr_id = client.mk_eve_attr()
    eve_side2_affectee_attr_id = client.mk_eve_attr()
    eve_primary_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_primary_affector_attr_id,
        affectee_attr_id=eve_primary_affectee_attr_id)
    eve_primary_effect_id = client.mk_eve_effect(mod_info=[eve_primary_mod])
    eve_side1_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_side1_affector_attr_id,
        affectee_attr_id=eve_side1_affectee_attr_id)
    eve_side1_effect_id = client.mk_eve_effect(chance_attr_id=eve_side1_chance_attr_id, mod_info=[eve_side1_mod])
    eve_side2_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_side2_affector_attr_id,
        affectee_attr_id=eve_side2_affectee_attr_id)
    eve_side2_effect_id = client.mk_eve_effect(chance_attr_id=eve_side2_chance_attr_id, mod_info=[eve_side2_mod])
    eve_booster_id = client.mk_eve_item(
        attrs={
            eve_primary_affector_attr_id: 20,
            eve_side1_chance_attr_id: 0.4, eve_side1_affector_attr_id: 25,
            eve_side2_chance_attr_id: 0.2, eve_side2_affector_attr_id: 10},
        eff_ids=[eve_primary_effect_id, eve_side1_effect_id, eve_side2_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Check default upon addition
    api_booster = api_fit.add_booster(type_id=eve_booster_id, side_effects={eve_side2_effect_id: True})
    assert isinstance(api_booster.id, str)
    with check_no_field():
        api_booster.kind  # noqa: B018
    with check_no_field():
        api_booster.side_effects  # noqa: B018
    with check_no_field():
        api_booster.attrs  # noqa: B018
    api_booster_id = api_booster.id
    # ID only
    api_booster.update(item_info_mode=consts.ApiItemInfoMode.id)
    assert api_booster.id == api_booster_id
    with check_no_field():
        api_booster.kind  # noqa: B018
    with check_no_field():
        api_booster.side_effects  # noqa: B018
    with check_no_field():
        api_booster.attrs  # noqa: B018
    # Partial
    api_booster.update(item_info_mode=consts.ApiItemInfoMode.partial)
    assert api_booster.id == api_booster_id
    assert api_booster.kind == consts.ApiItemKind.booster
    assert len(api_booster.side_effects) == 2
    api_side1 = api_booster.side_effects[eve_side1_effect_id]
    assert api_side1.chance == approx(0.4)
    assert api_side1.status is False
    assert api_side1.str.op == consts.ApiSideEffectOp.perc
    assert api_side1.str.val == approx(25)
    api_side2 = api_booster.side_effects[eve_side2_effect_id]
    assert api_side2.chance == approx(0.2)
    assert api_side2.status is True
    assert api_side2.str.op == consts.ApiSideEffectOp.perc
    assert api_side2.str.val == approx(10)
    with check_no_field():
        api_booster.attrs  # noqa: B018
    # Full
    api_booster.update(item_info_mode=consts.ApiItemInfoMode.full)
    assert api_booster.id == api_booster_id
    assert api_booster.kind == consts.ApiItemKind.booster
    assert len(api_booster.side_effects) == 2
    api_side1 = api_booster.side_effects[eve_side1_effect_id]
    assert api_side1.chance == approx(0.4)
    assert api_side1.status is False
    assert api_side1.str.op == consts.ApiSideEffectOp.perc
    assert api_side1.str.val == approx(25)
    api_side2 = api_booster.side_effects[eve_side2_effect_id]
    assert api_side2.chance == approx(0.2)
    assert api_side2.status is True
    assert api_side2.str.op == consts.ApiSideEffectOp.perc
    assert api_side2.str.val == approx(10)
    assert len(api_booster.attrs) == 5
    assert api_booster.attrs[eve_primary_affector_attr_id].extra == approx(20)
    assert api_booster.attrs[eve_side1_chance_attr_id].extra == approx(0.4)
    assert api_booster.attrs[eve_side1_affector_attr_id].extra == approx(25)
    assert api_booster.attrs[eve_side2_chance_attr_id].extra == approx(0.2)
    assert api_booster.attrs[eve_side2_affector_attr_id].extra == approx(10)


def test_strength_matching(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_chance_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp1_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp2_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod1, eve_mod2])
    eve_ship_id = client.mk_eve_ship()
    eve_booster_id = client.mk_eve_item(
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr_id: 25},
        eff_ids=[eve_effect_id])
    eve_module1_id = client.mk_eve_item(grp_id=eve_grp1_id, attrs={eve_affectee_attr_id: 100})
    eve_module2_id = client.mk_eve_item(grp_id=eve_grp2_id, attrs={eve_affectee_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_module1_id)
    api_module2 = api_fit.add_module(type_id=eve_module2_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_module1.update().attrs[eve_affectee_attr_id].extra == approx(100)
    assert api_module2.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
    # Action
    api_booster.change_booster(side_effects={eve_effect_id: True})
    # Verification
    assert api_module1.update().attrs[eve_affectee_attr_id].extra == approx(125)
    assert api_module2.update().attrs[eve_affectee_attr_id].extra == approx(250)
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)


def test_strength_mismatch_op(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_chance_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp1_id,
        op=consts.EveModOp.pre_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp2_id,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod1, eve_mod2])
    eve_ship_id = client.mk_eve_ship()
    eve_booster_id = client.mk_eve_item(
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr_id: 1.25},
        eff_ids=[eve_effect_id])
    eve_module1_id = client.mk_eve_item(grp_id=eve_grp1_id, attrs={eve_affectee_attr_id: 100})
    eve_module2_id = client.mk_eve_item(grp_id=eve_grp2_id, attrs={eve_affectee_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_module1_id)
    api_module2 = api_fit.add_module(type_id=eve_module2_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_module1.update().attrs[eve_affectee_attr_id].extra == approx(100)
    assert api_module2.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str is None
    # Action
    api_booster.change_booster(side_effects={eve_effect_id: True})
    # Verification
    assert api_module1.update().attrs[eve_affectee_attr_id].extra == approx(125)
    assert api_module2.update().attrs[eve_affectee_attr_id].extra == approx(250)
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str is None


def test_strength_mismatch_attr(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_chance_attr_id = client.mk_eve_attr()
    eve_affector_attr1_id = client.mk_eve_attr()
    eve_affector_attr2_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp1_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr1_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp2_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr2_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod1, eve_mod2])
    eve_ship_id = client.mk_eve_ship()
    eve_booster_id = client.mk_eve_item(
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr1_id: 25, eve_affector_attr2_id: 25},
        eff_ids=[eve_effect_id])
    eve_module1_id = client.mk_eve_item(grp_id=eve_grp1_id, attrs={eve_affectee_attr_id: 100})
    eve_module2_id = client.mk_eve_item(grp_id=eve_grp2_id, attrs={eve_affectee_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_module1_id)
    api_module2 = api_fit.add_module(type_id=eve_module2_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_module1.update().attrs[eve_affectee_attr_id].extra == approx(100)
    assert api_module2.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str is None
    # Action
    api_booster.change_booster(side_effects={eve_effect_id: True})
    # Verification
    assert api_module1.update().attrs[eve_affectee_attr_id].extra == approx(125)
    assert api_module2.update().attrs[eve_affectee_attr_id].extra == approx(250)
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str is None


def test_modded_chance(client, consts):
    eve_chance_attr_id = client.mk_eve_attr()
    eve_chance_mod_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_side_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_side_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_side_mod])
    eve_booster_id = client.mk_eve_item(
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr_id: 25},
        eff_ids=[eve_side_effect_id])
    eve_chance_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_chance_mod_attr_id,
        affectee_attr_id=eve_chance_attr_id)
    eve_chance_effect_id = client.mk_eve_effect(mod_info=[eve_chance_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_chance_mod_attr_id: 0.9}, eff_ids=[eve_chance_effect_id])
    eve_char_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_char(type_id=eve_char_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_booster.update().side_effects[eve_side_effect_id].chance == approx(0.4)
    # Action
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_booster.update().side_effects[eve_side_effect_id].chance == approx(0.36)
    # Action
    api_implant.remove()
    # Verification
    assert api_booster.update().side_effects[eve_side_effect_id].chance == approx(0.4)


def test_modded_strength(client, consts):
    eve_chance_attr_id = client.mk_eve_attr()
    eve_str_mod_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_side_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_side_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_side_mod])
    eve_booster_id = client.mk_eve_item(
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr_id: 25},
        eff_ids=[eve_side_effect_id])
    eve_str_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_str_mod_attr_id,
        affectee_attr_id=eve_affector_attr_id)
    eve_str_effect_id = client.mk_eve_effect(mod_info=[eve_str_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_str_mod_attr_id: 1.2}, eff_ids=[eve_str_effect_id])
    eve_char_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_char(type_id=eve_char_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_booster.update().side_effects[eve_side_effect_id].str.val == approx(25)
    # Action
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_booster.update().side_effects[eve_side_effect_id].str.val == approx(30)
    # Action
    api_implant.remove()
    # Verification
    assert api_booster.update().side_effects[eve_side_effect_id].str.val == approx(25)
