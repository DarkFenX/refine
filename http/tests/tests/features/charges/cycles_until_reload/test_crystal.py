from tests import check_no_field


def test_basic_not_damaged(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_hp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_dmg_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystals_get_damaged)
    eve_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_chance)
    eve_dmg_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_damage)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.cycle_crystal)
    eve_charge_id = client.mk_eve_item(attrs={
        eve_volume_attr_id: 1,
        eve_dmg_flag_attr_id: 0,
        eve_hp_attr_id: 1,
        eve_chance_attr_id: 0.1,
        eve_dmg_attr_id: 0.01})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_reload is None


def test_basic_damaged(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_hp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_dmg_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystals_get_damaged)
    eve_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_chance)
    eve_dmg_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_damage)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.cycle_crystal)
    eve_charge_id = client.mk_eve_item(attrs={
        eve_volume_attr_id: 1,
        eve_dmg_flag_attr_id: 1,
        eve_hp_attr_id: 1,
        eve_chance_attr_id: 0.1,
        eve_dmg_attr_id: 0.01})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_reload == 1000


def test_no_charge(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.cycle_crystal)
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_module.update()
    with check_no_field():
        api_module.cycles_until_reload  # noqa: B018


def test_charge_not_loaded(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.cycle_crystal)
    eve_charge_id = client.alloc_item_id()
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_reload == 0
