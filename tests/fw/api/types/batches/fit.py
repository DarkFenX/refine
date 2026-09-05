import typing

from fw.api.commands import (
    FitCtlAutochargeChangeCmd,
    FitCtlBoosterAddCmd,
    FitCtlBoosterChangeCmd,
    FitCtlCharacterChangeCmd,
    FitCtlCharacterSetCmd,
    FitCtlCharacterUnsetCmd,
    FitCtlChargeChangeCmd,
    FitCtlDroneAddCmd,
    FitCtlDroneChangeCmd,
    FitCtlFighterAddCmd,
    FitCtlFighterChangeCmd,
    FitCtlFitChangeCmd,
    FitCtlFwEffectAddCmd,
    FitCtlFwEffectChangeCmd,
    FitCtlImplantAddCmd,
    FitCtlImplantChangeCmd,
    FitCtlItemRemoveCmd,
    FitCtlModuleAddCmd,
    FitCtlModuleChangeCmd,
    FitCtlRigAddCmd,
    FitCtlRigChangeCmd,
    FitCtlServiceAddCmd,
    FitCtlServiceChangeCmd,
    FitCtlShipChangeCmd,
    FitCtlShipSetCmd,
    FitCtlShipUnsetCmd,
    FitCtlSkillAddCmd,
    FitCtlSkillChangeCmd,
    FitCtlStanceChangeCmd,
    FitCtlStanceSetCmd,
    FitCtlStanceUnsetCmd,
    FitCtlSubsystemAddCmd,
    FitCtlSubsystemChangeCmd,
    FitInfoFitCmd,
    FitInfoItemCmd,
    FitStatsFitCmd,
    FitStatsItemCmd,
    FitTryItemsFitCmd,
    FitValFitCmd,
)
from fw.api.types.stats import FitBatchStats, ItemBatchStats
from fw.api.types.validation import FitValResult
from fw.consts import ApiMinionState, ApiModAddMode, ApiModuleState, ApiRack, ApiServiceState
from fw.util import Absent
from .base import BaseCmdBatchCtx

if typing.TYPE_CHECKING:
    from types import TracebackType

    from fw.api import ApiClient
    from fw.api.aliases import DpsProfileAlias, InfoMode, MutaAdd, MutaChange, ReqHook, StatsOptions
    from fw.api.types.fit import Fit
    from fw.api.types.item import Item
    from fw.api.types.stats import FitStatsOptions, ItemStatsOptions
    from fw.api.types.validation import ValOptions
    from fw.consts import (
        ApiEffMode,
        ApiFitInfoMode,
        ApiItemInfoMode,
        ApiModMvMode,
        ApiModRmMode,
        ApiNpcProp,
        ApiOptionalReload,
        ApiRearmMinion,
        ApiValInfoMode,
    )


