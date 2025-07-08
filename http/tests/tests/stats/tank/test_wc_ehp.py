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
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(wc_ehp=True))
    assert api_stats.wc_ehp.shield == (approx(225), 0, 0, approx(1))
    assert api_stats.wc_ehp.armor == (approx(766.666667), 0, 0, approx(1.333333))
    assert api_stats.wc_ehp.structure == (approx(783.58209), 0, 0, approx(1.492537))
