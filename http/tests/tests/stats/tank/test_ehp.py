from tests import approx
from tests.fw.api import StatsOptions


def test_buffer(client, consts):
    eve_shield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_shield_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_em_dmg_resonance)
    eve_shield_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_therm_dmg_resonance)
    eve_shield_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_kin_dmg_resonance)
    eve_shield_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_expl_dmg_resonance)
    eve_armor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_armor_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_em_dmg_resonance)
    eve_armor_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_therm_dmg_resonance)
    eve_armor_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_kin_dmg_resonance)
    eve_armor_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_expl_dmg_resonance)
    eve_structure_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_struct_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.em_dmg_resonance)
    eve_struct_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.therm_dmg_resonance)
    eve_struct_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.kin_dmg_resonance)
    eve_struct_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.expl_dmg_resonance)
    eve_ship_id = client.mk_eve_ship(attrs={
        eve_shield_attr_id: 225,
        eve_shield_em_attr_id: 1,
        eve_shield_therm_attr_id: 0.8,
        eve_shield_kin_attr_id: 0.6,
        eve_shield_expl_attr_id: 0.4,
        eve_armor_attr_id: 575,
        eve_armor_em_attr_id: 0.5,
        eve_armor_therm_attr_id: 0.65,
        eve_armor_kin_attr_id: 0.75,
        eve_armor_expl_attr_id: 0.7,
        eve_structure_attr_id: 525,
        eve_struct_em_attr_id: 0.67,
        eve_struct_therm_attr_id: 0.67,
        eve_struct_kin_attr_id: 0.67,
        eve_struct_expl_attr_id: 0.67})
    client.create_sources()
    api_sol = client.create_sol(default_incoming_dps=(1, 1, 1, 1))
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(ehp=True))
    assert api_stats.ehp[0].shield == (approx(321.428571), 0, 0, approx(1.428571))
    assert api_stats.ehp[0].armor == (approx(884.615385), 0, 0, approx(1.538462))
    assert api_stats.ehp[0].hull == (approx(783.58209), 0, 0, approx(1.492537))
    # Action
    api_sol.change(default_incoming_dps=(1, 1, 0, 0))
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(ehp=True))
    assert api_stats.ehp[0].shield == (approx(250), 0, 0, approx(1.111111))
    assert api_stats.ehp[0].armor == (approx(1000), 0, 0, approx(1.73913))
    assert api_stats.ehp[0].hull == (approx(783.58209), 0, 0, approx(1.492537))


def test_local_asb(client, consts):
    eve_shield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_shield_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_em_dmg_resonance)
    eve_shield_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_therm_dmg_resonance)
    eve_shield_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_kin_dmg_resonance)
    eve_shield_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_expl_dmg_resonance)
    eve_armor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_armor_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_em_dmg_resonance)
    eve_armor_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_therm_dmg_resonance)
    eve_armor_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_kin_dmg_resonance)
    eve_armor_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_expl_dmg_resonance)
    eve_structure_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_struct_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.em_dmg_resonance)
    eve_struct_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.therm_dmg_resonance)
    eve_struct_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.kin_dmg_resonance)
    eve_struct_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.expl_dmg_resonance)
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_bonus)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_rep_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fueled_shield_boosting,
        cat_id=consts.EveEffCat.active)
    eve_ship_id = client.mk_eve_ship(attrs={
        eve_shield_attr_id: 833,
        eve_shield_em_attr_id: 0.25,
        eve_shield_therm_attr_id: 0.4,
        eve_shield_kin_attr_id: 0.6,
        eve_shield_expl_attr_id: 0.5,
        eve_armor_attr_id: 457,
        eve_armor_em_attr_id: 0.1,
        eve_armor_therm_attr_id: 0.325,
        eve_armor_kin_attr_id: 0.75,
        eve_armor_expl_attr_id: 0.9,
        eve_structure_attr_id: 605,
        eve_struct_em_attr_id: 0.67,
        eve_struct_therm_attr_id: 0.67,
        eve_struct_kin_attr_id: 0.67,
        eve_struct_expl_attr_id: 0.67})
    eve_rep_item_id = client.mk_eve_item(
        attrs={
            eve_rep_amount_attr_id: 146,
            eve_capacity_attr_id: 14,
            eve_charge_rate_attr_id: 1},
        eff_ids=[eve_rep_effect_id],
        defeff_id=eve_rep_effect_id)
    eve_charge_item_id = client.mk_eve_item(attrs={eve_volume_attr_id: 1.5})
    client.create_sources()
    api_sol = client.create_sol(default_incoming_dps=(1, 1, 1, 1))
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(ehp=True))
    assert api_stats.ehp[0].shield == (approx(1904), approx(3003.428571), 0, approx(2.285714))
    assert api_stats.ehp[0].armor == (approx(880.963855), 0, 0, approx(1.927711))
    assert api_stats.ehp[0].hull == (approx(902.985075), 0, 0, approx(1.492537))
    # Action
    api_sol.change(default_incoming_dps=(1, 1, 0, 0))
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(ehp=True))
    assert api_stats.ehp[0].shield == (approx(2563.076923), approx(4043.076923), 0, approx(3.076923))
    assert api_stats.ehp[0].armor == (approx(2150.588235), 0, 0, approx(4.705882))
    assert api_stats.ehp[0].hull == (approx(902.985075), 0, 0, approx(1.492537))


