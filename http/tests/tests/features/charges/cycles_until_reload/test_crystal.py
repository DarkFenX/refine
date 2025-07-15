from tests import approx, check_no_field


def test_basic_not_damaged(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_hp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_dmg_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystals_get_damaged)
    eve_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_chance)
    eve_dmg_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_damage)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_crystal,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_charge_id = client.mk_eve_item(attrs={
        eve_volume_attr_id: 1,
        eve_dmg_flag_attr_id: 0,
        eve_hp_attr_id: 1,
        eve_chance_attr_id: 0.1,
        eve_dmg_attr_id: 0.01})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1, eve_cycle_time_attr_id: 1000},
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
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_crystal,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_charge_id = client.mk_eve_item(attrs={
        eve_volume_attr_id: 1,
        eve_dmg_flag_attr_id: 1,
        eve_hp_attr_id: 1,
        eve_chance_attr_id: 0.1,
        eve_dmg_attr_id: 0.01})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_reload == 1000


def test_damage_flag_values(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_hp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_dmg_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystals_get_damaged)
    eve_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_chance)
    eve_dmg_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_damage)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_crystal,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)

    def mk_eve_charge(*, dmg_flag_val: float) -> int:
        return client.mk_eve_item(attrs={
            eve_volume_attr_id: 1,
            eve_dmg_flag_attr_id: dmg_flag_val,
            eve_hp_attr_id: 1,
            eve_chance_attr_id: 0.1,
            eve_dmg_attr_id: 0.01})

    eve_charge1_id = mk_eve_charge(dmg_flag_val=-55)
    eve_charge2_id = mk_eve_charge(dmg_flag_val=-0.1)
    eve_charge3_id = mk_eve_charge(dmg_flag_val=0.1)
    eve_charge4_id = mk_eve_charge(dmg_flag_val=23)
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    api_module2 = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge2_id)
    api_module3 = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge3_id)
    api_module4 = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge4_id)
    # Verification
    assert api_module1.update().cycles_until_reload == 1000
    assert api_module2.update().cycles_until_reload == 1000
    assert api_module3.update().cycles_until_reload == 1000
    assert api_module4.update().cycles_until_reload == 1000


def test_zero_chance(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_hp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_dmg_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystals_get_damaged)
    eve_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_chance)
    eve_dmg_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_damage)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_crystal,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_charge_id = client.mk_eve_item(attrs={
        eve_volume_attr_id: 1,
        eve_dmg_flag_attr_id: 1,
        eve_hp_attr_id: 1,
        eve_chance_attr_id: 0,
        eve_dmg_attr_id: 0.01})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_reload is None


def test_zero_damage(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_hp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_dmg_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystals_get_damaged)
    eve_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_chance)
    eve_dmg_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_damage)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_crystal,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_charge_id = client.mk_eve_item(attrs={
        eve_volume_attr_id: 1,
        eve_dmg_flag_attr_id: 1,
        eve_hp_attr_id: 1,
        eve_chance_attr_id: 0.1,
        eve_dmg_attr_id: 0})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_reload is None


def test_no_attr_damage_flag(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_hp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_chance)
    eve_dmg_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_damage)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_crystal,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_charge_id = client.mk_eve_item(attrs={
        eve_volume_attr_id: 1,
        eve_hp_attr_id: 1,
        eve_chance_attr_id: 0.1,
        eve_dmg_attr_id: 0.01})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_reload is None


def test_no_attr_chance(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_hp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_dmg_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystals_get_damaged)
    eve_dmg_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_damage)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_crystal,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_charge_id = client.mk_eve_item(attrs={
        eve_volume_attr_id: 1,
        eve_dmg_flag_attr_id: 1,
        eve_hp_attr_id: 1,
        eve_dmg_attr_id: 0.01})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_reload is None


def test_no_attr_damage(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_hp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_dmg_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystals_get_damaged)
    eve_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_chance)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_crystal,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_charge_id = client.mk_eve_item(attrs={
        eve_volume_attr_id: 1,
        eve_dmg_flag_attr_id: 1,
        eve_hp_attr_id: 1,
        eve_chance_attr_id: 0.1})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_reload is None


def test_modified(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_hp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_dmg_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystals_get_damaged)
    eve_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_chance)
    eve_dmg_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_damage)
    eve_hp_mod_attr_id = client.mk_eve_attr()
    eve_dmg_flag_mod_attr_id = client.mk_eve_attr()
    eve_chance_mod_attr_id = client.mk_eve_attr()
    eve_dmg_mod_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_dmg_flag_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_dmg_flag_mod_attr_id,
        affectee_attr_id=eve_dmg_flag_attr_id)
    eve_hp_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_hp_mod_attr_id,
        affectee_attr_id=eve_hp_attr_id)
    eve_chance_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_chance_mod_attr_id,
        affectee_attr_id=eve_chance_attr_id)
    eve_dmg_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_dmg_mod_attr_id,
        affectee_attr_id=eve_dmg_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.passive,
        mod_info=[eve_hp_mod, eve_dmg_flag_mod, eve_chance_mod, eve_dmg_mod])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_crystal,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_charge_id = client.mk_eve_item(attrs={
        eve_volume_attr_id: 1,
        eve_dmg_flag_attr_id: 1,
        eve_hp_attr_id: 1,
        eve_chance_attr_id: 0.1,
        eve_dmg_attr_id: 0.01})
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_capacity_attr_id: 1,
            eve_dmg_flag_mod_attr_id: 0,
            eve_hp_mod_attr_id: 100,
            eve_chance_mod_attr_id: -50,
            eve_dmg_mod_attr_id: -50,
            eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id, eve_mod_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification - unmodified values are used from charge
    api_module.update()
    assert api_module.charge.attrs[eve_dmg_flag_attr_id].extra == approx(0)
    assert api_module.charge.attrs[eve_hp_attr_id].extra == approx(2)
    assert api_module.charge.attrs[eve_chance_attr_id].extra == approx(0.05)
    assert api_module.charge.attrs[eve_dmg_attr_id].extra == approx(0.005)
    assert api_module.cycles_until_reload == 1000


def test_multiple_charges(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_hp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_dmg_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystals_get_damaged)
    eve_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_chance)
    eve_dmg_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_damage)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_crystal,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_charge_id = client.mk_eve_item(attrs={
        eve_volume_attr_id: 1,
        eve_dmg_flag_attr_id: 1,
        eve_hp_attr_id: 1,
        eve_chance_attr_id: 0.1,
        eve_dmg_attr_id: 0.01})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 2.9, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification - cycles until reload increases proportionally with loaded charge count
    api_module.update()
    assert api_module.charge_count == 2
    assert api_module.cycles_until_reload == 2000


def test_no_charge(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_crystal,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1, eve_cycle_time_attr_id: 1000},
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
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_crystal,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_charge_id = client.alloc_item_id()
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_reload is None
