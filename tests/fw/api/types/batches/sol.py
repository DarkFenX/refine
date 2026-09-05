import typing

from fw.api.commands import (
    SolCtlAutochargeChangeCmd,
    SolCtlBoosterAddCmd,
    SolCtlBoosterChangeCmd,
    SolCtlCharacterChangeViaFitIdCmd,
    SolCtlCharacterChangeViaItemIdCmd,
    SolCtlCharacterSetCmd,
    SolCtlCharacterUnsetCmd,
    SolCtlChargeChangeCmd,
    SolCtlDroneAddCmd,
    SolCtlDroneChangeCmd,
    SolCtlFighterAddCmd,
    SolCtlFighterChangeCmd,
    SolCtlFitAddCmd,
    SolCtlFitChangeCmd,
    SolCtlFitRemoveCmd,
    SolCtlFleetAddCmd,
    SolCtlFleetChangeCmd,
    SolCtlFleetRemoveCmd,
    SolCtlFwEffectAddCmd,
    SolCtlFwEffectChangeCmd,
    SolCtlImplantAddCmd,
    SolCtlImplantChangeCmd,
    SolCtlItemRemoveCmd,
    SolCtlModuleAddCmd,
    SolCtlModuleChangeCmd,
    SolCtlProjEffectAddCmd,
    SolCtlProjEffectChangeCmd,
    SolCtlRigAddCmd,
    SolCtlRigChangeCmd,
    SolCtlServiceAddCmd,
    SolCtlServiceChangeCmd,
    SolCtlShipChangeViaFitIdCmd,
    SolCtlShipChangeViaItemIdCmd,
    SolCtlShipSetCmd,
    SolCtlShipUnsetCmd,
    SolCtlSkillAddCmd,
    SolCtlSkillChangeCmd,
    SolCtlSolChangeCmd,
    SolCtlStanceChangeViaFitIdCmd,
    SolCtlStanceChangeViaItemIdCmd,
    SolCtlStanceSetCmd,
    SolCtlStanceUnsetCmd,
    SolCtlSubsystemAddCmd,
    SolCtlSubsystemChangeCmd,
    SolCtlSwEffectAddCmd,
    SolCtlSwEffectChangeCmd,
    SolInfoFitCmd,
    SolInfoFleetCmd,
    SolInfoItemCmd,
    SolInfoSolCmd,
    SolStatsFitCmd,
    SolStatsFleetCmd,
    SolStatsItemCmd,
    SolStatsSolCmd,
    SolTryItemsFitCmd,
    SolValFitCmd,
    SolValSolCmd,
)
from fw.api.types.fit import Fit
from fw.api.types.fleet import Fleet
from fw.api.types.stats import FitBatchStats, FleetBatchStats, ItemBatchStats, SolBatchStats
from fw.api.types.validation import FitValResult, SolValResult
from fw.consts import ApiMinionState, ApiModAddMode, ApiModuleState, ApiRack, ApiServiceState
from fw.util import Absent
from .base import BaseCmdBatchCtx, DataFillKind, EntityData

if typing.TYPE_CHECKING:
    from types import TracebackType

    from fw.api import ApiClient
    from fw.api.aliases import DpsProfileAlias, InfoMode, MutaAdd, MutaChange, ReqHook, StatsOptions
    from fw.api.types.item import Item
    from fw.api.types.sol import SolarSystem
    from fw.api.types.stats import FitStatsOptions, FleetStatsOptions, ItemStatsOptions
    from fw.api.types.validation import ValOptions
    from fw.consts import (
        ApiEffMode,
        ApiFitInfoMode,
        ApiFleetInfoMode,
        ApiItemInfoMode,
        ApiModMvMode,
        ApiModRmMode,
        ApiNpcProp,
        ApiOptionalReload,
        ApiRearmMinion,
        ApiSecZone,
        ApiSolInfoMode,
        ApiValInfoMode,
    )


