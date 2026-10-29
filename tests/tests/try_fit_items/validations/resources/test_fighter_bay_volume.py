from fw import approx
from fw.api import FitStatsOptions, ValOptions


def test_fail_single(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 800, eve_count_attr_id: 9})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 8000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(0), approx(8000))
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=True),
        passing=[eve_fighter2_id],
        failing=[eve_fighter1_id])


def test_fail_multiple_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 800, eve_count_attr_id: 12})
    eve_fighter3_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 600, eve_count_attr_id: 10})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 15000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_fighter(type_id=eve_fighter1_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(9000), approx(15000))
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=True),
        passing=[eve_fighter3_id],
        failing=[eve_fighter2_id])


def test_fail_multiple_struct(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 800, eve_count_attr_id: 12})
    eve_fighter3_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 600, eve_count_attr_id: 10})
    eve_struct_id = client.mk_eve_struct(attrs={eve_max_attr_id: 15000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_fit.add_fighter(type_id=eve_fighter1_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(9000), approx(15000))
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=True),
        passing=[eve_fighter3_id],
        failing=[eve_fighter2_id])


def test_equal(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 9000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(0), approx(9000))
    api_fit.check_try_fit_items(val_options=ValOptions(fighter_bay_volume=True), passing=[eve_fighter_id], failing=[])


def test_known_failures(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 800, eve_count_attr_id: 12})
    eve_fighter3_id = client.mk_eve_fighter(attrs={eve_use_attr_id: -100, eve_count_attr_id: 5})
    eve_fighter4_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 0, eve_count_attr_id: 20})
    eve_fighter5_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 0.5, eve_count_attr_id: 3})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 8000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter1 = api_fit.add_fighter(type_id=eve_fighter1_id)
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter2_id)
    # Verification - items which do not use the resource do not fail it
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(18600), approx(8000))
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=[api_fighter1.id]),
        passing=[],
        failing=[eve_fighter3_id, eve_fighter4_id, eve_fighter2_id, eve_fighter5_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=[api_fighter2.id]),
        passing=[],
        failing=[eve_fighter3_id, eve_fighter4_id, eve_fighter2_id, eve_fighter5_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=[api_fighter1.id, api_fighter2.id]),
        passing=[eve_fighter3_id, eve_fighter4_id],
        failing=[eve_fighter2_id, eve_fighter5_id])


def test_changed_count(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 3})
    eve_fighter3_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 4})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 5000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter1_id, count_override=5)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(5000), approx(5000))
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=True),
        passing=[],
        failing=[eve_fighter2_id, eve_fighter3_id])
    # Action
    api_fighter.change_fighter(count_override=2)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(2000), approx(5000))
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=True),
        passing=[eve_fighter2_id],
        failing=[eve_fighter3_id])


def test_modified_count(client, consts):
    # Max fighter squad size is never modified, so the lib just uses unmodified attributes for
    # faster access to the attr value
    eve_skill_id = client.mk_eve_item()
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_count_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 100, eve_count_attr_id: 9}, srqs={eve_skill_id: 1})
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 700, eve_count_attr_id: 9}, srqs={eve_skill_id: 1})
    eve_fighter3_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 900, eve_count_attr_id: 9}, srqs={eve_skill_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 8000})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 12}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter1_id)
    # Verification
    assert api_fighter.update().attrs[eve_count_attr_id].modified == approx(12)
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(900), approx(8000))
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=True),
        passing=[eve_fighter2_id],
        failing=[eve_fighter3_id])
    # Action
    api_implant.remove()
    # Verification
    assert api_fighter.update().attrs[eve_count_attr_id].modified == approx(9)
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(900), approx(8000))
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=True),
        passing=[eve_fighter2_id],
        failing=[eve_fighter3_id])


def test_modified_use(client, consts):
    # Fighter volume is never modified, so the lib just uses unmodified attributes for faster access
    # to the attr value
    eve_skill_id = client.mk_eve_item()
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_use_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 100, eve_count_attr_id: 9}, srqs={eve_skill_id: 1})
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 700, eve_count_attr_id: 9}, srqs={eve_skill_id: 1})
    eve_fighter3_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 900, eve_count_attr_id: 9}, srqs={eve_skill_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 8000})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: -50}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter1_id)
    # Verification
    assert api_fighter.update().attrs[eve_use_attr_id].modified == approx(50)
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(900), approx(8000))
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=True),
        passing=[eve_fighter2_id],
        failing=[eve_fighter3_id])
    # Action
    api_implant.remove()
    # Verification
    assert api_fighter.update().attrs[eve_use_attr_id].modified == approx(100)
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(900), approx(8000))
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=True),
        passing=[eve_fighter2_id],
        failing=[eve_fighter3_id])


