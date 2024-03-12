from pytest import approx


def test_state_offline(client, consts):
    # Offline/passive effects are not run only due to effect fitting usage chance attribute
    # specified
    eve_chance_attr = client.mk_eve_attr()
    eve_src_attr1 = client.mk_eve_attr()
    eve_src_attr2 = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr1.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect1 = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr2.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect2 = client.mk_eve_effect(
        cat_id=consts.EveEffCat.passive,
        chance_attr_id=eve_chance_attr.id,
        mod_info=[eve_mod2])
    eve_item = client.mk_eve_item(
        attrs={eve_chance_attr.id: 0.2, eve_src_attr1.id: 20, eve_src_attr2.id: 30, eve_tgt_attr.id: 100},
        eff_ids=[eve_effect1.id, eve_effect2.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item.id, state=consts.ApiState.ghost)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_item.effects[eve_effect1.id] == (False, consts.ApiEffMode.full_compliance)
    assert api_item.effects[eve_effect2.id] == (False, consts.ApiEffMode.full_compliance)
    # Action
    api_item.change_mod(state=consts.ApiState.offline)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(120)
    assert api_item.effects[eve_effect1.id] == (True, consts.ApiEffMode.full_compliance)
    assert api_item.effects[eve_effect2.id] == (False, consts.ApiEffMode.full_compliance)
    # Action
    api_item.change_mod(state=consts.ApiState.online)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(120)
    assert api_item.effects[eve_effect1.id] == (True, consts.ApiEffMode.full_compliance)
    assert api_item.effects[eve_effect2.id] == (False, consts.ApiEffMode.full_compliance)
    # Action
    api_item.change_mod(state=consts.ApiState.active)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(120)
    assert api_item.effects[eve_effect1.id] == (True, consts.ApiEffMode.full_compliance)
    assert api_item.effects[eve_effect2.id] == (False, consts.ApiEffMode.full_compliance)
    # Action
    api_item.change_mod(state=consts.ApiState.overload)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(120)
    assert api_item.effects[eve_effect1.id] == (True, consts.ApiEffMode.full_compliance)
    assert api_item.effects[eve_effect2.id] == (False, consts.ApiEffMode.full_compliance)


def test_state_online_running(client, consts):
    # Online effects should be run only when item has running "online" effect
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_online_effect = client.mk_eve_online_effect()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.online, mod_info=[eve_mod])
    eve_item = client.mk_eve_item(
        attrs={eve_src_attr.id: 20, eve_tgt_attr.id: 100},
        eff_ids=[eve_online_effect.id, eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item.id, state=consts.ApiState.offline)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_item.effects[eve_online_effect.id] == (False, consts.ApiEffMode.full_compliance)
    assert api_item.effects[eve_effect.id] == (False, consts.ApiEffMode.full_compliance)
    # Action
    api_item.change_mod(state=consts.ApiState.online)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(120)
    assert api_item.effects[eve_online_effect.id] == (True, consts.ApiEffMode.full_compliance)
    assert api_item.effects[eve_effect.id] == (True, consts.ApiEffMode.full_compliance)
    # Action
    api_item.change_mod(effect_modes={eve_online_effect.id: consts.ApiEffMode.force_stop})
    # Verification
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_item.effects[eve_online_effect.id] == (False, consts.ApiEffMode.force_stop)
    assert api_item.effects[eve_effect.id] == (False, consts.ApiEffMode.full_compliance)
    # Action - effects from online category rely only on actual "online" effect, ignoring everything
    # else
    api_item.change_mod(state=consts.ApiState.offline, effect_modes={eve_online_effect.id: consts.ApiEffMode.force_run})
    # Verification
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(120)
    assert api_item.effects[eve_online_effect.id] == (True, consts.ApiEffMode.force_run)
    assert api_item.effects[eve_effect.id] == (True, consts.ApiEffMode.full_compliance)


def test_state_online_absent(client, consts):
    # No online effect - other effects from online category are not running
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.online, mod_info=[eve_mod])
    eve_item = client.mk_eve_item(attrs={eve_src_attr.id: 20, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item.id, state=consts.ApiState.online)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_item.effects[eve_effect.id] == (False, consts.ApiEffMode.full_compliance)
    # Action
    api_item.change_mod(state=consts.ApiState.overload)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_item.effects[eve_effect.id] == (False, consts.ApiEffMode.full_compliance)


def test_state_active_default(client, consts):
    # Default active effect is run, non-default is not
    eve_src_attr1 = client.mk_eve_attr()
    eve_src_attr2 = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr1.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect1 = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr2.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect2 = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod2])
    eve_item = client.mk_eve_item(
        attrs={eve_src_attr1.id: 20, eve_src_attr2.id: 30, eve_tgt_attr.id: 100},
        eff_ids=[eve_effect1.id, eve_effect2.id],
        defeff_id=eve_effect1.id)
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item.id, state=consts.ApiState.online)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_item.effects[eve_effect1.id] == (False, consts.ApiEffMode.full_compliance)
    assert api_item.effects[eve_effect2.id] == (False, consts.ApiEffMode.full_compliance)
    # Action
    api_item.change_mod(state=consts.ApiState.active)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(120)
    assert api_item.effects[eve_effect1.id] == (True, consts.ApiEffMode.full_compliance)
    assert api_item.effects[eve_effect2.id] == (False, consts.ApiEffMode.full_compliance)
    # Action
    api_item.change_mod(state=consts.ApiState.overload)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(120)
    assert api_item.effects[eve_effect1.id] == (True, consts.ApiEffMode.full_compliance)
    assert api_item.effects[eve_effect2.id] == (False, consts.ApiEffMode.full_compliance)


def test_state_active_absent(client, consts):
    # No default - nothing is running
    eve_src_attr1 = client.mk_eve_attr()
    eve_src_attr2 = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr1.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect1 = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr2.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect2 = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod2])
    eve_item = client.mk_eve_item(
        attrs={eve_src_attr1.id: 20, eve_src_attr2.id: 30, eve_tgt_attr.id: 100},
        eff_ids=[eve_effect1.id, eve_effect2.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item.id, state=consts.ApiState.online)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_item.effects[eve_effect1.id] == (False, consts.ApiEffMode.full_compliance)
    assert api_item.effects[eve_effect2.id] == (False, consts.ApiEffMode.full_compliance)
    # Action
    api_item.change_mod(state=consts.ApiState.active)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_item.effects[eve_effect1.id] == (False, consts.ApiEffMode.full_compliance)
    assert api_item.effects[eve_effect2.id] == (False, consts.ApiEffMode.full_compliance)
    # Action
    api_item.change_mod(state=consts.ApiState.overload)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_item.effects[eve_effect1.id] == (False, consts.ApiEffMode.full_compliance)
    assert api_item.effects[eve_effect2.id] == (False, consts.ApiEffMode.full_compliance)


def test_state_overload(client, consts):
    # Overload just needs item to be overloaded and nothing else
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.overload, mod_info=[eve_mod])
    eve_item = client.mk_eve_item(attrs={eve_src_attr.id: 20, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item.id, state=consts.ApiState.online)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_item.effects[eve_effect.id] == (False, consts.ApiEffMode.full_compliance)
    # Action
    api_item.change_mod(state=consts.ApiState.overload)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(120)
    assert api_item.effects[eve_effect.id] == (True, consts.ApiEffMode.full_compliance)
