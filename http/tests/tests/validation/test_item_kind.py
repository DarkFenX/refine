from tests import check_no_field


def test_kind_booster(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.boosterness)
    eve_booster_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, attrs={eve_slot_attr_id: 1})
    eve_other_id = client.mk_eve_item(cat_id=consts.EveItemCat.drone)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_other1 = api_fit.add_booster(type_id=eve_other_id)
    api_other2 = api_fit.add_implant(type_id=eve_booster_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is False
    assert api_val.details.item_kind == {
        api_other1.id: (consts.ApiValItemType.drone, consts.ApiValItemType.booster),
        api_other2.id: (consts.ApiValItemType.booster, consts.ApiValItemType.implant)}


def test_kind_character(client, consts):
    eve_character_id = client.mk_eve_item(grp_id=consts.EveItemGrp.character)
    eve_other_id = client.mk_eve_item(cat_id=consts.EveItemCat.ship)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_char(type_id=eve_character_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_other1 = api_fit.set_char(type_id=eve_other_id)
    api_other2 = api_fit.add_implant(type_id=eve_character_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is False
    assert api_val.details.item_kind == {
        api_other1.id: (consts.ApiValItemType.ship, consts.ApiValItemType.character),
        api_other2.id: (consts.ApiValItemType.character, consts.ApiValItemType.implant)}


def test_kind_charge(client, consts):
    eve_charge_id = client.mk_eve_item(cat_id=consts.EveItemCat.charge)
    eve_rack_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_rack_effect_id])
    eve_other_id = client.mk_eve_item(cat_id=consts.EveItemCat.ship)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_mod(type_id=eve_module_id, rack=consts.ApiRack.high, charge_type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module = api_fit.add_mod(type_id=eve_module_id, rack=consts.ApiRack.high, charge_type_id=eve_other_id)
    api_other = api_fit.add_implant(type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is False
    assert api_val.details.item_kind == {
        api_module.charge.id: (consts.ApiValItemType.ship, consts.ApiValItemType.charge),
        api_other.id: (consts.ApiValItemType.charge, consts.ApiValItemType.implant)}


def test_kind_drone(client, consts):
    eve_drone_id = client.mk_eve_item(cat_id=consts.EveItemCat.drone)
    eve_other_id = client.mk_eve_item(cat_id=consts.EveItemCat.charge)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_other1 = api_fit.add_drone(type_id=eve_other_id)
    api_other2 = api_fit.add_implant(type_id=eve_drone_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is False
    assert api_val.details.item_kind == {
        api_other1.id: (consts.ApiValItemType.charge, consts.ApiValItemType.drone),
        api_other2.id: (consts.ApiValItemType.drone, consts.ApiValItemType.implant)}


def test_kind_fighter(client, consts):
    eve_fighter_id = client.mk_eve_item(cat_id=consts.EveItemCat.fighter)
    eve_other_id = client.mk_eve_item(cat_id=consts.EveItemCat.charge)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_other1 = api_fit.add_fighter(type_id=eve_other_id)
    api_other2 = api_fit.add_implant(type_id=eve_fighter_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is False
    assert api_val.details.item_kind == {
        api_other1.id: (consts.ApiValItemType.charge, consts.ApiValItemType.fighter),
        api_other2.id: (consts.ApiValItemType.fighter, consts.ApiValItemType.implant)}


def test_kind_implant(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.implantness)
    eve_implant_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, attrs={eve_slot_attr_id: 1})
    eve_other_id = client.mk_eve_item(cat_id=consts.EveItemCat.drone)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_other1 = api_fit.add_implant(type_id=eve_other_id)
    api_other2 = api_fit.add_booster(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is False
    assert api_val.details.item_kind == {
        api_other1.id: (consts.ApiValItemType.drone, consts.ApiValItemType.implant),
        api_other2.id: (consts.ApiValItemType.implant, consts.ApiValItemType.booster)}


def test_kind_module_high(client, consts):
    eve_rack_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_rack_effect_id])
    eve_other_id = client.mk_eve_item(cat_id=consts.EveItemCat.ship)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_mod(type_id=eve_module_id, rack=consts.ApiRack.high)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_other1 = api_fit.add_mod(type_id=eve_other_id, rack=consts.ApiRack.high)
    api_other2 = api_fit.add_mod(type_id=eve_module_id, rack=consts.ApiRack.low)
    api_other3 = api_fit.add_implant(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is False
    assert api_val.details.item_kind == {
        api_other1.id: (consts.ApiValItemType.ship, consts.ApiValItemType.module_high),
        api_other2.id: (consts.ApiValItemType.module_high, consts.ApiValItemType.module_low),
        api_other3.id: (consts.ApiValItemType.module_high, consts.ApiValItemType.implant)}


def test_kind_module_mid(client, consts):
    eve_rack_effect_id = client.mk_eve_effect(id_=consts.EveEffect.med_power)
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_rack_effect_id])
    eve_other_id = client.mk_eve_item(cat_id=consts.EveItemCat.ship)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_mod(type_id=eve_module_id, rack=consts.ApiRack.mid)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_other1 = api_fit.add_mod(type_id=eve_other_id, rack=consts.ApiRack.mid)
    api_other2 = api_fit.add_mod(type_id=eve_module_id, rack=consts.ApiRack.high)
    api_other3 = api_fit.add_implant(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is False
    assert api_val.details.item_kind == {
        api_other1.id: (consts.ApiValItemType.ship, consts.ApiValItemType.module_mid),
        api_other2.id: (consts.ApiValItemType.module_mid, consts.ApiValItemType.module_high),
        api_other3.id: (consts.ApiValItemType.module_mid, consts.ApiValItemType.implant)}


def test_kind_module_low(client, consts):
    eve_rack_effect_id = client.mk_eve_effect(id_=consts.EveEffect.lo_power)
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_rack_effect_id])
    eve_other_id = client.mk_eve_item(cat_id=consts.EveItemCat.ship)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_mod(type_id=eve_module_id, rack=consts.ApiRack.low)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_other1 = api_fit.add_mod(type_id=eve_other_id, rack=consts.ApiRack.low)
    api_other2 = api_fit.add_mod(type_id=eve_module_id, rack=consts.ApiRack.mid)
    api_other3 = api_fit.add_implant(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is False
    assert api_val.details.item_kind == {
        api_other1.id: (consts.ApiValItemType.ship, consts.ApiValItemType.module_low),
        api_other2.id: (consts.ApiValItemType.module_low, consts.ApiValItemType.module_mid),
        api_other3.id: (consts.ApiValItemType.module_low, consts.ApiValItemType.implant)}


def test_kind_rig(client, consts):
    eve_rig_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot)
    eve_rig_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_rig_effect_id])
    eve_other_id = client.mk_eve_item(grp_id=consts.EveItemGrp.character)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_other1 = api_fit.add_rig(type_id=eve_other_id)
    api_other2 = api_fit.add_implant(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is False
    assert api_val.details.item_kind == {
        api_other1.id: (consts.ApiValItemType.character, consts.ApiValItemType.rig),
        api_other2.id: (consts.ApiValItemType.rig, consts.ApiValItemType.implant)}


def test_kind_ship(client, consts):
    eve_ship_id = client.mk_eve_item(cat_id=consts.EveItemCat.ship)
    eve_other_id = client.mk_eve_item(cat_id=consts.EveItemCat.drone)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_other1 = api_fit.set_ship(type_id=eve_other_id)
    api_other2 = api_fit.add_implant(type_id=eve_ship_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is False
    assert api_val.details.item_kind == {
        api_other1.id: (consts.ApiValItemType.drone, consts.ApiValItemType.ship),
        api_other2.id: (consts.ApiValItemType.ship, consts.ApiValItemType.implant)}


def test_kind_skill(client, consts):
    eve_skill_id = client.mk_eve_item(cat_id=consts.EveItemCat.skill)
    eve_other_id = client.mk_eve_item(cat_id=consts.EveItemCat.charge)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_skill(type_id=eve_skill_id, level=1)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_other1 = api_fit.add_skill(type_id=eve_other_id, level=1)
    api_other2 = api_fit.add_implant(type_id=eve_skill_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is False
    assert api_val.details.item_kind == {
        api_other1.id: (consts.ApiValItemType.charge, consts.ApiValItemType.skill),
        api_other2.id: (consts.ApiValItemType.skill, consts.ApiValItemType.implant)}


def test_kind_stance(client, consts):
    eve_stance_id = client.mk_eve_item(grp_id=consts.EveItemGrp.ship_mod)
    eve_other_id = client.mk_eve_item(cat_id=consts.EveItemCat.drone)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_stance(type_id=eve_stance_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_other1 = api_fit.set_stance(type_id=eve_other_id)
    api_other2 = api_fit.add_implant(type_id=eve_stance_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is False
    assert api_val.details.item_kind == {
        api_other1.id: (consts.ApiValItemType.drone, consts.ApiValItemType.stance),
        api_other2.id: (consts.ApiValItemType.stance, consts.ApiValItemType.implant)}


def test_kind_subsystem(client, consts):
    eve_subsystem_id = client.mk_eve_item(cat_id=consts.EveItemCat.subsystem)
    eve_other_id = client.mk_eve_item(grp_id=consts.EveItemGrp.character)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_other1 = api_fit.add_subsystem(type_id=eve_other_id)
    api_other2 = api_fit.add_implant(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_kind])
    assert api_val.passed is False
    assert api_val.details.item_kind == {
        api_other1.id: (consts.ApiValItemType.character, consts.ApiValItemType.subsystem),
        api_other2.id: (consts.ApiValItemType.subsystem, consts.ApiValItemType.implant)}
