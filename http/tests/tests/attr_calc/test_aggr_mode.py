# We test just 4 operation types, since CCP doesn't use any other in buffs at the present time

from tests import approx


def test_add_max(client, consts):
    # Setting HiG here just to check that aggregation mode takes precedence
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value, high_is_good=False)
    eve_affectee_attr_id = client.mk_eve_attr(high_is_good=False)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.mod_add,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect1_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: -40},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_sw_effect2_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 30},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_sw_effect1_id)
    api_sol.add_sw_effect(type_id=eve_sw_effect2_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr_id].dogma == approx(180)
    api_mod = api_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.mod_add
    assert api_mod.initial_val == approx(30)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(30)


def test_add_min(client, consts):
    # Setting HiG here just to check that aggregation mode takes precedence
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value, high_is_good=True)
    eve_affectee_attr_id = client.mk_eve_attr(high_is_good=True)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.mod_add,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect1_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: -40},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_sw_effect2_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 30},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_sw_effect1_id)
    api_sol.add_sw_effect(type_id=eve_sw_effect2_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr_id].dogma == approx(110)
    api_mod = api_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.mod_add
    assert api_mod.initial_val == approx(-40)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(-40)


def test_postmul_max(client, consts):
    # Setting HiG here just to check that aggregation mode takes precedence
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value, high_is_good=False)
    eve_affectee_attr_id = client.mk_eve_attr(high_is_good=False)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect1_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 0.6},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_sw_effect2_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 1.3},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_sw_effect1_id)
    api_sol.add_sw_effect(type_id=eve_sw_effect2_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr_id].dogma == approx(195)
    api_mod = api_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_mul
    assert api_mod.initial_val == approx(1.3)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(1.3)


def test_postmul_min(client, consts):
    # Setting HiG here just to check that aggregation mode takes precedence
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value, high_is_good=True)
    eve_affectee_attr_id = client.mk_eve_attr(high_is_good=True)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect1_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 0.6},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_sw_effect2_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 1.3},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_sw_effect1_id)
    api_sol.add_sw_effect(type_id=eve_sw_effect2_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr_id].dogma == approx(90)
    api_mod = api_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_mul
    assert api_mod.initial_val == approx(0.6)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(0.6)


def test_postperc_max(client, consts):
    # Setting HiG here just to check that aggregation mode takes precedence
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value, high_is_good=False)
    eve_affectee_attr_id = client.mk_eve_attr(high_is_good=False)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect1_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: -40},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_sw_effect2_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 30},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_sw_effect1_id)
    api_sol.add_sw_effect(type_id=eve_sw_effect2_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr_id].dogma == approx(195)
    api_mod = api_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(30)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(30)


def test_postperc_min(client, consts):
    # Setting HiG here just to check that aggregation mode takes precedence
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value, high_is_good=True)
    eve_affectee_attr_id = client.mk_eve_attr(high_is_good=True)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect1_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: -40},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_sw_effect2_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 30},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_sw_effect1_id)
    api_sol.add_sw_effect(type_id=eve_sw_effect2_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr_id].dogma == approx(90)
    api_mod = api_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(-40)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(-40)


def test_postassign_max(client, consts):
    # Setting HiG here just to check that aggregation mode takes precedence
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value, high_is_good=False)
    eve_affectee_attr_id = client.mk_eve_attr(high_is_good=False)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_assign,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect1_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: -40},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_sw_effect2_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 30},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_sw_effect1_id)
    api_sol.add_sw_effect(type_id=eve_sw_effect2_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr_id].dogma == approx(30)
    api_mod = api_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_assign
    assert api_mod.initial_val == approx(30)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(30)


def test_postassign_min(client, consts):
    # Setting HiG here just to check that aggregation mode takes precedence
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value, high_is_good=True)
    eve_affectee_attr_id = client.mk_eve_attr(high_is_good=True)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_assign,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect1_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: -40},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_sw_effect2_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 30},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_sw_effect1_id)
    api_sol.add_sw_effect(type_id=eve_sw_effect2_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr_id].dogma == approx(-40)
    api_mod = api_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_assign
    assert api_mod.initial_val == approx(-40)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(-40)


def test_different_buffs(client, consts):
    # Different buffs should stack instead of overriding each other
    eve_buff_type_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_type_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_2_id)
    eve_buff_val_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_2_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff1_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_buff2_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect_id = client.mk_eve_item(
        attrs={
            eve_buff_type_attr1_id: eve_buff1_id, eve_buff_val_attr1_id: -40,
            eve_buff_type_attr2_id: eve_buff2_id, eve_buff_val_attr2_id: 30},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr_id].dogma == approx(117)
    api_mods = api_ship.mods[eve_affectee_attr_id]
    assert len(api_mods) == 2
    api_mod1 = api_mods.find_by_affector_attr(affector_attr_id=eve_buff_val_attr1_id).one()
    assert api_mod1.op == consts.ApiModOp.post_percent
    assert api_mod1.initial_val == approx(-40)
    assert api_mod1.stacking_mult is None
    assert api_mod1.applied_val == approx(-40)
    api_mod2 = api_mods.find_by_affector_attr(affector_attr_id=eve_buff_val_attr2_id).one()
    assert api_mod2.op == consts.ApiModOp.post_percent
    assert api_mod2.initial_val == approx(30)
    assert api_mod2.stacking_mult is None
    assert api_mod2.applied_val == approx(30)


def test_different_sources(client, consts):
    # Same buff type from different sources shouldn't stack
    eve_buff_val_mult_attr_id = client.mk_eve_attr()
    eve_buff_type_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_type_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_2_id)
    eve_buff_val_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_2_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr1_id: eve_buff_id, eve_buff_val_attr1_id: 4.7},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_buff_val_attr2_id: 1.25},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_charge_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_buff_type_attr2_id,
        affectee_attr_id=eve_buff_type_attr2_id)
    eve_charge_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_buff_val_mult_attr_id,
        affectee_attr_id=eve_buff_val_attr2_id)
    eve_charge_effect_id = client.mk_eve_effect(mod_info=[eve_charge_mod1, eve_charge_mod2])
    eve_charge_id = client.mk_eve_item(
        attrs={eve_buff_type_attr2_id: eve_buff_id, eve_buff_val_mult_attr_id: 4},
        eff_ids=[eve_charge_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_mod(
        type_id=eve_module_id,
        charge_type_id=eve_charge_id,
        state=consts.ApiModuleState.active)
    # Aggregation mode is set to max, and fleet buff value is higher (1.25*4 = 5 vs 4.7), so only
    # fleet buff is applied
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr_id].dogma == approx(750)
    api_mod = api_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_mul
    assert api_mod.initial_val == approx(5)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(5)
    assert api_mod.affectors.one().item_id == api_module.id
    assert api_mod.affectors.one().attr_id == eve_buff_val_attr2_id