def test_local_aar(client, consts):
    eve_shield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_shield_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_em_dmg_resonance)
    eve_shield_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_therm_dmg_resonance)
    eve_shield_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_kin_dmg_resonance)
    eve_shield_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_expl_dmg_resonance)
    eve_armor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_armor_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_em_dmg_resonance)
    eve_armor_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_therm_dmg_resonance)
    eve_armor_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_kin_dmg_resonance)
    eve_armor_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_expl_dmg_resonance)
    eve_structure_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_struct_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.em_dmg_resonance)
    eve_struct_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.therm_dmg_resonance)
    eve_struct_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.kin_dmg_resonance)
    eve_struct_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.expl_dmg_resonance)
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_rep_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_rep_effect_id = client.mk_eve_effect(id_=consts.EveEffect.fueled_armor_repair, cat_id=consts.EveEffCat.active)
    eve_ship_id = client.mk_eve_ship(attrs={
        eve_shield_attr_id: 225,
        eve_shield_em_attr_id: 1,
        eve_shield_therm_attr_id: 0.8,
        eve_shield_kin_attr_id: 0.6,
        eve_shield_expl_attr_id: 0.4,
        eve_armor_attr_id: 575,
        eve_armor_em_attr_id: 0.5,
        eve_armor_therm_attr_id: 0.65,
        eve_armor_kin_attr_id: 0.75,
        eve_armor_expl_attr_id: 0.7,
        eve_structure_attr_id: 525,
        eve_struct_em_attr_id: 0.67,
        eve_struct_therm_attr_id: 0.67,
        eve_struct_kin_attr_id: 0.67,
        eve_struct_expl_attr_id: 0.67})
    eve_rep_item_id = client.mk_eve_item(
        attrs={
            eve_rep_mult_attr_id: 3,
            eve_rep_amount_attr_id: 52,
            eve_capacity_attr_id: 0.08,
            eve_charge_rate_attr_id: 1},
        eff_ids=[eve_rep_effect_id],
        defeff_id=eve_rep_effect_id)
    eve_charge_item_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste, attrs={eve_volume_attr_id: 0.01})
    client.create_sources()
    api_sol = client.create_sol(default_incoming_dps=(1, 1, 1, 1))
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(ehp=True))
    assert api_stats.ehp[0].shield == (approx(321.428571), 0, 0, approx(1.428571))
    assert api_stats.ehp[0].armor == (approx(884.615385), approx(1920), 0, approx(1.538462))
    assert api_stats.ehp[0].hull == (approx(783.58209), 0, 0, approx(1.492537))
    # Action
    api_sol.change(default_incoming_dps=(1, 1, 0, 0))
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(ehp=True))
    assert api_stats.ehp[0].shield == (approx(250), 0, 0, approx(1.111111))
    assert api_stats.ehp[0].armor == (approx(1000), approx(2170.434782), 0, approx(1.73913))
    assert api_stats.ehp[0].hull == (approx(783.58209), 0, 0, approx(1.492537))


