from tests import Effect, check_no_field
from tests.fw.api import ValOptions


def test_state(client, consts):
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_launch_bomb, cat_id=consts.EveEffCat.active)
    eve_abil_id = client.mk_eve_abil(id_=consts.EveAbil.launch_bomb, banned_hisec=True, banned_lowsec=True)
    eve_fighter_id = client.mk_eve_fighter(eff_ids=[eve_effect_id], abils=[client.mk_eve_item_abil(id_=eve_abil_id)])
    client.create_sources()
    api_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_effect=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fighter.change_fighter(abilities={eve_abil_id: True})
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_effect=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_effect.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_effect.items == {api_fighter.id: {
        api_effect_id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}}
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_effect=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fighter.change_fighter(effect_modes={api_effect_id: consts.ApiEffMode.force_run})
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_effect=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_effect.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_effect.items == {api_fighter.id: {
        api_effect_id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}}


def test_sec_zones(client, consts):
    eve_effect1_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_launch_bomb, cat_id=consts.EveEffCat.active)
    eve_effect2_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_attack_m, cat_id=consts.EveEffCat.active)
    eve_effect3_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_missiles, cat_id=consts.EveEffCat.active)
    eve_abil1_id = client.mk_eve_abil(id_=consts.EveAbil.launch_bomb, banned_hisec=True, banned_lowsec=True)
    eve_abil2_id = client.mk_eve_abil(id_=consts.EveAbil.pulse_cannon, banned_hisec=True, banned_lowsec=False)
    eve_abil3_id = client.mk_eve_abil(id_=consts.EveAbil.heavy_rocket_salvo, banned_hisec=False, banned_lowsec=True)
    eve_fighter_id = client.mk_eve_fighter(
        eff_ids=[eve_effect1_id, eve_effect2_id, eve_effect3_id],
        abils=[
            client.mk_eve_item_abil(id_=eve_abil1_id),
            client.mk_eve_item_abil(id_=eve_abil2_id),
            client.mk_eve_item_abil(id_=eve_abil3_id)])
    client.create_sources()
    api_effect1_id = Effect.dogma_to_api(dogma_effect_id=eve_effect1_id)
    api_effect2_id = Effect.dogma_to_api(dogma_effect_id=eve_effect2_id)
    api_effect3_id = Effect.dogma_to_api(dogma_effect_id=eve_effect3_id)
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_abil1_id: True, eve_abil2_id: True, eve_abil3_id: True})
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_effect=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_effect.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_effect.items == {api_fighter.id: {
        api_effect1_id: sorted([
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_effect2_id: sorted([
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard])}}
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.hisec_c5)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_effect=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_effect.zone == consts.ApiSecZone.hisec_c5
    assert api_val.details.sec_zone_effect.items == {api_fighter.id: {
        api_effect1_id: sorted([
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_effect2_id: sorted([
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard])}}
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_effect=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_effect.zone == consts.ApiSecZone.lowsec
    assert api_val.details.sec_zone_effect.items == {api_fighter.id: {
        api_effect1_id: sorted([
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_effect3_id: sorted([
            consts.ApiSecZone.hisec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard])}}
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec_c5)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_effect=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_effect.zone == consts.ApiSecZone.lowsec_c5
    assert api_val.details.sec_zone_effect.items == {api_fighter.id: {
        api_effect1_id: sorted([
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_effect3_id: sorted([
            consts.ApiSecZone.hisec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard])}}
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.nullsec)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_effect=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.wspace)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_effect=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.hazard)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_effect=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_effect1_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_launch_bomb, cat_id=consts.EveEffCat.active)
    eve_effect2_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_attack_m, cat_id=consts.EveEffCat.active)
    eve_effect3_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_missiles, cat_id=consts.EveEffCat.active)
    eve_abil1_id = client.mk_eve_abil(id_=consts.EveAbil.launch_bomb, banned_hisec=True, banned_lowsec=True)
    eve_abil2_id = client.mk_eve_abil(id_=consts.EveAbil.pulse_cannon, banned_hisec=True, banned_lowsec=True)
    eve_abil3_id = client.mk_eve_abil(id_=consts.EveAbil.heavy_rocket_salvo, banned_hisec=True, banned_lowsec=True)
    eve_fighter1_id = client.mk_eve_fighter(eff_ids=[eve_effect1_id], abils=[client.mk_eve_item_abil(id_=eve_abil1_id)])
    eve_fighter2_id = client.mk_eve_fighter(
        eff_ids=[eve_effect1_id, eve_effect2_id, eve_effect3_id],
        abils=[
            client.mk_eve_item_abil(id_=eve_abil1_id),
            client.mk_eve_item_abil(id_=eve_abil2_id),
            client.mk_eve_item_abil(id_=eve_abil3_id)])
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_effect1_id = Effect.dogma_to_api(dogma_effect_id=eve_effect1_id)
    api_effect2_id = Effect.dogma_to_api(dogma_effect_id=eve_effect2_id)
    api_effect3_id = Effect.dogma_to_api(dogma_effect_id=eve_effect3_id)
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_fighter1 = api_fit.add_fighter(
        type_id=eve_fighter1_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_abil1_id: True})
    api_fighter2 = api_fit.add_fighter(
        type_id=eve_fighter2_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_abil1_id: True, eve_abil2_id: True, eve_abil3_id: True})
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_effect=(True, [api_fighter1.id])))
    assert api_val.passed is False
    assert api_val.details.sec_zone_effect.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_effect.items == {api_fighter2.id: {
        api_effect1_id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_effect2_id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_effect3_id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}}
    api_val = api_fit.validate(options=ValOptions(sec_zone_effect=(True, [api_fighter2.id])))
    assert api_val.passed is False
    assert api_val.details.sec_zone_effect.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_effect.items == {api_fighter1.id: {
        api_effect1_id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}}
    api_val = api_fit.validate(options=ValOptions(sec_zone_effect=(True, [api_fighter1.id, api_fighter2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        sec_zone_effect=(True, [api_fighter1.id, api_other.id, api_fighter2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded(client, consts):
    eve_fighter_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_effect=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
