from tests import approx, check_no_field


def setup_penalization_test(*, client, consts, stackable):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(stackable=stackable)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent_immune,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_affector1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_item_affector2_id = client.mk_eve_item(attrs={eve_affector_attr_id: 50}, eff_ids=[eve_effect_id])
    eve_item_affector3_id = client.mk_eve_item(attrs={eve_affector_attr_id: -90}, eff_ids=[eve_effect_id])
    eve_item_affector4_id = client.mk_eve_item(attrs={eve_affector_attr_id: -25}, eff_ids=[eve_effect_id])
    eve_item_affector5_id = client.mk_eve_item(attrs={eve_affector_attr_id: 400}, eff_ids=[eve_effect_id])
    # 0% bonus is considered insignificant, and won't be exposed as modification
    eve_item_affector6_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0}, eff_ids=[eve_effect_id])
    eve_item_affectee_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item_affector1 = api_fit.add_rig(type_id=eve_item_affector1_id)
    api_item_affector2 = api_fit.add_rig(type_id=eve_item_affector2_id)
    api_item_affector3 = api_fit.add_rig(type_id=eve_item_affector3_id)
    api_item_affector4 = api_fit.add_rig(type_id=eve_item_affector4_id)
    api_item_affector5 = api_fit.add_rig(type_id=eve_item_affector5_id)
    api_fit.add_rig(type_id=eve_item_affector6_id)
    api_item_affectee = api_fit.set_ship(type_id=eve_item_affectee_id)
    api_item_affectee.update()
    return (
        api_item_affectee.attrs[eve_affectee_attr_id].dogma,
        api_item_affectee.mods[eve_affectee_attr_id],
        api_item_affector1,
        api_item_affector2,
        api_item_affector3,
        api_item_affector4,
        api_item_affector5)


def test_non_penalized(client, consts):
    (attr_val,
     attr_mods,
     api_item_affector1,
     api_item_affector2,
     api_item_affector3,
     api_item_affector4,
     api_item_affector5) = setup_penalization_test(client=client, consts=consts, stackable=True)
    # Verification
    assert attr_val == approx(67.5)
    assert len(attr_mods) == 5
    api_mod1 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one()
    assert api_mod1.op == consts.ApiModOp.post_percent
    assert api_mod1.initial_val == approx(20)
    assert api_mod1.stacking_mult is None
    assert api_mod1.applied_val == approx(20)
    api_mod2 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one()
    assert api_mod2.op == consts.ApiModOp.post_percent
    assert api_mod2.initial_val == approx(50)
    assert api_mod2.stacking_mult is None
    assert api_mod2.applied_val == approx(50)
    api_mod3 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector3.id).one()
    assert api_mod3.op == consts.ApiModOp.post_percent
    assert api_mod3.initial_val == approx(-90)
    assert api_mod3.stacking_mult is None
    assert api_mod3.applied_val == approx(-90)
    api_mod4 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector4.id).one()
    assert api_mod4.op == consts.ApiModOp.post_percent
    assert api_mod4.initial_val == approx(-25)
    assert api_mod4.stacking_mult is None
    assert api_mod4.applied_val == approx(-25)
    api_mod5 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector5.id).one()
    assert api_mod5.op == consts.ApiModOp.post_percent
    assert api_mod5.initial_val == approx(400)
    assert api_mod5.stacking_mult is None
    assert api_mod5.applied_val == approx(400)


