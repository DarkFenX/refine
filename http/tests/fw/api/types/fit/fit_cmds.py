import typing

from fw.api.commands import (
    FitAutochargeChangeCmd,
    FitBoosterAddCmd,
    FitBoosterChangeCmd,
    FitCharacterChangeCmd,
    FitCharacterSetCmd,
    FitCharacterUnsetCmd,
    FitChargeChangeCmd,
    FitDroneAddCmd,
    FitDroneChangeCmd,
    FitFighterAddCmd,
    FitFighterChangeCmd,
    FitFwEffectAddCmd,
    FitFwEffectChangeCmd,
    FitImplantAddCmd,
    FitImplantChangeCmd,
    FitItemRemoveCmd,
    FitModuleAddCmd,
    FitModuleChangeCmd,
    FitRigAddCmd,
    FitRigChangeCmd,
    FitServiceAddCmd,
    FitServiceChangeCmd,
    FitShipChangeCmd,
    FitShipSetCmd,
    FitShipUnsetCmd,
    FitSkillAddCmd,
    FitSkillChangeCmd,
    FitStanceChangeCmd,
    FitStanceSetCmd,
    FitStanceUnsetCmd,
    FitSubsystemAddCmd,
    FitSubsystemChangeCmd,
)
from fw.api.types.helpers import process_effect_map_request, process_muta_add_request, process_muta_change_request
from fw.api.types.item import Item
from fw.consts import ApiMinionState, ApiModAddMode, ApiModuleState, ApiRack, ApiServiceState
from fw.util import Absent

if typing.TYPE_CHECKING:
    from types import TracebackType

    from fw.api import ApiClient
    from fw.api.aliases import MutaAdd, MutaChange
    from fw.api.commands import BaseCommand
    from fw.consts import (
        ApiEffMode,
        ApiFitInfoMode,
        ApiItemInfoMode,
        ApiModRmMode,
        ApiNpcProp,
        ApiOptionalReload,
        ApiRearmMinion,
    )
    from .fit import Fit