def test_remote_asb(client, consts):
    eve_shield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_shield_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_em_dmg_resonance)
    eve_shield_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_therm_dmg_resonance)
    eve_shield_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_kin_dmg_resonance)
    eve_shield_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_expl_dmg_resonance)
    eve_armor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_armor_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_em_dmg_resonance)
    eve_armor_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_therm_dmg_resonance)
    eve_armor_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_kin_dmg_resonance)
    eve_armor_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_expl_dmg_resonance)
    eve_structure_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_struct_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.em_dmg_resonance)
    eve_struct_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.therm_dmg_resonance)
    eve_struct_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.kin_dmg_resonance)
    eve_struct_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.expl_dmg_resonance)
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_bonus)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_rep_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_ancillary_remote_shield_booster,
        cat_id=consts.EveEffCat.target)
    eve_ship_id = client.mk_eve_ship(attrs={
        eve_shield_attr_id: 833,
        eve_shield_em_attr_id: 0.25,
        eve_shield_therm_attr_id: 0.4,
        eve_shield_kin_attr_id: 0.6,
        eve_shield_expl_attr_id: 0.5,
        eve_armor_attr_id: 457,
        eve_armor_em_attr_id: 0.1,
        eve_armor_therm_attr_id: 0.325,
        eve_armor_kin_attr_id: 0.75,
        eve_armor_expl_attr_id: 0.9,
        eve_structure_attr_id: 605,
        eve_struct_em_attr_id: 0.67,
        eve_struct_therm_attr_id: 0.67,
        eve_struct_kin_attr_id: 0.67,
        eve_struct_expl_attr_id: 0.67})
    eve_rep_item_id = client.mk_eve_item(
        attrs={
            eve_rep_amount_attr_id: 475,
            eve_capacity_attr_id: 14,
            eve_charge_rate_attr_id: 1},
        eff_ids=[eve_rep_effect_id],
        defeff_id=eve_rep_effect_id)
    eve_charge_item_id = client.mk_eve_item(attrs={eve_volume_attr_id: 1.5})
    client.create_sources()
    api_sol = client.create_sol(default_incoming_dps=(1, 1, 1, 1))
    api_src_fit = api_sol.create_fit()
    api_rasb = api_src_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_rasb.change_module(add_projs=[api_ship.id])
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(ehp=True))
    assert api_stats.ehp[0].shield == (approx(1904), 0, approx(9771.428571), approx(2.285714))
    assert api_stats.ehp[0].armor == (approx(880.963855), 0, 0, approx(1.927711))
    assert api_stats.ehp[0].hull == (approx(902.985075), 0, 0, approx(1.492537))
    # Action
    api_sol.change(default_incoming_dps=(1, 1, 0, 0))
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(ehp=True))
    assert api_stats.ehp[0].shield == (approx(2563.076923), 0, approx(13153.846154), approx(3.076923))
    assert api_stats.ehp[0].armor == (approx(2150.588235), 0, 0, approx(4.705882))
    assert api_stats.ehp[0].hull == (approx(902.985075), 0, 0, approx(1.492537))