def test_modified_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 8000})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 50}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].modified == approx(8000)
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(0), approx(8000))
    api_fit.check_try_fit_items(val_options=ValOptions(fighter_bay_volume=True), passing=[], failing=[eve_fighter_id])
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].modified == approx(12000)
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(0), approx(12000))
    api_fit.check_try_fit_items(val_options=ValOptions(fighter_bay_volume=True), passing=[eve_fighter_id], failing=[])


def test_rounding(client, consts):
    # Volume/capacity shouldn't have its sum or individual values rounded
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 0.0002, eve_count_attr_id: 10})
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 0.52215, eve_count_attr_id: 10})
    eve_fighter3_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 0.522, eve_count_attr_id: 10})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 5.223})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_fighter(type_id=eve_fighter1_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(0.002), approx(5.223))
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=True),
        passing=[eve_fighter3_id],
        failing=[eve_fighter2_id])


def test_no_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 0, eve_count_attr_id: 9})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(0), None)
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=True),
        passing=[eve_fighter2_id],
        failing=[eve_fighter1_id])


def test_no_attr_use(client, consts):
    eve_use_attr_id = consts.EveAttr.volume
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification - users are assumed to take no resource when attribute does not exist
    api_fit.check_try_fit_items(val_options=ValOptions(fighter_bay_volume=True), passing=[eve_fighter_id], failing=[])


def test_no_attr_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = consts.EveAttr.ftr_capacity
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 5})
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 0, eve_count_attr_id: 5})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 8000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification - when output attr does not exist, it is assumed to be 0
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=True),
        passing=[eve_fighter2_id],
        failing=[eve_fighter1_id])


def test_not_loaded_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 0, eve_count_attr_id: 9})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(0), None)
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=True),
        passing=[eve_fighter2_id],
        failing=[eve_fighter1_id])


def test_non_positive(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 0, eve_count_attr_id: 10})
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 15, eve_count_attr_id: 10})
    eve_fighter3_id = client.mk_eve_fighter(attrs={eve_use_attr_id: -1, eve_count_attr_id: 10})
    eve_fighter4_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 12, eve_count_attr_id: 10})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_fighter(type_id=eve_fighter3_id)
    # Verification - items with negative volume are considered as 0
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(0), approx(125))
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=True),
        passing=[eve_fighter1_id, eve_fighter3_id, eve_fighter4_id],
        failing=[eve_fighter2_id])


def test_no_value_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_count_attr_id: 9})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 8000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(0), approx(8000))
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=True),
        passing=[eve_fighter2_id],
        failing=[eve_fighter1_id])


def test_no_value_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 0, eve_count_attr_id: 9})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(0), approx(0))
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=True),
        passing=[eve_fighter2_id],
        failing=[eve_fighter1_id])


def test_no_value_count(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 1000})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 8000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification - fighter without squad size takes volume of a single fighter
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(0), approx(8000))
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=True),
        passing=[eve_fighter2_id],
        failing=[eve_fighter1_id])


def test_no_attr_count(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = consts.EveAttr.ftr_sq_max_size
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 400, eve_count_attr_id: 9})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification - when squad size attribute does not exist, fighter takes volume of a single
    # fighter
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=True),
        passing=[eve_fighter2_id],
        failing=[eve_fighter1_id])


def test_criterion_item_kind(client, consts):
    # Validation applies only to fighters
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_booster_id = client.mk_eve_booster(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_charge_id = client.mk_eve_charge(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_drone_id = client.mk_eve_drone(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_implant_id = client.mk_eve_implant(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_module_id = client.mk_eve_module(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_rig_id = client.mk_eve_rig(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_service_id = client.mk_eve_service(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_subsystem_id = client.mk_eve_subsystem(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_launcher_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 900, eve_use_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_launcher_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume.one() == (approx(0), approx(900))
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_bay_volume=True),
        passing=[
            eve_booster_id,
            eve_charge_id,
            eve_drone_id,
            eve_implant_id,
            eve_module_id,
            eve_rig_id,
            eve_service_id,
            eve_subsystem_id],
        failing=[eve_fighter_id])
