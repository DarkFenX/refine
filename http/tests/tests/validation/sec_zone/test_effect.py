from tests import Effect, check_no_field
from tests.fw.api import ValOptions


def test_main(client, consts):
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
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_effect=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fighter.change_fighter(abilities={eve_abil1_id: True, eve_abil2_id: True, eve_abil3_id: True})
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
    api_fighter.change_fighter(state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_effect=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