class SolCmdBatchCtx(BaseCmdBatchCtx):

    def __init__(
            self, *,
            client: ApiClient,
            sol_id: str,
            hook_req: ReqHook | None,
            status_code: int,
            json_predicate: dict | None,
    ) -> None:
        super().__init__(
            client=client,
            sol_id=sol_id,
            hook_req=hook_req,
            status_code=status_code,
            json_predicate=json_predicate)

    def __enter__(self) -> typing.Self:
        return self

    def __exit__(
            self,
            exc_type: type[BaseException] | None,
            exc_val: BaseException | None,
            exc_tb: TracebackType | None,
    ) -> None:
        # Clear temporary data first, it better be cleaned if anything fails
        self._clear_ret_datas()
        req = self._client.sol_command_batch_request(sol_id=self._sol_id, commands=self._commands)
        resp = self._process_request(req=req)
        # In case of successful response, update entity data
        if resp.status_code == 200:
            self._fill_entity_data(resp_data=resp.json())

    # Entity making methods are supposed to be called after command has been added
    def _make_fleet(self) -> Fleet:
        index = len(self._commands) - 1
        data = {'id': f'#{index}'}
        self._ret_datas[index] = EntityData(kind=DataFillKind.id_regular, data=data)
        return Fleet(client=self._client, data=data, sol_id=self._sol_id)

    def _make_fit(self) -> Fit:
        index = len(self._commands) - 1
        data = {'id': f'#{index}'}
        self._ret_datas[index] = EntityData(kind=DataFillKind.id_regular, data=data)
        return Fit(client=self._client, data=data, sol_id=self._sol_id)

    def _make_sol_info(self) -> SolarSystem:
        from fw.api.types.sol import SolarSystem  # ruff:ignore[import-outside-top-level]
        index = len(self._commands) - 1
        data = {}
        self._ret_datas[index] = EntityData(kind=DataFillKind.copy_map, data=data)
        return SolarSystem(client=self._client, data=data)

    def _make_fleet_info(self) -> Fleet:
        index = len(self._commands) - 1
        data = {}
        self._ret_datas[index] = EntityData(kind=DataFillKind.copy_map, data=data)
        return Fleet(client=self._client, data=data, sol_id=self._sol_id)

    ################################################################################################
    # Control
    ################################################################################################
    # Sol
    def change_sol(
            self, *,
            sec_zone: ApiSecZone | type[Absent] = Absent,
            default_incoming_dps: DpsProfileAlias | type[Absent] = Absent,
            default_spool: str | type[Absent] = Absent,
            default_npc_prop: ApiNpcProp | type[Absent] = Absent,
            default_optional_reloads: ApiOptionalReload | type[Absent] = Absent,
            default_rearm_minions: ApiRearmMinion | type[Absent] = Absent,
    ) -> None:
        command = SolCtlSolChangeCmd(
            sec_zone=sec_zone,
            default_incoming_dps=default_incoming_dps,
            default_spool=default_spool,
            default_npc_prop=default_npc_prop,
            default_optional_reloads=default_optional_reloads,
            default_rearm_minions=default_rearm_minions)
        self._commands.append(command)

    # Fleet
    def create_fleet(
            self, *,
            fit_ids: list[str] | type[Absent] = Absent,
    ) -> Fleet:
        command = SolCtlFleetAddCmd(fit_ids=fit_ids)
        self._commands.append(command)
        return self._make_fleet()

    def change_fleet(
            self, *,
            fleet_id: str,
            add_fit_ids: list[str] | type[Absent] = Absent,
            rm_fit_ids: list[str] | type[Absent] = Absent,
    ) -> None:
        command = SolCtlFleetChangeCmd(
            fleet_id=fleet_id,
            add_fit_ids=add_fit_ids,
            rm_fit_ids=rm_fit_ids)
        self._commands.append(command)

    def remove_fleet(self, *, fleet_id: str) -> None:
        command = SolCtlFleetRemoveCmd(fleet_id=fleet_id)
        self._commands.append(command)

    # Fit
    def create_fit(
            self, *,
            fleet_id: str | type[Absent] = Absent,
            sec_status: float | type[Absent] = Absent,
            rah_incoming_dps: DpsProfileAlias | type[Absent] = Absent,
    ) -> Fit:
        command = SolCtlFitAddCmd(
            fleet_id=fleet_id,
            sec_status=sec_status,
            rah_incoming_dps=rah_incoming_dps)
        self._commands.append(command)
        return self._make_fit()

    def change_fit(
            self, *,
            fit_id: str,
            fleet_id: str | type[Absent] = Absent,
            sec_status: float | type[Absent] = Absent,
            rah_incoming_dps: DpsProfileAlias | type[Absent] = Absent,
    ) -> None:
        command = SolCtlFitChangeCmd(
            fit_id=fit_id,
            fleet_id=fleet_id,
            sec_status=sec_status,
            rah_incoming_dps=rah_incoming_dps)
        self._commands.append(command)

    def remove_fit(self, *, fit_id: str) -> None:
        command = SolCtlFitRemoveCmd(fit_id=fit_id)
        self._commands.append(command)

    # Item
    def remove_item(
            self, *,
            item_id: str,
            rm_mode: ApiModRmMode | type[Absent] = Absent,
    ) -> None:
        command = SolCtlItemRemoveCmd(
            item_id=item_id,
            rm_mode=rm_mode)
        self._commands.append(command)

    # Item - autocharge
    def change_autocharge(
            self, *,
            item_id: str,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = SolCtlAutochargeChangeCmd(
            item_id=item_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    # Item - booster
    def add_booster(
            self, *,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent] = Absent,
            side_effects: dict[int | str, bool] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolCtlBoosterAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            side_effects=side_effects,
            effect_modes=effect_modes)
        self._commands.append(command)
        return self._make_item()

    def change_booster(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            side_effects: dict[int | str, bool] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = SolCtlBoosterChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            side_effects=side_effects,
            effect_modes=effect_modes)
        self._commands.append(command)

    # Item - character
    def set_character(
            self, *,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolCtlCharacterSetCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)
        return self._make_item()

    def change_character_via_fit_id(
            self, *,
            fit_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = SolCtlCharacterChangeViaFitIdCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    def change_character_via_item_id(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = SolCtlCharacterChangeViaItemIdCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    def unset_character(self, *, fit_id: str) -> None:
        command = SolCtlCharacterUnsetCmd(fit_id=fit_id)
        self._commands.append(command)

    # Item - charge
    def change_charge(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = SolCtlChargeChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    # Item - drone
    def add_drone(
            self, *,
            fit_id: str,
            type_id: int,
            state: ApiMinionState = ApiMinionState.in_bay,
            mutation: MutaAdd | type[Absent] = Absent,
            npc_prop_override: ApiNpcProp | type[Absent] = Absent,
            proj_item_ids: list[str] | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolCtlDroneAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            mutation=mutation,
            npc_prop_override=npc_prop_override,
            proj_item_ids=proj_item_ids,
            coordinates=coordinates,
            movement=movement,
            effect_modes=effect_modes)
        self._commands.append(command)
        return self._make_item()

    def change_drone(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: ApiMinionState | type[Absent] = Absent,
            mutation: MutaAdd | MutaChange | type[Absent] | None = Absent,
            npc_prop_override: ApiNpcProp | type[Absent] | None = Absent,
            add_proj_item_ids: list[str] | type[Absent] = Absent,
            rm_proj_item_ids: list[str] | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = SolCtlDroneChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            mutation=mutation,
            npc_prop_override=npc_prop_override,
            add_proj_item_ids=add_proj_item_ids,
            rm_proj_item_ids=rm_proj_item_ids,
            coordinates=coordinates,
            movement=movement,
            effect_modes=effect_modes)
        self._commands.append(command)

    # Item - fighter
    def add_fighter(
            self, *,
            fit_id: str,
            type_id: int,
            state: ApiMinionState = ApiMinionState.in_bay,
            count_override: int | type[Absent] = Absent,
            abilities: dict[int, bool] | type[Absent] = Absent,
            rearm_minion_override: ApiRearmMinion | type[Absent] = Absent,
            proj_item_ids: list[str] | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolCtlFighterAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            count_override=count_override,
            abilities=abilities,
            rearm_minion_override=rearm_minion_override,
            proj_item_ids=proj_item_ids,
            coordinates=coordinates,
            movement=movement,
            effect_modes=effect_modes)
        self._commands.append(command)
        return self._make_item()

    def change_fighter(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: ApiMinionState | type[Absent] = Absent,
            count_override: int | type[Absent] | None = Absent,
            abilities: dict[int, bool] | type[Absent] = Absent,
            rearm_minion_override: ApiRearmMinion | type[Absent] | None = Absent,
            add_proj_item_ids: list[str] | type[Absent] = Absent,
            rm_proj_item_ids: list[str] | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = SolCtlFighterChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            count_override=count_override,
            abilities=abilities,
            rearm_minion_override=rearm_minion_override,
            add_proj_item_ids=add_proj_item_ids,
            rm_proj_item_ids=rm_proj_item_ids,
            coordinates=coordinates,
            movement=movement,
            effect_modes=effect_modes)
        self._commands.append(command)

    # Item - fit-wide effect
    def add_fw_effect(
            self, *,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolCtlFwEffectAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)
        return self._make_item()

    def change_fw_effect(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = SolCtlFwEffectChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    # Item - implant
    def add_implant(
            self, *,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolCtlImplantAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)
        return self._make_item()

    def change_implant(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = SolCtlImplantChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    # Item - module
    def add_module(
            self, *,
            fit_id: str,
            type_id: int,
            rack: ApiRack = ApiRack.high,
            add_mode: ApiModAddMode | dict[ApiModAddMode, int] | type[Absent] = ApiModAddMode.equip,
            state: ApiModuleState = ApiModuleState.offline,
            mutation: MutaAdd | type[Absent] = Absent,
            charge_type_id: int | type[Absent] = Absent,
            spool_override: str | type[Absent] = Absent,
            optional_reload_override: ApiOptionalReload | type[Absent] = Absent,
            proj_item_ids: list[str] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolCtlModuleAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            rack=rack,
            add_mode=add_mode,
            state=state,
            mutation=mutation,
            charge_type_id=charge_type_id,
            spool_override=spool_override,
            optional_reload_override=optional_reload_override,
            proj_item_ids=proj_item_ids,
            effect_modes=effect_modes)
        self._commands.append(command)
        return self._make_item()

    def change_module(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            move: ApiModMvMode | dict[ApiModMvMode, int] | type[Absent] = Absent,
            state: ApiModuleState | type[Absent] = Absent,
            mutation: MutaAdd | MutaChange | type[Absent] | None = Absent,
            charge_type_id: int | type[Absent] | None = Absent,
            spool_override: str | type[Absent] | None = Absent,
            optional_reload_override: ApiOptionalReload | type[Absent] | None = Absent,
            add_proj_item_ids: list[str] | type[Absent] = Absent,
            rm_proj_item_ids: list[str] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolCtlModuleChangeCmd(
            item_id=item_id,
            type_id=type_id,
            move=move,
            state=state,
            mutation=mutation,
            charge_type_id=charge_type_id,
            spool_override=spool_override,
            optional_reload_override=optional_reload_override,
            add_proj_item_ids=add_proj_item_ids,
            rm_proj_item_ids=rm_proj_item_ids,
            effect_modes=effect_modes)
        self._commands.append(command)
        return self._make_item_charge()

    # Item - projected effect
    def add_proj_effect(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            proj_item_ids: list[str] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolCtlProjEffectAddCmd(
            type_id=type_id,
            state=state,
            proj_item_ids=proj_item_ids,
            effect_modes=effect_modes)
        self._commands.append(command)
        return self._make_item()

    def change_proj_effect(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            add_proj_item_ids: list[str] | type[Absent] = Absent,
            rm_proj_item_ids: list[str] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = SolCtlProjEffectChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            add_proj_item_ids=add_proj_item_ids,
            rm_proj_item_ids=rm_proj_item_ids,
            effect_modes=effect_modes)
        self._commands.append(command)

    # Item - rig
    def add_rig(
            self, *,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolCtlRigAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)
        return self._make_item()

    def change_rig(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = SolCtlRigChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    # Item - service
    def add_service(
            self, *,
            fit_id: str,
            type_id: int,
            state: ApiServiceState = ApiServiceState.offline,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolCtlServiceAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)
        return self._make_item()

    def change_service(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: ApiServiceState | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = SolCtlServiceChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    # Item - ship
    def set_ship(
            self, *,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolCtlShipSetCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            coordinates=coordinates,
            movement=movement,
            effect_modes=effect_modes)
        self._commands.append(command)
        return self._make_item()

    def change_ship_via_fit_id(
            self, *,
            fit_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = SolCtlShipChangeViaFitIdCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            coordinates=coordinates,
            movement=movement,
            effect_modes=effect_modes)
        self._commands.append(command)

    def change_ship_via_item_id(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = SolCtlShipChangeViaItemIdCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            coordinates=coordinates,
            movement=movement,
            effect_modes=effect_modes)
        self._commands.append(command)

    def unset_ship(self, *, fit_id: str) -> None:
        command = SolCtlShipUnsetCmd(fit_id=fit_id)
        self._commands.append(command)

    # Item - skill
    def add_skill(
            self, *,
            fit_id: str,
            type_id: int,
            level: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolCtlSkillAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            level=level,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)
        return self._make_item()

    def change_skill(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            level: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = SolCtlSkillChangeCmd(
            item_id=item_id,
            type_id=type_id,
            level=level,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    # Item - stance
    def set_stance(
            self, *,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolCtlStanceSetCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)
        return self._make_item()

    def change_stance_via_fit_id(
            self, *,
            fit_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = SolCtlStanceChangeViaFitIdCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    def change_stance_via_item_id(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = SolCtlStanceChangeViaItemIdCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    def unset_stance(self, *, fit_id: str) -> None:
        command = SolCtlStanceUnsetCmd(fit_id=fit_id)
        self._commands.append(command)

    # Item - subsystem
    def add_subsystem(
            self, *,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolCtlSubsystemAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)
        return self._make_item()

    def change_subsystem(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = SolCtlSubsystemChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    # Item - system-wide effect
    def add_sw_effect(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolCtlSwEffectAddCmd(
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)
        return self._make_item()

    def change_sw_effect(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = SolCtlSwEffectChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    ################################################################################################
    # Info
    ################################################################################################
    def get_sol_info(
            self, *,
            sol_mode: InfoMode[ApiSolInfoMode] | type[Absent] = Absent,
            fleet_mode: InfoMode[ApiFleetInfoMode] | type[Absent] = Absent,
            fit_mode: InfoMode[ApiFitInfoMode] | type[Absent] = Absent,
            item_mode: InfoMode[ApiItemInfoMode] | type[Absent] = Absent,
    ) -> SolarSystem:
        command = SolInfoSolCmd(
            sol_mode=sol_mode,
            fleet_mode=fleet_mode,
            fit_mode=fit_mode,
            item_mode=item_mode)
        self._commands.append(command)
        return self._make_sol_info()

    def get_fleet_info(
            self, *,
            fleet_id: str,
            fleet_mode: InfoMode[ApiFleetInfoMode] | type[Absent] = Absent,
    ) -> Fleet:
        command = SolInfoFleetCmd(
            fleet_id=fleet_id,
            fleet_mode=fleet_mode)
        self._commands.append(command)
        return self._make_fleet_info()

    def get_fit_info(
            self, *,
            fit_id: str,
            fit_mode: InfoMode[ApiFitInfoMode] | type[Absent] = Absent,
            item_mode: InfoMode[ApiItemInfoMode] | type[Absent] = Absent,
    ) -> Fit:
        command = SolInfoFitCmd(
            fit_id=fit_id,
            fit_mode=fit_mode,
            item_mode=item_mode)
        self._commands.append(command)
        return self._make_fit_info()

    def get_item_info(
            self, *,
            item_id: str,
            item_mode: InfoMode[ApiItemInfoMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolInfoItemCmd(
            item_id=item_id,
            item_mode=item_mode)
        self._commands.append(command)
        return self._make_item_info()

    ################################################################################################
    # Stats
    ################################################################################################
    def get_sol_stats(
            self, *,
            fleet_options: StatsOptions[FleetStatsOptions] | type[Absent] = Absent,
            fit_options: StatsOptions[FitStatsOptions] | type[Absent] = Absent,
            item_options: StatsOptions[ItemStatsOptions] | type[Absent] = Absent,
    ) -> SolBatchStats:
        command = SolStatsSolCmd(
            fleet_options=fleet_options,
            fit_options=fit_options,
            item_options=item_options)
        self._commands.append(command)
        return self._make_stats(cls=SolBatchStats)

    def get_fleet_stats(
            self, *,
            fleet_id: str,
            fleet_options: FleetStatsOptions | type[Absent] = Absent,
            fit_options: StatsOptions[FitStatsOptions] | type[Absent] = Absent,
            item_options: StatsOptions[ItemStatsOptions] | type[Absent] = Absent,
    ) -> FleetBatchStats:
        command = SolStatsFleetCmd(
            fleet_id=fleet_id,
            fleet_options=fleet_options,
            fit_options=fit_options,
            item_options=item_options)
        self._commands.append(command)
        return self._make_stats(cls=FleetBatchStats)

    def get_fit_stats(
            self, *,
            fit_id: str,
            fit_options: FitStatsOptions | type[Absent] = Absent,
            item_options: StatsOptions[ItemStatsOptions] | type[Absent] = Absent,
    ) -> FitBatchStats:
        command = SolStatsFitCmd(
            fit_id=fit_id,
            fit_options=fit_options,
            item_options=item_options)
        self._commands.append(command)
        return self._make_stats(cls=FitBatchStats)

    def get_item_stats(
            self, *,
            item_id: str,
            item_options: ItemStatsOptions | type[Absent] = Absent,
    ) -> ItemBatchStats:
        command = SolStatsItemCmd(
            item_id=item_id,
            item_options=item_options)
        self._commands.append(command)
        return self._make_stats(cls=ItemBatchStats)

    ################################################################################################
    # Validation
    ################################################################################################
    def validate_sol(
            self, *,
            options: ValOptions | type[Absent] = Absent,
            fit_ids: list[str] | type[Absent] = Absent,
            info_mode: ApiValInfoMode | type[Absent] = Absent,
    ) -> SolValResult:
        command = SolValSolCmd(
            options=options,
            fit_ids=fit_ids,
            info_mode=info_mode)
        self._commands.append(command)
        return self._make_val_result(cls=SolValResult)

    def validate_fit(
            self, *,
            fit_id: str,
            options: ValOptions | type[Absent] = Absent,
            info_mode: ApiValInfoMode | type[Absent] = Absent,
    ) -> FitValResult:
        command = SolValFitCmd(
            fit_id=fit_id,
            options=options,
            info_mode=info_mode)
        self._commands.append(command)
        return self._make_val_result(cls=FitValResult)

    ################################################################################################
    # Try items
    ################################################################################################
    def try_fit_items(
            self, *,
            fit_id: str,
            type_ids: list[int],
            val_options: ValOptions | type[Absent] = Absent,
    ) -> list[int]:
        command = SolTryItemsFitCmd(
            fit_id=fit_id,
            type_ids=type_ids,
            val_options=val_options)
        self._commands.append(command)
        return self._make_try_items()
