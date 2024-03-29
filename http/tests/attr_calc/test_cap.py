from pytest import approx


def test_default(client, consts):
    # Check that cap is applied properly when item doesn't have base value of capping attribute
    eve_capping_attr = client.mk_eve_attr(def_val=5)
    eve_capped_attr = client.mk_eve_attr(max_attr_id=eve_capping_attr.id)
    eve_src_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_capped_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item = client.mk_eve_item(
        attrs={eve_capped_attr.id: 3, eve_src_attr.id: 6},
        eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item.id)
    # Verification - should be 3 * 6 = 18 without cap, but 5 with cap
    api_item.update()
    assert api_item.attrs[eve_capped_attr.id].dogma == approx(5)
    api_mod = api_item.mods.find_by_src_attr(tgt_attr_id=eve_capped_attr.id, src_attr_id=eve_capping_attr.id).one()
    assert api_mod.val == approx(5)
    assert api_mod.op == consts.ApiModOp.limit
    assert api_mod.penalized is False
    assert api_mod.aggr_mode == consts.ApiAggrMode.stack
    assert api_mod.src.one().item_id == api_item.id
    assert api_mod.src.one().attr_id == eve_capping_attr.id


def test_unmodified(client, consts):
    # Check that cap is applied properly when item defines its value
    eve_capping_attr = client.mk_eve_attr(def_val=5)
    eve_capped_attr = client.mk_eve_attr(max_attr_id=eve_capping_attr.id)
    eve_src_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_capped_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item = client.mk_eve_item(
        attrs={eve_capping_attr.id: 2, eve_capped_attr.id: 3, eve_src_attr.id: 6},
        eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item.id)
    # Verification - should be 3 * 6 = 18 without cap, but 2 with cap
    api_item.update()
    assert api_item.attrs[eve_capped_attr.id].dogma == approx(2)
    api_mod = api_item.mods.find_by_src_attr(tgt_attr_id=eve_capped_attr.id, src_attr_id=eve_capping_attr.id).one()
    assert api_mod.val == approx(2)
    assert api_mod.op == consts.ApiModOp.limit
    assert api_mod.penalized is False
    assert api_mod.aggr_mode == consts.ApiAggrMode.stack
    assert api_mod.src.one().item_id == api_item.id
    assert api_mod.src.one().attr_id == eve_capping_attr.id


def test_modified(client, consts):
    # Check that cap is applied properly when item defines its value, and it's modified further
    eve_capping_attr = client.mk_eve_attr()
    eve_capped_attr = client.mk_eve_attr(max_attr_id=eve_capping_attr.id)
    eve_src_attr = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_capped_attr.id)
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_capping_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod1, eve_mod2])
    eve_item = client.mk_eve_item(
        attrs={eve_capping_attr.id: 0.1, eve_capped_attr.id: 3, eve_src_attr.id: 6},
        eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item.id)
    # Verification - should be 3 * 6 = 18 without cap, but 0.1 * 6 = 0.6 with cap
    api_item.update()
    assert api_item.attrs[eve_capped_attr.id].dogma == approx(0.6)
    api_mod = api_item.mods.find_by_src_attr(tgt_attr_id=eve_capped_attr.id, src_attr_id=eve_capping_attr.id).one()
    assert api_mod.val == approx(0.6)
    assert api_mod.op == consts.ApiModOp.limit
    assert api_mod.penalized is False
    assert api_mod.aggr_mode == consts.ApiAggrMode.stack
    assert api_mod.src.one().item_id == api_item.id
    assert api_mod.src.one().attr_id == eve_capping_attr.id


def test_update(client, consts):
    # Make sure that when value of capping attribute changes, values which depend on it are updated
    eve_capping_attr = client.mk_eve_attr()
    eve_capped_attr = client.mk_eve_attr(max_attr_id=eve_capping_attr.id)
    eve_src_attr = client.mk_eve_attr()
    eve_capped_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_capped_attr.id)
    eve_capped_effect = client.mk_eve_effect(mod_info=[eve_capped_mod])
    eve_capped_item = client.mk_eve_item(
        attrs={eve_capping_attr.id: 2, eve_capped_attr.id: 3, eve_src_attr.id: 6},
        eff_ids=[eve_capped_effect.id])
    eve_capping_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_capping_attr.id)
    eve_capping_effect = client.mk_eve_effect(mod_info=[eve_capping_mod])
    eve_capping_item = client.mk_eve_item(attrs={eve_src_attr.id: 3.5}, eff_ids=[eve_capping_effect.id])
    eve_ship_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.set_ship(type_id=eve_ship_item.id)
    api_capped_item = api_fit.add_rig(type_id=eve_capped_item.id)
    # Verification - request capped attribute value before adding capping item, to make sure capping
    # attribute value is calculated
    api_capped_item.update()
    assert api_capped_item.attrs[eve_capped_attr.id].dogma == approx(2)
    assert api_capped_item.mods.find_by_src_attr(
        tgt_attr_id=eve_capped_attr.id, src_attr_id=eve_capping_attr.id).one().val == approx(2)
    # Action
    api_capping_item = api_fit.add_implant(type_id=eve_capping_item.id)
    # Verification - here, capping attribute should be multiplied by 3.5 (2 * 3.5 = 7), which is
    # still below uncapped value of capped attribute (18)
    api_capped_item.update()
    assert api_capped_item.attrs[eve_capped_attr.id].dogma == approx(7)
    assert api_capped_item.mods.find_by_src_attr(
        tgt_attr_id=eve_capped_attr.id, src_attr_id=eve_capping_attr.id).one().val == approx(7)
    # Action
    api_capping_item.remove()
    # Verification - should revert back to base value after change of capping attribute
    api_capped_item.update()
    assert api_capped_item.attrs[eve_capped_attr.id].dogma == approx(2)
    assert api_capped_item.mods.find_by_src_attr(
        tgt_attr_id=eve_capped_attr.id, src_attr_id=eve_capping_attr.id).one().val == approx(2)


def test_uncapped(client, consts):
    # Check which modification data is exposed when calculated value is below cap
    eve_capping_attr = client.mk_eve_attr(def_val=5)
    eve_capped_attr = client.mk_eve_attr(max_attr_id=eve_capping_attr.id)
    eve_src_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_capped_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item = client.mk_eve_item(
        attrs={eve_capping_attr.id: 20, eve_capped_attr.id: 3, eve_src_attr.id: 6},
        eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item.id)
    # Verification - should be 3 * 6 = 18 without cap, and cap is higher - 20, so 18 is exposed
    api_item.update()
    assert api_item.attrs[eve_capped_attr.id].dogma == approx(18)
    assert len(api_item.mods.find_by_src_attr(tgt_attr_id=eve_capped_attr.id, src_attr_id=eve_capping_attr.id)) == 0
