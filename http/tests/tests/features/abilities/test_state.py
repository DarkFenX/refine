from fw import approx


def test_switch_state_local(client, consts):
    # Check that local abilities are applied only when they are enabled, and that they apply
    # regardless of fighter state
    eve_affector_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_ab_speed_bonus)
    eve_affectee_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_affector_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_mjd_sig_radius_bonus)
    eve_affectee_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_primary_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_ab, cat_id=consts.EveEffCat.active)
    eve_secondary_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_mjd, cat_id=consts.EveEffCat.active)
    eve_primary_abil_id = client.mk_eve_abil(id_=consts.EveAbil.ab)
    eve_secondary_abil_id = client.mk_eve_abil(id_=consts.EveAbil.mjd)
    eve_fighter_id = client.mk_eve_fighter(
        attrs={
            eve_affector_attr1_id: 400, eve_affectee_attr1_id: 1017.5,
            eve_affector_attr2_id: 150, eve_affectee_attr2_id: 100},
        eff_ids=[eve_primary_effect_id, eve_secondary_effect_id],
        defeff_id=eve_primary_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_primary_abil_id), client.mk_eve_item_abil(id_=eve_secondary_abil_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_fighter.update()
    assert len(api_fighter.abilities) == 2
    assert api_fighter.abilities[eve_primary_abil_id].state is True
    assert api_fighter.abilities[eve_secondary_abil_id].state is False
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(5087.5)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(100)
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.in_space)
    # Verification
    api_fighter.update()
    assert len(api_fighter.abilities) == 2
    assert api_fighter.abilities[eve_primary_abil_id].state is True
    assert api_fighter.abilities[eve_secondary_abil_id].state is False
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(5087.5)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(100)
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.engaging)
    # Verification
    api_fighter.update()
    assert len(api_fighter.abilities) == 2
    assert api_fighter.abilities[eve_primary_abil_id].state is True
    assert api_fighter.abilities[eve_secondary_abil_id].state is False
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(5087.5)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(100)
    # Action
    api_fighter.change_fighter(abilities={eve_primary_abil_id: False, eve_secondary_abil_id: True})
    # Verification
    api_fighter.update()
    assert len(api_fighter.abilities) == 2
    assert api_fighter.abilities[eve_primary_abil_id].state is False
    assert api_fighter.abilities[eve_secondary_abil_id].state is True
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(1017.5)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(250)
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.in_space)
    # Verification
    api_fighter.update()
    assert len(api_fighter.abilities) == 2
    assert api_fighter.abilities[eve_primary_abil_id].state is False
    assert api_fighter.abilities[eve_secondary_abil_id].state is True
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(1017.5)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(250)
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.in_bay)
    # Verification
    api_fighter.update()
    assert len(api_fighter.abilities) == 2
    assert api_fighter.abilities[eve_primary_abil_id].state is False
    assert api_fighter.abilities[eve_secondary_abil_id].state is True
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(1017.5)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(250)
    # Action
    api_fighter.change_fighter(abilities={eve_primary_abil_id: True, eve_secondary_abil_id: False})
    # Verification
    api_fighter.update()
    assert len(api_fighter.abilities) == 2
    assert api_fighter.abilities[eve_primary_abil_id].state is True
    assert api_fighter.abilities[eve_secondary_abil_id].state is False
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(5087.5)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(100)
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.in_space)
    # Verification
    api_fighter.update()
    assert len(api_fighter.abilities) == 2
    assert api_fighter.abilities[eve_primary_abil_id].state is True
    assert api_fighter.abilities[eve_secondary_abil_id].state is False
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(5087.5)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(100)
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.engaging)
    # Verification
    api_fighter.update()
    assert len(api_fighter.abilities) == 2
    assert api_fighter.abilities[eve_primary_abil_id].state is True
    assert api_fighter.abilities[eve_secondary_abil_id].state is False
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(5087.5)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(100)


