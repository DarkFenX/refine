from fw import check_no_field


def test_count_set_not_loaded(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_max_count_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2], id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_fighter(datas=[eve_d1], id_=eve_fighter_id, attrs={eve_max_count_attr_id: 12})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_fighter.update()
    assert api_fighter.count_effective == [12, 12]
    with check_no_field():
        api_fighter.count_override  # ruff:ignore[useless-expression]
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_fighter.update()
    with check_no_field():
        api_fighter.count_effective  # ruff:ignore[useless-expression]
    with check_no_field():
        api_fighter.count_override  # ruff:ignore[useless-expression]
    # Action
    api_fighter.change_fighter(count_override=3)
    # Verification
    api_fighter.update()
    with check_no_field():
        api_fighter.count_effective  # ruff:ignore[useless-expression]
    assert api_fighter.count_override == 3
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    api_fighter.update()
    assert api_fighter.count_effective == [3, 12]
    assert api_fighter.count_override == 3


def test_rearm_minion_set_not_loaded(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_cycle_time_attr_id = client.mk_eve_attr(datas=[eve_d1])
    eve_effect_id = client.mk_eve_effect(
        datas=[eve_d1],
        id_=consts.EveEffect.ftr_abil_missiles,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_abil_id = client.mk_eve_abil(datas=[eve_d1], id_=consts.EveAbil.heavy_rocket_salvo)
    eve_fighter_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_fighter(
        datas=[eve_d1],
        id_=eve_fighter_id,
        attrs={eve_cycle_time_attr_id: 6000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_abil_id, charge_count=12, charge_rearm_time=5)])
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1, default_rearm_minions=consts.ApiRearmMinion.on_first_empty)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_fighter.update()
    assert api_fighter.rearm_minion_effective == consts.ApiRearmMinion.on_first_empty
    with check_no_field():
        api_fighter.rearm_minion_override  # ruff:ignore[useless-expression]
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_fighter.update()
    with check_no_field():
        api_fighter.rearm_minion_effective  # ruff:ignore[useless-expression]
    with check_no_field():
        api_fighter.rearm_minion_override  # ruff:ignore[useless-expression]
    # Action
    api_fighter.change_fighter(rearm_minion_override=consts.ApiRearmMinion.disabled)
    # Verification
    api_fighter.update()
    with check_no_field():
        api_fighter.rearm_minion_effective  # ruff:ignore[useless-expression]
    assert api_fighter.rearm_minion_override == consts.ApiRearmMinion.disabled
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    api_fighter.update()
    assert api_fighter.rearm_minion_effective == consts.ApiRearmMinion.disabled
    assert api_fighter.rearm_minion_override == consts.ApiRearmMinion.disabled


def test_count_float(client, consts):
    # Floats are rounded, then clamped
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_max_count_attr_id: 12})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, count_override=2.6)
    # Verification
    api_fighter.update()
    assert api_fighter.count_effective == [3, 12]
    assert api_fighter.count_override == 3
    # Action
    api_fighter.change_fighter(count_override=4.2)
    # Verification
    api_fighter.update()
    assert api_fighter.count_effective == [4, 12]
    assert api_fighter.count_override == 4
    # Action
    with api_fit.batch() as api_fit_batch:
        api_fit_batch.change_fighter(item_id=api_fighter.id, count_override=0.4)
    # Verification
    api_fighter.update()
    assert api_fighter.count_effective == [1, 12]
    assert api_fighter.count_override == 1


def test_rearm_minion_ability_charges(client, consts):
    # The setting is used only by fighters which have an ability with limited charge count
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_missiles,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_abil_id = client.mk_eve_abil(id_=consts.EveAbil.heavy_rocket_salvo)
    eve_fighter1_id = client.mk_eve_fighter(
        attrs={eve_cycle_time_attr_id: 6000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_abil_id, charge_count=12, charge_rearm_time=5)])
    eve_fighter2_id = client.mk_eve_fighter(
        attrs={eve_cycle_time_attr_id: 6000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_abil_id)])
    eve_fighter3_id = client.mk_eve_fighter(
        attrs={eve_cycle_time_attr_id: 6000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_abil_id, charge_count=0, charge_rearm_time=5)])
    client.create_sources()
    api_sol = client.create_sol(default_rearm_minions=consts.ApiRearmMinion.on_first_empty)
    api_fit = api_sol.create_fit()
    # Action
    api_fighter = api_fit.add_fighter(type_id=eve_fighter1_id)
    # Verification
    api_fighter.update()
    assert api_fighter.rearm_minion_effective == consts.ApiRearmMinion.on_first_empty
    with check_no_field():
        api_fighter.rearm_minion_override  # ruff:ignore[useless-expression]
    # Action
    api_fighter.change_fighter(rearm_minion_override=consts.ApiRearmMinion.disabled)
    # Verification
    api_fighter.update()
    assert api_fighter.rearm_minion_effective == consts.ApiRearmMinion.disabled
    assert api_fighter.rearm_minion_override == consts.ApiRearmMinion.disabled
    # Action
    api_fighter.change_fighter(type_id=eve_fighter2_id)
    # Verification - override is kept, but nothing uses it when charges are unlimited
    api_fighter.update()
    with check_no_field():
        api_fighter.rearm_minion_effective  # ruff:ignore[useless-expression]
    assert api_fighter.rearm_minion_override == consts.ApiRearmMinion.disabled
    # Action
    api_fighter.change_fighter(type_id=eve_fighter3_id)
    # Verification - same when the ability has no charges at all
    api_fighter.update()
    with check_no_field():
        api_fighter.rearm_minion_effective  # ruff:ignore[useless-expression]
    assert api_fighter.rearm_minion_override == consts.ApiRearmMinion.disabled
    # Action
    api_fighter.change_fighter(type_id=eve_fighter1_id)
    # Verification
    api_fighter.update()
    assert api_fighter.rearm_minion_effective == consts.ApiRearmMinion.disabled
    assert api_fighter.rearm_minion_override == consts.ApiRearmMinion.disabled
