import typing
from contextlib import contextmanager
from dataclasses import dataclass

from fw.consts import ApiFitInfoMode, ApiFleetInfoMode, ApiItemInfoMode, ApiSolInfoMode

if typing.TYPE_CHECKING:
    from collections.abc import Generator

    from fw.api.types.batches.sol import SolCmdBatchCtx
    from fw.api.types.sol import Sol
    from fw.client import TestClient
    from fw.eve.types import EffectModifier


@dataclass(kw_only=True)
class ModBasicInfo:
    affector_attr_id: int
    affectee_attr_id: int
    # Both modifiers increase affectee attribute by affector attribute value, in percent
    ship_mod: EffectModifier
    tgt_mod: EffectModifier


def setup_mod_basics(
        *,
        client: TestClient,
        consts,  # ruff:ignore[missing-type-function-argument]
) -> ModBasicInfo:
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    return ModBasicInfo(
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id,
        ship_mod=client.mk_eve_effect_mod(
            func=consts.EveModFunc.item,
            loc=consts.EveModLoc.ship,
            op=consts.EveModOp.post_percent,
            affector_attr_id=eve_affector_attr_id,
            affectee_attr_id=eve_affectee_attr_id),
        tgt_mod=client.mk_eve_effect_mod(
            func=consts.EveModFunc.item,
            loc=consts.EveModLoc.tgt,
            op=consts.EveModOp.post_percent,
            affector_attr_id=eve_affector_attr_id,
            affectee_attr_id=eve_affectee_attr_id))


@contextmanager
def rollback_batch(*, api_sol: Sol) -> Generator[SolCmdBatchCtx]:
    """
    Solar system batch which fails after commands added to it. Before failing, it requests full info
    on everything, so that rollback has to drop values calculated for the changed solar system.
    """
    api_other_fleet = api_sol.create_fleet()
    api_other_fleet.remove()
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'FLT-001',
            'message': f'failed to remove fleet: fleet {api_other_fleet.id} not found',
    }) as api_sol_batch:
        # Nothing to clean up if commands fail to be added, so exceptions just propagate
        yield api_sol_batch  # ruff:ignore[fallible-context-manager]
        api_sol_batch.get_sol_info(
            sol_mode=ApiSolInfoMode.full,
            fleet_mode=ApiFleetInfoMode.full,
            fit_mode=ApiFitInfoMode.full,
            item_mode=ApiItemInfoMode.full)
        api_sol_batch.remove_fleet(fleet_id=api_other_fleet.id)