def test_switch_state_projected(client, consts):
    # Projected abilities are applied only when they are enabled + fighter is engaging
    eve_affector_attr1_id = client.mk_eve_attr()
    eve_affector_attr2_id = client.mk_eve_attr()
    eve_affectee_attr1_id = client.mk_eve_attr()
    eve_affectee_attr2_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr1_id,
        affectee_attr_id=eve_affectee_attr1_id)
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr2_id,
        affectee_attr_id=eve_affectee_attr2_id)
    eve_primary_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_attack_m,
        cat_id=consts.EveEffCat.target,
        mod_info=[eve_mod1])
    eve_secondary_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_missiles,
        cat_id=consts.EveEffCat.target,
        mod_info=[eve_mod2])
    eve_primary_abil_id = client.mk_eve_abil(id_=consts.EveAbil.pulse_cannon)
    eve_secondary_abil_id = client.mk_eve_abil(id_=consts.EveAbil.heavy_rocket_salvo)
    eve_fighter_id = client.mk_eve_fighter(
        attrs={eve_affector_attr1_id: 20, eve_affector_attr2_id: 30},
        eff_ids=[eve_primary_effect_id, eve_secondary_effect_id],
        defeff_id=eve_primary_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_primary_abil_id), client.mk_eve_item_abil(id_=eve_secondary_abil_id)])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr1_id: 100, eve_affectee_attr2_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fighter = api_affector_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_bay)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_affector_fighter.change_fighter(add_projs=[api_affectee_ship.id])
    # Verification
    api_affector_fighter.update()
    assert len(api_affector_fighter.abilities) == 2
    assert api_affector_fighter.abilities[eve_primary_abil_id].state is True
    assert api_affector_fighter.abilities[eve_secondary_abil_id].state is False
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].modified == approx(100)
    # Action
    api_affector_fighter.change_fighter(state=consts.ApiMinionState.in_space)
    # Verification
    api_affector_fighter.update()
    assert len(api_affector_fighter.abilities) == 2
    assert api_affector_fighter.abilities[eve_primary_abil_id].state is True
    assert api_affector_fighter.abilities[eve_secondary_abil_id].state is False
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].modified == approx(100)
    # Action
    api_affector_fighter.change_fighter(state=consts.ApiMinionState.engaging)
    # Verification
    api_affector_fighter.update()
    assert len(api_affector_fighter.abilities) == 2
    assert api_affector_fighter.abilities[eve_primary_abil_id].state is True
    assert api_affector_fighter.abilities[eve_secondary_abil_id].state is False
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].modified == approx(120)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].modified == approx(100)
    # Action
    api_affector_fighter.change_fighter(abilities={eve_primary_abil_id: False, eve_secondary_abil_id: True})
    # Verification
    api_affector_fighter.update()
    assert len(api_affector_fighter.abilities) == 2
    assert api_affector_fighter.abilities[eve_primary_abil_id].state is False
    assert api_affector_fighter.abilities[eve_secondary_abil_id].state is True
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].modified == approx(130)
    # Action
    api_affector_fighter.change_fighter(state=consts.ApiMinionState.in_space)
    # Verification
    api_affector_fighter.update()
    assert len(api_affector_fighter.abilities) == 2
    assert api_affector_fighter.abilities[eve_primary_abil_id].state is False
    assert api_affector_fighter.abilities[eve_secondary_abil_id].state is True
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].modified == approx(100)
    # Action
    api_affector_fighter.change_fighter(state=consts.ApiMinionState.in_bay)
    # Verification
    api_affector_fighter.update()
    assert len(api_affector_fighter.abilities) == 2
    assert api_affector_fighter.abilities[eve_primary_abil_id].state is False
    assert api_affector_fighter.abilities[eve_secondary_abil_id].state is True
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].modified == approx(100)
    # Action
    api_affector_fighter.change_fighter(abilities={eve_primary_abil_id: True, eve_secondary_abil_id: False})
    # Verification
    api_affector_fighter.update()
    assert len(api_affector_fighter.abilities) == 2
    assert api_affector_fighter.abilities[eve_primary_abil_id].state is True
    assert api_affector_fighter.abilities[eve_secondary_abil_id].state is False
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].modified == approx(100)
    # Action
    api_affector_fighter.change_fighter(state=consts.ApiMinionState.in_space)
    # Verification
    api_affector_fighter.update()
    assert len(api_affector_fighter.abilities) == 2
    assert api_affector_fighter.abilities[eve_primary_abil_id].state is True
    assert api_affector_fighter.abilities[eve_secondary_abil_id].state is False
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].modified == approx(100)
    # Action
    api_affector_fighter.change_fighter(state=consts.ApiMinionState.engaging)
    # Verification
    api_affector_fighter.update()
    assert len(api_affector_fighter.abilities) == 2
    assert api_affector_fighter.abilities[eve_primary_abil_id].state is True
    assert api_affector_fighter.abilities[eve_secondary_abil_id].state is False
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].modified == approx(120)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].modified == approx(100)


