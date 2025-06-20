from tests import approx
from tests.fw.api import StatsOptions


def test_buffer(client, consts):
    eve_shield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_armor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_structure_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_shield_attr_id: 3000, eve_armor_attr_id: 2000, eve_structure_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