def test_penalized(client, consts):
    (attr_val,
     attr_mods,
     api_item_affector1,
     api_item_affector2,
     api_item_affector3,
     api_item_affector4,
     api_item_affector5) = setup_penalization_test(client=client, consts=consts, stackable=False)
    # Verification
    assert attr_val == approx(67.5)
    assert len(attr_mods) == 5
    api_mod1 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one()
    assert api_mod1.op == consts.ApiModOp.post_percent
    assert api_mod1.initial_val == approx(20)
    assert api_mod1.stacking_mult is None
    assert api_mod1.applied_val == approx(20)
    api_mod2 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one()
    assert api_mod2.op == consts.ApiModOp.post_percent
    assert api_mod2.initial_val == approx(50)
    assert api_mod2.stacking_mult is None
    assert api_mod2.applied_val == approx(50)
    api_mod3 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector3.id).one()
    assert api_mod3.op == consts.ApiModOp.post_percent
    assert api_mod3.initial_val == approx(-90)
    assert api_mod3.stacking_mult is None
    assert api_mod3.applied_val == approx(-90)
    api_mod4 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector4.id).one()
    assert api_mod4.op == consts.ApiModOp.post_percent
    assert api_mod4.initial_val == approx(-25)
    assert api_mod4.stacking_mult is None
    assert api_mod4.applied_val == approx(-25)
    api_mod5 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector5.id).one()
    assert api_mod5.op == consts.ApiModOp.post_percent
    assert api_mod5.initial_val == approx(400)
    assert api_mod5.stacking_mult is None
    assert api_mod5.applied_val == approx(400)


def test_insignificant_base(client, consts):
    # When value on top of which modifications should be applied is 0, all percent modifications are
    # insignificant and are not exposed as modifications
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent_immune,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_id = client.mk_eve_item(attrs={eve_affector_attr_id: 300}, eff_ids=[eve_effect_id])
    eve_affectee_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_affector_id)
    api_affectee = api_fit.set_ship(type_id=eve_affectee_id)
    # Verification
    api_affectee.update()
    assert api_affectee.attrs[eve_affectee_attr_id].dogma == approx(0)
    with check_no_field():
        api_affectee.mods  # noqa: B018


def test_insignificant_modified_base(client, consts):
    # When value on top of which modifications should be applied is 0, all percent modifications are
    # insignificant and are not exposed as modifications
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect1_id = client.mk_eve_effect(mod_info=[eve_mod1])
    eve_affector1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0}, eff_ids=[eve_effect1_id])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent_immune,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect2_id = client.mk_eve_effect(mod_info=[eve_mod2])
    eve_affector2_id = client.mk_eve_item(attrs={eve_affector_attr_id: 300}, eff_ids=[eve_effect2_id])
    eve_affectee_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_affector1_id)
    api_fit.add_rig(type_id=eve_affector2_id)
    api_affectee = api_fit.set_ship(type_id=eve_affectee_id)
    # Verification
    api_affectee.update()
    assert api_affectee.attrs[eve_affectee_attr_id].dogma == approx(0)
    api_mod = api_affectee.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_mul
    assert api_mod.initial_val == approx(0)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(0)