def test_src_switch_no_abil_data(client, consts):
    # Case when ability is defined, but is not attached to a fighter
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_affector_attr1_id = client.mk_eve_attr(datas=[eve_d1, eve_d2], id_=consts.EveAttr.ftr_abil_ab_speed_bonus)
    eve_affectee_attr1_id = client.mk_eve_attr(datas=[eve_d1, eve_d2], id_=consts.EveAttr.max_velocity)
    eve_affector_attr2_id = client.mk_eve_attr(datas=[eve_d1, eve_d2], id_=consts.EveAttr.ftr_abil_mjd_sig_radius_bonus)
    eve_affectee_attr2_id = client.mk_eve_attr(datas=[eve_d1, eve_d2], id_=consts.EveAttr.sig_radius)
    eve_primary_effect_id = client.mk_eve_effect(
        datas=[eve_d1, eve_d2], id_=consts.EveEffect.ftr_abil_ab, cat_id=consts.EveEffCat.active)
    eve_secondary_effect_id = client.mk_eve_effect(
        datas=[eve_d1, eve_d2], id_=consts.EveEffect.ftr_abil_mjd, cat_id=consts.EveEffCat.active)
    eve_primary_abil_id = client.mk_eve_abil(datas=[eve_d1, eve_d2], id_=consts.EveAbil.ab)
    eve_secondary_abil_id = client.mk_eve_abil(datas=[eve_d1, eve_d2], id_=consts.EveAbil.mjd)
    eve_fighter_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_fighter(
        datas=[eve_d1],
        id_=eve_fighter_id,
        attrs={
            eve_affector_attr1_id: 400, eve_affectee_attr1_id: 1017.5,
            eve_affector_attr2_id: 150, eve_affectee_attr2_id: 100},
        eff_ids=[eve_primary_effect_id, eve_secondary_effect_id],
        defeff_id=eve_primary_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_primary_abil_id)])
    client.mk_eve_fighter(
        datas=[eve_d2],
        id_=eve_fighter_id,
        attrs={
            eve_affector_attr1_id: 400, eve_affectee_attr1_id: 2035,
            eve_affector_attr2_id: 150, eve_affectee_attr2_id: 200},
        eff_ids=[eve_primary_effect_id, eve_secondary_effect_id],
        defeff_id=eve_primary_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_primary_abil_id), client.mk_eve_item_abil(id_=eve_secondary_abil_id)])
    # Another fighter, just to keep secondary ability from being cleaned up in first source
    client.mk_eve_fighter(
        datas=[eve_d1],
        eff_ids=[eve_secondary_effect_id],
        abils=[client.mk_eve_item_abil(id_=eve_secondary_abil_id)])
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fighter.update()
    assert len(api_fighter.abilities) == 1
    assert api_fighter.abilities[eve_primary_abil_id].state is True
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(5087.5)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(100)
    # Action
    api_fighter.change_fighter(abilities={eve_primary_abil_id: False, eve_secondary_abil_id: True})
    # Verification - secondary ability not defined does not prevent primary ability from getting
    # toggled
    api_fighter.update()
    assert len(api_fighter.abilities) == 1
    assert api_fighter.abilities[eve_primary_abil_id].state is False
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(1017.5)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(100)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - attempt to change state of ability which is not defined for a fighter did not
    # affect effect mode, and it is still disabled on 2nd source
    api_fighter.update()
    assert len(api_fighter.abilities) == 2
    assert api_fighter.abilities[eve_primary_abil_id].state is False
    assert api_fighter.abilities[eve_secondary_abil_id].state is False
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(2035)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(200)
    # Action
    api_fighter.change_fighter(abilities={eve_secondary_abil_id: True})
    # Verification - attempt to change state of ability which is not defined for a fighter did not
    # affect effect mode, and it is still disabled on 2nd source
    api_fighter.update()
    assert len(api_fighter.abilities) == 2
    assert api_fighter.abilities[eve_primary_abil_id].state is False
    assert api_fighter.abilities[eve_secondary_abil_id].state is True
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(2035)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(500)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification - only one ability is exposed. But, since abilities are implemented as effect
    # mode switches, effect mode stays as-is when source is switched back, and its modifiers are
    # applied
    api_fighter.update()
    assert len(api_fighter.abilities) == 1
    assert api_fighter.abilities[eve_primary_abil_id].state is False
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(1017.5)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(250)


