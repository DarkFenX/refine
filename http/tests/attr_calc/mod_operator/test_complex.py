from tests import approx


def test_almost_all_in(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(stackable=False)
    # Pre-assignment
    pre_ass_val = 5
    eve_pre_ass_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
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
        dom=consts.EveModDom.ship,
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
        dom=consts.EveModDom.ship,
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
        dom=consts.EveModDom.ship,
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
        dom=consts.EveModDom.ship,
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
        dom=consts.EveModDom.ship,
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
        dom=consts.EveModDom.ship,
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
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_post_perc_effect_id = client.mk_eve_effect(mod_info=[eve_post_perc_mod])
    eve_post_perc_affector_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: post_perc_val},
        eff_ids=[eve_post_perc_effect_id])
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
    api_affectee = api_fit.set_ship(type_id=eve_affectee_id)
    # Verification
    api_affectee.update()
    expected_value = (
            (pre_ass_val * pre_mul_val / pre_div_val + mod_add_val - mod_sub_val)
            * post_mul_val / post_div_val * (1 + post_perc_val / 100))
    assert api_affectee.attrs[eve_affectee_attr_id].dogma == approx(expected_value)
    api_mods = api_affectee.mods[eve_affectee_attr_id]
    api_pre_assign_mod = api_mods.find_by_op(op=consts.ApiModOp.pre_assign).one()
    assert api_pre_assign_mod.initial_val == approx(pre_ass_val)
    assert api_pre_assign_mod.stacking_mult is None
    assert api_pre_assign_mod.applied_val == approx(pre_ass_val)
    api_pre_mul_mod = api_mods.find_by_op(op=consts.ApiModOp.pre_mul).one()
    assert api_pre_mul_mod.initial_val == approx(pre_mul_val)
    assert api_pre_mul_mod.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_pre_mul_mod.applied_val == approx(pre_mul_val)
    api_pre_div_mod = api_mods.find_by_op(op=consts.ApiModOp.pre_div).one()
    assert api_pre_div_mod.initial_val == approx(pre_div_val)
    assert api_pre_div_mod.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_pre_div_mod.applied_val == approx(pre_div_val)
    api_add_mod = api_mods.find_by_op(op=consts.ApiModOp.mod_add).one()
    assert api_add_mod.initial_val == approx(mod_add_val)
    assert api_add_mod.stacking_mult is None
    assert api_add_mod.applied_val == approx(mod_add_val)
    api_sub_mod = api_mods.find_by_op(op=consts.ApiModOp.mod_sub).one()
    assert api_sub_mod.initial_val == approx(mod_sub_val)
    assert api_sub_mod.stacking_mult is None
    assert api_sub_mod.applied_val == approx(mod_sub_val)
    api_post_mul_mod = api_mods.find_by_op(op=consts.ApiModOp.post_mul).one()
    assert api_post_mul_mod.initial_val == approx(post_mul_val)
    assert api_post_mul_mod.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_post_mul_mod.applied_val == approx(post_mul_val)
    api_post_div_mod = api_mods.find_by_op(op=consts.ApiModOp.post_div).one()
    assert api_post_div_mod.initial_val == approx(post_div_val)
    assert api_post_div_mod.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_post_div_mod.applied_val == approx(post_div_val)
    api_post_perc_mod = api_mods.find_by_op(op=consts.ApiModOp.post_percent).one()
    assert api_post_perc_mod.initial_val == approx(post_perc_val)
    assert api_post_perc_mod.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_post_perc_mod.applied_val == approx(post_perc_val)