def test_insignificant_earlier_ops(client, consts):
    # When a value is multiplied by 0, all earlier modifications are insignificant
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(stackable=False)
    # Pre-assignment
    pre_ass_val = 5
    eve_pre_ass_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_pre_ass_effect_id = client.mk_eve_effect(mod_info=[eve_pre_ass_mod])
    eve_pre_ass_affector_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: pre_ass_val},
        eff_ids=[eve_pre_ass_effect_id])
    # Pre-multiplication
    pre_mul_val = 50
    eve_pre_mul_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.pre_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_pre_mul_effect_id = client.mk_eve_effect(mod_info=[eve_pre_mul_mod])
    eve_pre_mul_affector_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: pre_mul_val},
        eff_ids=[eve_pre_mul_effect_id])
    # Pre-division
    pre_div_val = 0.5
    eve_pre_div_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.pre_div,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_pre_div_effect_id = client.mk_eve_effect(mod_info=[eve_pre_div_mod])
    eve_pre_div_affector_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: pre_div_val},
        eff_ids=[eve_pre_div_effect_id])
    # Addition
    mod_add_val = 10
    eve_mod_add_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_mod_add_effect_id = client.mk_eve_effect(mod_info=[eve_mod_add_mod])
    eve_mod_add_affector_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: mod_add_val},
        eff_ids=[eve_mod_add_effect_id])
    # Subtraction
    mod_sub_val = 63
    eve_mod_sub_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_sub,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_mod_sub_effect_id = client.mk_eve_effect(mod_info=[eve_mod_sub_mod])
    eve_mod_sub_affector_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: mod_sub_val},
        eff_ids=[eve_mod_sub_effect_id])
    # Post-multiplication
    post_mul_val = 1.35
    eve_post_mul_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_post_mul_effect_id = client.mk_eve_effect(mod_info=[eve_post_mul_mod])
    eve_post_mul_affector_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: post_mul_val},
        eff_ids=[eve_post_mul_effect_id])
    # Post-division
    post_div_val = 2.7
    eve_post_div_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_div,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_post_div_effect_id = client.mk_eve_effect(mod_info=[eve_post_div_mod])
    eve_post_div_affector_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: post_div_val},
        eff_ids=[eve_post_div_effect_id])
    # Post-percent
    post_perc_val = 20
    eve_post_perc_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_post_perc_effect_id = client.mk_eve_effect(mod_info=[eve_post_perc_mod])
    eve_post_perc_affector_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: post_perc_val},
        eff_ids=[eve_post_perc_effect_id])
    # Post-percent-immune
    post_perc_immune_val = -100
    eve_post_perc_immune_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent_immune,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_post_perc_immune_effect_id = client.mk_eve_effect(mod_info=[eve_post_perc_immune_mod])
    eve_post_perc_immune_affector_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: post_perc_immune_val},
        eff_ids=[eve_post_perc_immune_effect_id])
    eve_affectee_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_pre_ass_affector_id)
    api_fit.add_rig(type_id=eve_pre_mul_affector_id)
    api_fit.add_rig(type_id=eve_pre_div_affector_id)
    api_fit.add_rig(type_id=eve_mod_add_affector_id)
    api_fit.add_rig(type_id=eve_mod_sub_affector_id)
    api_fit.add_rig(type_id=eve_post_mul_affector_id)
    api_fit.add_rig(type_id=eve_post_div_affector_id)
    api_fit.add_rig(type_id=eve_post_perc_affector_id)
    api_fit.add_rig(type_id=eve_post_perc_immune_affector_id)
    api_affectee = api_fit.set_ship(type_id=eve_affectee_id)
    # Verification - post-percent-immune change by -100 should remove earlier modifiers, i.e. all
    # the modifiers but post-percent-immune itself in this case.
    api_affectee.update()
    expected_value = (
            (pre_ass_val * pre_mul_val / pre_div_val + mod_add_val - mod_sub_val) * post_mul_val
            / post_div_val * (1 + post_perc_val / 100) * (1 + post_perc_immune_val / 100))
    assert api_affectee.attrs[eve_affectee_attr_id].dogma == approx(expected_value)
    api_post_perc_immune_mod = api_affectee.mods[eve_affectee_attr_id].one()
    assert api_post_perc_immune_mod.op == consts.ApiModOp.post_percent
    assert api_post_perc_immune_mod.initial_val == approx(post_perc_immune_val)
    assert api_post_perc_immune_mod.stacking_mult is None
    assert api_post_perc_immune_mod.applied_val == approx(post_perc_immune_val)


def test_insignificant_op_collision(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_post_mul_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_mod_sub_effect_id = client.mk_eve_effect(mod_info=[eve_post_mul_mod])
    eve_mod_sub_affector_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 0},
        eff_ids=[eve_mod_sub_effect_id])
    eve_post_perc_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent_immune,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_post_perc_effect_id = client.mk_eve_effect(mod_info=[eve_post_perc_mod])
    eve_post_perc_affector_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -100},
        eff_ids=[eve_post_perc_effect_id])
    eve_affectee_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_mod_sub_affector_id)
    api_fit.add_rig(type_id=eve_post_perc_affector_id)
    api_affectee = api_fit.set_ship(type_id=eve_affectee_id)
    # Verification - when both sides of multiplication are 0, right side is preferred for fewer mods
    api_affectee.update()
    assert api_affectee.attrs[eve_affectee_attr_id].dogma == approx(0)
    api_post_perc_mod = api_affectee.mods[eve_affectee_attr_id].one()
    assert api_post_perc_mod.op == consts.ApiModOp.post_percent
    assert api_post_perc_mod.initial_val == approx(-100)
    assert api_post_perc_mod.stacking_mult is None
    assert api_post_perc_mod.applied_val == approx(-100)


