from tests import approx


def setup_hig_test(*, client, consts, high_is_good):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(stackable=False, high_is_good=high_is_good)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_affector1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 10}, eff_ids=[eve_effect_id])
    eve_item_affector2_id = client.mk_eve_item(attrs={eve_affector_attr_id: -20}, eff_ids=[eve_effect_id])
    eve_item_affector3_id = client.mk_eve_item(attrs={eve_affector_attr_id: 53.02}, eff_ids=[eve_effect_id])
    eve_item_affectee_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_item_affector1_id)
    api_item_affector2 = api_fit.add_rig(type_id=eve_item_affector2_id)
    api_item_affector3 = api_fit.add_rig(type_id=eve_item_affector3_id)
    api_item_affectee = api_fit.set_ship(type_id=eve_item_affectee_id)
    api_item_affectee.update()
    return (
        api_item_affectee.attrs[eve_affectee_attr_id].dogma,
        api_item_affectee.mods[eve_affectee_attr_id],
        api_item_affector2,
        api_item_affector3)


def test_high_is_good(client, consts):
    attr_val, attr_mods, _, api_item_affector3 = setup_hig_test(client=client, consts=consts, high_is_good=True)
    # Verification
    assert attr_val == approx(53.02)
    attr_mod = attr_mods.one()
    assert attr_mod.op == consts.ApiModOp.post_assign
    assert attr_mod.initial_val == approx(53.02)
    assert attr_mod.stacking_mult is None
    assert attr_mod.applied_val == approx(53.02)
    assert attr_mod.affectors.one().item_id == api_item_affector3.id


def test_high_is_bad(client, consts):
    attr_val, attr_mods, api_item_affector2, _ = setup_hig_test(client=client, consts=consts, high_is_good=False)
    # Verification
    assert attr_val == approx(-20)
    attr_mod = attr_mods.one()
    assert attr_mod.op == consts.ApiModOp.post_assign
    assert attr_mod.initial_val == approx(-20)
    assert attr_mod.stacking_mult is None
    assert attr_mod.applied_val == approx(-20)
    assert attr_mod.affectors.one().item_id == api_item_affector2.id


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
    post_perc_val = 15
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
    post_perc_immune_val = -20
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
    # Post-assignment
    post_ass_val = 68
    eve_post_ass_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_post_ass_effect_id = client.mk_eve_effect(mod_info=[eve_post_ass_mod])
    eve_post_ass_affector_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: post_ass_val},
        eff_ids=[eve_post_ass_effect_id])
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
    api_fit.add_rig(type_id=eve_post_ass_affector_id)
    api_affectee = api_fit.set_ship(type_id=eve_affectee_id)
    # Verification
    api_affectee.update()
    assert api_affectee.attrs[eve_affectee_attr_id].dogma == approx(post_ass_val)
    api_post_assign_mod = api_affectee.mods[eve_affectee_attr_id].one()
    assert api_post_assign_mod.op == consts.ApiModOp.post_assign
    assert api_post_assign_mod.initial_val == approx(post_ass_val)
    assert api_post_assign_mod.stacking_mult is None
    assert api_post_assign_mod.applied_val == approx(post_ass_val)