def test_src_switch_no_abil(client, consts):
    # Case when ability is not defined altogether
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_affector_attr1_id = client.mk_eve_attr(datas=[eve_d1, eve_d2], id_=consts.EveAttr.ftr_abil_ab_speed_bonus)
    eve_affectee_attr1_id = client.mk_eve_attr(datas=[eve_d1, eve_d2], id_=consts.EveAttr.max_velocity)
    eve_affector_attr2_id = client.mk_eve_attr(datas=[eve_d1, eve_d2], id_=consts.EveAttr.ftr_abil_mjd_sig_radius_bonus)
    eve_affectee_attr2_id = client.mk_eve_attr(datas=[eve_d1, eve_d2], id_=consts.EveAttr.sig_radius)
    eve_primary_effect_id = client.mk_eve_effect(
        datas=[eve_d1, eve_d2], id_=consts.EveEffect.ftr_abil_ab, cat_id=consts.EveEffCat.active)
    eve_secondary_effect_id = client.mk_eve_effect(
        datas=[eve_d1, eve_d2], id_=consts.EveEffect.ftr_abil_mjd, cat_id=consts.EveEffCat.active)
    eve_primary_abil_id = client.mk_eve_abil(datas=[eve_d1, eve_d2], id_=consts.EveAbil.ab)
    eve_secondary_abil_id = client.mk_eve_abil(datas=[eve_d2], id_=consts.EveAbil.mjd)
    eve_fighter_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_fighter(
        datas=[eve_d1],
        id_=eve_fighter_id,
        attrs={
            eve_affector_attr1_id: 400, eve_affectee_attr1_id: 1017.5,
            eve_affector_attr2_id: 150, eve_affectee_attr2_id: 100},
        eff_ids=[eve_primary_effect_id, eve_secondary_effect_id],
        defeff_id=eve_primary_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_primary_abil_id)])
    client.mk_eve_fighter(
        datas=[eve_d2],
        id_=eve_fighter_id,
        attrs={
            eve_affector_attr1_id: 400, eve_affectee_attr1_id: 2035,
            eve_affector_attr2_id: 150, eve_affectee_attr2_id: 200},
        eff_ids=[eve_primary_effect_id, eve_secondary_effect_id],
        defeff_id=eve_primary_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_primary_abil_id), client.mk_eve_item_abil(id_=eve_secondary_abil_id)])
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fighter.update()
    assert len(api_fighter.abilities) == 1
    assert api_fighter.abilities[eve_primary_abil_id].state is True
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(5087.5)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(100)
    # Action
    api_fighter.change_fighter(abilities={eve_primary_abil_id: False, eve_secondary_abil_id: True})
    # Verification - secondary ability not defined does not prevent primary ability from getting
    # toggled
    api_fighter.update()
    assert len(api_fighter.abilities) == 1
    assert api_fighter.abilities[eve_primary_abil_id].state is False
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(1017.5)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(100)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - attempt to change state of ability which is not defined for a fighter did not
    # affect effect mode, and it is still disabled on 2nd source
    api_fighter.update()
    assert len(api_fighter.abilities) == 2
    assert api_fighter.abilities[eve_primary_abil_id].state is False
    assert api_fighter.abilities[eve_secondary_abil_id].state is False
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(2035)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(200)
    # Action
    api_fighter.change_fighter(abilities={eve_secondary_abil_id: True})
    # Verification - attempt to change state of ability which is not defined for a fighter did not
    # affect effect mode, and it is still disabled on 2nd source
    api_fighter.update()
    assert len(api_fighter.abilities) == 2
    assert api_fighter.abilities[eve_primary_abil_id].state is False
    assert api_fighter.abilities[eve_secondary_abil_id].state is True
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(2035)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(500)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification - only one ability is exposed. But, since abilities are implemented as effect
    # mode switches, effect mode stays as-is when source is switched back, and its modifiers are
    # applied
    api_fighter.update()
    assert len(api_fighter.abilities) == 1
    assert api_fighter.abilities[eve_primary_abil_id].state is False
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(1017.5)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(250)