def setup_insignificant_chain_values_test(*, client, consts, stackable):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(stackable=stackable)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent_immune,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector1_id = client.mk_eve_item(attrs={eve_affector_attr_id: -100}, eff_ids=[eve_effect_id])
    eve_affector2_id = client.mk_eve_item(attrs={eve_affector_attr_id: 300}, eff_ids=[eve_effect_id])
    eve_affector3_id = client.mk_eve_item(attrs={eve_affector_attr_id: -100}, eff_ids=[eve_effect_id])
    eve_affector4_id = client.mk_eve_item(attrs={eve_affector_attr_id: -60}, eff_ids=[eve_effect_id])
    eve_affectee_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affector1 = api_fit.add_rig(type_id=eve_affector1_id)
    api_fit.add_rig(type_id=eve_affector2_id)
    api_affector3 = api_fit.add_rig(type_id=eve_affector3_id)
    api_fit.add_rig(type_id=eve_affector4_id)
    api_affectee = api_fit.set_ship(type_id=eve_affectee_id)
    api_affectee.update()
    return (
        api_affectee.attrs[eve_affectee_attr_id].dogma,
        api_affectee.mods[eve_affectee_attr_id],
        api_affector1,
        api_affector3)


def test_insignificant_chain_values_non_penalized(client, consts):
    # When some values in chain result in final value of 0, only they should be exposed
    (attr_val,
     api_mods,
     api_affector1,
     api_affector3) = setup_insignificant_chain_values_test(client=client, consts=consts, stackable=True)
    # Verification
    assert attr_val == approx(0)
    # Without stacking penalty, all modifications which multiply by 0 are returned
    assert len(api_mods) == 2
    api_mod1 = api_mods.find_by_affector_item(affector_item_id=api_affector1.id).one()
    assert api_mod1.op == consts.ApiModOp.post_percent
    assert api_mod1.initial_val == approx(-100)
    assert api_mod1.stacking_mult is None
    assert api_mod1.applied_val == approx(-100)
    api_mod3 = api_mods.find_by_affector_item(affector_item_id=api_affector3.id).one()
    assert api_mod3.op == consts.ApiModOp.post_percent
    assert api_mod3.initial_val == approx(-100)
    assert api_mod3.stacking_mult is None
    assert api_mod3.applied_val == approx(-100)


def test_insignificant_chain_values_penalized(client, consts):
    # When some values in chain result in final value of 0, only they should be exposed
    (attr_val,
     api_mods,
     api_affector1,
     api_affector3) = setup_insignificant_chain_values_test(client=client, consts=consts, stackable=False)
    # Verification
    assert attr_val == approx(0)
    # Unlike regular post-percent, immune yields the same result for penalizable attributes too
    assert len(api_mods) == 2
    api_mod1 = api_mods.find_by_affector_item(affector_item_id=api_affector1.id).one()
    assert api_mod1.op == consts.ApiModOp.post_percent
    assert api_mod1.initial_val == approx(-100)
    assert api_mod1.stacking_mult is None
    assert api_mod1.applied_val == approx(-100)
    api_mod3 = api_mods.find_by_affector_item(affector_item_id=api_affector3.id).one()
    assert api_mod3.op == consts.ApiModOp.post_percent
    assert api_mod3.initial_val == approx(-100)
    assert api_mod3.stacking_mult is None
    assert api_mod3.applied_val == approx(-100)