class FitCmdBatchCtx(BaseCmdBatchCtx):

    def __init__(
            self, *,
            client: ApiClient,
            sol_id: str,
            fit_id: str,
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
        self._fit_id = fit_id

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
        req = self._client.fit_command_batch_request(
            sol_id=self._sol_id,
            fit_id=self._fit_id,
            commands=self._commands)
        resp = self._process_request(req=req)
        # In case of successful response, update entity data
        if resp.status_code == 200:
            self._fill_entity_data(resp_data=resp.json())

    ################################################################################################
    # Control
    ################################################################################################
    # Fit
    def change_fit(
            self, *,
            fleet_id: str | type[Absent] = Absent,
            sec_status: float | type[Absent] = Absent,
            rah_incoming_dps: DpsProfileAlias | type[Absent] = Absent,
    ) -> None:
        command = FitCtlFitChangeCmd(
            fleet_id=fleet_id,
            sec_status=sec_status,
            rah_incoming_dps=rah_incoming_dps)
        self._commands.append(command)

    # Item
    def remove_item(
            self, *,
            item_id: str,
            rm_mode: ApiModRmMode | type[Absent] = Absent,
    ) -> None:
        command = FitCtlItemRemoveCmd(
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
        command = FitCtlAutochargeChangeCmd(
            item_id=item_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    # Item - booster
    def add_booster(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            side_effects: dict[int | str, bool] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitCtlBoosterAddCmd(
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
        command = FitCtlBoosterChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            side_effects=side_effects,
            effect_modes=effect_modes)
        self._commands.append(command)

    # Item - character
    def set_character(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitCtlCharacterSetCmd(
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)
        return self._make_item()

    def change_character(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = FitCtlCharacterChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    def unset_character(self) -> None:
        command = FitCtlCharacterUnsetCmd()
        self._commands.append(command)

    # Item - charge
    def change_charge(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = FitCtlChargeChangeCmd(
            type_id=type_id,
            item_id=item_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    # Item - drone
    def add_drone(
            self, *,
            type_id: int,
            state: ApiMinionState = ApiMinionState.in_bay,
            mutation: MutaAdd | type[Absent] = Absent,
            npc_prop_override: ApiNpcProp | type[Absent] = Absent,
            proj_item_ids: list[str] | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitCtlDroneAddCmd(
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
        command = FitCtlDroneChangeCmd(
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
        command = FitCtlFighterAddCmd(
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
        command = FitCtlFighterChangeCmd(
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
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitCtlFwEffectAddCmd(
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
        command = FitCtlFwEffectChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    # Item - implant
    def add_implant(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitCtlImplantAddCmd(
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
        command = FitCtlImplantChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    # Item - module
    def add_module(
            self, *,
            type_id: int,
            rack: ApiRack = ApiRack.high,
            add_mode: ApiModAddMode | dict[ApiModAddMode, int] | type[Absent] = ApiModAddMode.equip,
            state: ApiModuleState = ApiModuleState.offline,
            mutation: MutaAdd | type[Absent] = Absent,
            charge_type_id: int | type[Absent] = Absent,
            spool: str | type[Absent] = Absent,
            optional_reload_override: ApiOptionalReload | type[Absent] = Absent,
            proj_item_ids: list[str] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitCtlModuleAddCmd(
            type_id=type_id,
            rack=rack,
            add_mode=add_mode,
            state=state,
            mutation=mutation,
            charge_type_id=charge_type_id,
            spool=spool,
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
            spool: str | type[Absent] | None = Absent,
            optional_reload_override: ApiOptionalReload | type[Absent] | None = Absent,
            add_proj_item_ids: list[str] | type[Absent] = Absent,
            rm_proj_item_ids: list[str] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitCtlModuleChangeCmd(
            item_id=item_id,
            type_id=type_id,
            move=move,
            state=state,
            mutation=mutation,
            charge_type_id=charge_type_id,
            spool=spool,
            optional_reload_override=optional_reload_override,
            add_proj_item_ids=add_proj_item_ids,
            rm_proj_item_ids=rm_proj_item_ids,
            effect_modes=effect_modes)
        self._commands.append(command)
        return self._make_item_charge()

    # Item - rig
    def add_rig(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitCtlRigAddCmd(
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
        command = FitCtlRigChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    # Item - service
    def add_service(
            self, *,
            type_id: int,
            state: ApiServiceState = ApiServiceState.offline,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitCtlServiceAddCmd(
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
        command = FitCtlServiceChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    # Item - ship
    def set_ship(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitCtlShipSetCmd(
            type_id=type_id,
            state=state,
            coordinates=coordinates,
            movement=movement,
            effect_modes=effect_modes)
        self._commands.append(command)
        return self._make_item()

    def change_ship(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = FitCtlShipChangeCmd(
            type_id=type_id,
            state=state,
            coordinates=coordinates,
            movement=movement,
            effect_modes=effect_modes)
        self._commands.append(command)

    def unset_ship(self) -> None:
        command = FitCtlShipUnsetCmd()
        self._commands.append(command)

    # Item - skill
    def add_skill(
            self, *,
            type_id: int,
            level: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitCtlSkillAddCmd(
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
        command = FitCtlSkillChangeCmd(
            item_id=item_id,
            type_id=type_id,
            level=level,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    # Item - stance
    def set_stance(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitCtlStanceSetCmd(
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)
        return self._make_item()

    def change_stance(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = FitCtlStanceChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    def unset_stance(self) -> None:
        command = FitCtlStanceUnsetCmd()
        self._commands.append(command)

    # Item - subsystem
    def add_subsystem(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitCtlSubsystemAddCmd(
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
        command = FitCtlSubsystemChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes)
        self._commands.append(command)

    ################################################################################################
    # Info
    ################################################################################################
    def get_fit_info(
            self, *,
            fit_mode: InfoMode[ApiFitInfoMode] | type[Absent] = Absent,
            item_mode: InfoMode[ApiItemInfoMode] | type[Absent] = Absent,
    ) -> Fit:
        command = FitInfoFitCmd(
            fit_mode=fit_mode,
            item_mode=item_mode)
        self._commands.append(command)
        return self._make_fit_info()

    def get_item_info(
            self, *,
            item_id: str,
            item_mode: InfoMode[ApiItemInfoMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitInfoItemCmd(
            item_id=item_id,
            item_mode=item_mode)
        self._commands.append(command)
        return self._make_item_info()

    ################################################################################################
    # Stats
    ################################################################################################
    def get_fit_stats(
            self, *,
            fit_options: FitStatsOptions | type[Absent] = Absent,
            item_options: StatsOptions[ItemStatsOptions] | type[Absent] = Absent,
    ) -> FitBatchStats:
        command = FitStatsFitCmd(
            fit_options=fit_options,
            item_options=item_options)
        self._commands.append(command)
        return self._make_stats(cls=FitBatchStats)

    def get_item_stats(
            self, *,
            item_id: str,
            item_options: ItemStatsOptions | type[Absent] = Absent,
    ) -> ItemBatchStats:
        command = FitStatsItemCmd(
            item_id=item_id,
            item_options=item_options)
        self._commands.append(command)
        return self._make_stats(cls=ItemBatchStats)

    ################################################################################################
    # Validation
    ################################################################################################
    def validate_fit(
            self, *,
            options: ValOptions | type[Absent] = Absent,
            info_mode: ApiValInfoMode | type[Absent] = Absent,
    ) -> FitValResult:
        command = FitValFitCmd(
            options=options,
            info_mode=info_mode)
        self._commands.append(command)
        return self._make_val_result(cls=FitValResult)

    ################################################################################################
    # Try items
    ################################################################################################
    def try_fit_items(
            self, *,
            type_ids: list[int],
            val_options: ValOptions | type[Absent] = Absent,
    ) -> list[int]:
        command = FitTryItemsFitCmd(
            type_ids=type_ids,
            val_options=val_options)
        self._commands.append(command)
        return self._make_try_items()