def test_remote_aar(client, consts):
    eve_shield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_shield_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_em_dmg_resonance)
    eve_shield_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_therm_dmg_resonance)
    eve_shield_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_kin_dmg_resonance)
    eve_shield_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_expl_dmg_resonance)
    eve_armor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_armor_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_em_dmg_resonance)
    eve_armor_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_therm_dmg_resonance)
    eve_armor_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_kin_dmg_resonance)
    eve_armor_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_expl_dmg_resonance)
    eve_structure_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_struct_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.em_dmg_resonance)
    eve_struct_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.therm_dmg_resonance)
    eve_struct_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.kin_dmg_resonance)
    eve_struct_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.expl_dmg_resonance)
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_rep_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_rep_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_ancillary_remote_armor_repairer,
        cat_id=consts.EveEffCat.target)
    eve_ship_id = client.mk_eve_ship(attrs={
        eve_shield_attr_id: 225,
        eve_shield_em_attr_id: 1,
        eve_shield_therm_attr_id: 0.8,
        eve_shield_kin_attr_id: 0.6,
        eve_shield_expl_attr_id: 0.4,
        eve_armor_attr_id: 575,
        eve_armor_em_attr_id: 0.5,
        eve_armor_therm_attr_id: 0.65,
        eve_armor_kin_attr_id: 0.75,
        eve_armor_expl_attr_id: 0.7,
        eve_structure_attr_id: 525,
        eve_struct_em_attr_id: 0.67,
        eve_struct_therm_attr_id: 0.67,
        eve_struct_kin_attr_id: 0.67,
        eve_struct_expl_attr_id: 0.67})
    eve_rep_item_id = client.mk_eve_item(
        attrs={
            eve_rep_mult_attr_id: 3,
            eve_rep_amount_attr_id: 37,
            eve_capacity_attr_id: 0.08,
            eve_charge_rate_attr_id: 1},
        eff_ids=[eve_rep_effect_id],
        defeff_id=eve_rep_effect_id)
    eve_charge_item_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste, attrs={eve_volume_attr_id: 0.01})
    client.create_sources()
    api_sol = client.create_sol(default_incoming_dps=(1, 1, 1, 1))
    api_src_fit = api_sol.create_fit()
    api_raar = api_src_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_raar.change_module(add_projs=[api_ship.id])
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(ehp=True))
    assert api_stats.ehp[0].shield == (approx(321.428571), 0, 0, approx(1.428571))
    assert api_stats.ehp[0].armor == (approx(884.615385), 0, approx(1366.153846), approx(1.538462))
    assert api_stats.ehp[0].hull == (approx(783.58209), 0, 0, approx(1.492537))
    # Action
    api_sol.change(default_incoming_dps=(1, 1, 0, 0))
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(ehp=True))
    assert api_stats.ehp[0].shield == (approx(250), 0, 0, approx(1.111111))
    assert api_stats.ehp[0].armor == (approx(1000), 0, approx(1544.347826), approx(1.73913))
    assert api_stats.ehp[0].hull == (approx(783.58209), 0, 0, approx(1.492537))


def test_no_ship(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    client.mk_eve_attr(id_=consts.EveAttr.shield_em_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.shield_therm_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.shield_kin_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.shield_expl_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    client.mk_eve_attr(id_=consts.EveAttr.armor_em_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.armor_therm_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.armor_kin_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.armor_expl_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.hp)
    client.mk_eve_attr(id_=consts.EveAttr.em_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.therm_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.kin_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.expl_dmg_resonance)
    client.create_sources()
    api_sol = client.create_sol(default_incoming_dps=(1, 1, 1, 1))
    api_fit = api_sol.create_fit()
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(ehp=True))
    assert api_stats.ehp == [None]


def test_ship_not_loaded(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    client.mk_eve_attr(id_=consts.EveAttr.shield_em_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.shield_therm_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.shield_kin_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.shield_expl_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    client.mk_eve_attr(id_=consts.EveAttr.armor_em_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.armor_therm_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.armor_kin_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.armor_expl_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.hp)
    client.mk_eve_attr(id_=consts.EveAttr.em_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.therm_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.kin_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.expl_dmg_resonance)
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol(default_incoming_dps=(1, 1, 1, 1))
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(ehp=True))
    assert api_stats.ehp == [None]