class FitCmdCtx:

    def __init__(
            self, *,
            client: ApiClient,
            fit: Fit,
            sol_id: str,
            fit_id: str,
            fit_info_mode: ApiFitInfoMode | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
            status_code: int,
            json_predicate: dict | None,
    ) -> None:
        self._client = client
        self._fit = fit
        self._sol_id = sol_id
        self._fit_id = fit_id
        self._fit_info_mode = fit_info_mode
        self._item_info_mode = item_info_mode
        self._status_code = status_code
        self._json_predicate = json_predicate
        self._commands: list[BaseCommand] = []
        self._ret_datas: dict[int, dict] = {}

    def __enter__(self) -> typing.Self:
        return self

    def __exit__(
            self,
            exc_type: type[BaseException] | None,
            exc_val: BaseException | None,
            exc_tb: TracebackType | None,
    ) -> None:
        # Clear temporary data first, it better be cleaned if anything fails
        for entity_data in self._ret_datas.values():
            entity_data.clear()
        resp = self._client.fit_commands_request(
            sol_id=self._sol_id,
            fit_id=self._fit_id,
            commands=self._commands,
            fit_info_mode=self._fit_info_mode,
            item_info_mode=self._item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=self._status_code, json_predicate=self._json_predicate)
        # In case of successful response, update entity data
        if resp.status_code == 200:
            resp_data = resp.json()
            # Fit which initiated the command chain
            self._fit._data = resp_data['fit']  # noqa: SLF001
            # Update IDs in all the entities which were created by the commands
            for i, cmd_result in enumerate(resp_data['cmd_results']):
                if i not in self._ret_datas:
                    continue
                entity_data = self._ret_datas[i]
                if 'id' in cmd_result:
                    entity_data['id'] = cmd_result['id']
                if 'charge_id' in cmd_result:
                    entity_data['charge'] = {'id': cmd_result['charge_id']}

    def __make_item(self) -> Item:
        # It is supposed to be called after command has been added
        index = len(self._commands) - 1
        data = {'id': f'#{index}', 'charge': {'id': f'#{index}c'}}
        self._ret_datas[index] = data
        return Item(client=self._client, data=data, sol_id=self._sol_id)

    # Item
    def remove_item(
            self, *,
            item_id: str,
            rm_mode: ApiModRmMode | type[Absent] = Absent,
    ) -> None:
        command = FitItemRemoveCmd(
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
        command = FitAutochargeChangeCmd(
            item_id=item_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)

    # Item - booster
    def add_booster(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            side_effects: dict[int | str, bool] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitBoosterAddCmd(
            type_id=type_id,
            state=state,
            side_effects=process_effect_map_request(effect_map=side_effects),
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
        return self.__make_item()

    def change_booster(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            side_effects: dict[int | str, bool] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = FitBoosterChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            side_effects=process_effect_map_request(effect_map=side_effects),
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)

    # Item - character
    def set_character(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitCharacterSetCmd(
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
        return self.__make_item()

    def change_character(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = FitCharacterChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)

    def unset_character(self) -> None:
        command = FitCharacterUnsetCmd()
        self._commands.append(command)

    # Item - charge
    def change_charge(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = FitChargeChangeCmd(
            type_id=type_id,
            item_id=item_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)

    # Item - drone
    def add_drone(
            self, *,
            type_id: int,
            state: ApiMinionState = ApiMinionState.in_bay,
            mutation: MutaAdd | type[Absent] = Absent,
            npc_prop: ApiNpcProp | type[Absent] = Absent,
            proj_item_ids: list[str] | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitDroneAddCmd(
            type_id=type_id,
            state=state,
            mutation=process_muta_add_request(mutation=mutation),
            npc_prop=npc_prop,
            proj_item_ids=proj_item_ids,
            coordinates=coordinates,
            movement=movement,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
        return self.__make_item()

    def change_drone(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: ApiMinionState | type[Absent] = Absent,
            mutation: MutaAdd | MutaChange | type[Absent] | None = Absent,
            npc_prop: ApiNpcProp | type[Absent] | None = Absent,
            add_proj_item_ids: list[str] | type[Absent] = Absent,
            rm_proj_item_ids: list[str] | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = FitDroneChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            mutation=process_muta_change_request(mutation=mutation),
            npc_prop=npc_prop,
            add_proj_item_ids=add_proj_item_ids,
            rm_proj_item_ids=rm_proj_item_ids,
            coordinates=coordinates,
            movement=movement,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)

    # Item - fighter
    def add_fighter(
            self, *,
            type_id: int,
            state: ApiMinionState = ApiMinionState.in_bay,
            count: int | type[Absent] = Absent,
            abilities: dict[int, bool] | type[Absent] = Absent,
            rearm_minion: ApiRearmMinion | type[Absent] = Absent,
            proj_item_ids: list[str] | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitFighterAddCmd(
            type_id=type_id,
            state=state,
            count=count,
            abilities=abilities,
            rearm_minion=rearm_minion,
            proj_item_ids=proj_item_ids,
            coordinates=coordinates,
            movement=movement,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
        return self.__make_item()

    def change_fighter(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: ApiMinionState | type[Absent] = Absent,
            count: int | type[Absent] | None = Absent,
            abilities: dict[int, bool] | type[Absent] = Absent,
            rearm_minion: ApiRearmMinion | type[Absent] | None = Absent,
            add_proj_item_ids: list[str] | type[Absent] = Absent,
            rm_proj_item_ids: list[str] | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = FitFighterChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            count=count,
            abilities=abilities,
            rearm_minion=rearm_minion,
            add_proj_item_ids=add_proj_item_ids,
            rm_proj_item_ids=rm_proj_item_ids,
            coordinates=coordinates,
            movement=movement,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)

    # Item - fit-wide effect
    def add_fw_effect(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitFwEffectAddCmd(
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
        return self.__make_item()

    def change_fw_effect(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = FitFwEffectChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)

    # Item - implant
    def add_implant(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitImplantAddCmd(
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
        return self.__make_item()

    def change_implant(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = FitImplantChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
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
            optional_reload: ApiOptionalReload | type[Absent] = Absent,
            proj_item_ids: list[str] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitModuleAddCmd(
            type_id=type_id,
            rack=rack,
            add_mode=add_mode,
            state=state,
            mutation=process_muta_add_request(mutation=mutation),
            charge_type_id=charge_type_id,
            spool=spool,
            optional_reload=optional_reload,
            proj_item_ids=proj_item_ids,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
        return self.__make_item()

    def change_module(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: ApiModuleState | type[Absent] = Absent,
            mutation: MutaAdd | MutaChange | type[Absent] | None = Absent,
            charge_type_id: int | type[Absent] | None = Absent,
            spool: str | type[Absent] | None = Absent,
            optional_reload: ApiOptionalReload | type[Absent] | None = Absent,
            add_proj_item_ids: list[str] | type[Absent] = Absent,
            rm_proj_item_ids: list[str] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = FitModuleChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            mutation=process_muta_change_request(mutation=mutation),
            charge_type_id=charge_type_id,
            spool=spool,
            optional_reload=optional_reload,
            add_proj_item_ids=add_proj_item_ids,
            rm_proj_item_ids=rm_proj_item_ids,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)

    # Item - rig
    def add_rig(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitRigAddCmd(
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
        return self.__make_item()

    def change_rig(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = FitRigChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)

    # Item - service
    def add_service(
            self, *,
            type_id: int,
            state: ApiServiceState = ApiServiceState.offline,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitServiceAddCmd(
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
        return self.__make_item()

    def change_service(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: ApiServiceState | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = FitServiceChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
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
        command = FitShipSetCmd(
            type_id=type_id,
            state=state,
            coordinates=coordinates,
            movement=movement,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
        return self.__make_item()

    def change_ship(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = FitShipChangeCmd(
            type_id=type_id,
            state=state,
            coordinates=coordinates,
            movement=movement,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)

    def unset_ship(self) -> None:
        command = FitShipUnsetCmd()
        self._commands.append(command)

    # Item - skill
    def add_skill(
            self, *,
            type_id: int,
            level: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitSkillAddCmd(
            type_id=type_id,
            level=level,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
        return self.__make_item()

    def change_skill(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            level: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = FitSkillChangeCmd(
            item_id=item_id,
            type_id=type_id,
            level=level,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)

    # Item - stance
    def set_stance(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitStanceSetCmd(
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
        return self.__make_item()

    def change_stance(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = FitStanceChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)

    def unset_stance(self) -> None:
        command = FitStanceUnsetCmd()
        self._commands.append(command)

    # Item - subsystem
    def add_subsystem(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = FitSubsystemAddCmd(
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
        return self.__make_item()

    def change_subsystem(
            self, *,
            item_id: str,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = FitSubsystemChangeCmd(
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
