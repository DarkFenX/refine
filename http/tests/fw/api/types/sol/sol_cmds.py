import typing

from fw.api.commands import (
    SolBoosterAddCmd,
    SolCharacterSetCmd,
    SolDroneAddCmd,
    SolFitAddCmd,
    SolFitRemoveCmd,
    SolImplantAddCmd,
    SolItemRemoveCmd,
    SolModuleAddCmd,
    SolModuleChangeCmd,
    SolRigAddCmd,
    SolShipSetCmd,
    SolSkillAddCmd,
    SolStanceSetCmd,
)
from fw.api.types.fit import Fit
from fw.api.types.helpers import process_effect_map_request, process_muta_add_request, process_muta_change_request
from fw.api.types.item import Item
from fw.consts import ApiMinionState, ApiModAddMode, ApiModuleState, ApiRack
from fw.util import Absent

if typing.TYPE_CHECKING:
    from types import TracebackType

    from fw.api import ApiClient
    from fw.api.aliases import DpsProfile, MutaAdd, MutaChange
    from fw.api.commands import BaseCommand
    from fw.consts import (
        ApiEffMode,
        ApiFitInfoMode,
        ApiFleetInfoMode,
        ApiItemInfoMode,
        ApiModRmMode,
        ApiNpcProp,
        ApiOptionalReload,
        ApiSolInfoMode,
    )
    from .sol import SolarSystem


class SolCmdCtx:

    def __init__(
            self, *,
            client: ApiClient,
            sol: SolarSystem,
            sol_id: str,
            sol_info_mode: ApiSolInfoMode | type[Absent],
            fleet_info_mode: ApiFleetInfoMode | type[Absent],
            fit_info_mode: ApiFitInfoMode | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
            status_code: int,
    ) -> None:
        self._client = client
        self._sol = sol
        self._sol_id = sol_id
        self._sol_info_mode = sol_info_mode
        self._fleet_info_mode = fleet_info_mode
        self._fit_info_mode = fit_info_mode
        self._item_info_mode = item_info_mode
        self._status_code = status_code
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
        resp = self._client.sol_commands_request(
            sol_id=self._sol_id,
            commands=self._commands,
            sol_info_mode=self._sol_info_mode,
            fit_info_mode=self._fit_info_mode,
            fleet_info_mode=self._fleet_info_mode,
            item_info_mode=self._item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=self._status_code)
        # In case of successful response, update entity data
        if resp.status_code == 200:
            resp_data = resp.json()
            # Solar system which initiated the command chain
            self._sol._data = resp_data['solar_system']  # noqa: SLF001
            # Update IDs in all the entities which were created by the commands
            for i, cmd_result in enumerate(resp_data['cmd_results']):
                if cmd_result is None:
                    continue
                if i not in self._ret_datas:
                    continue
                entity_data = self._ret_datas[i]
                if 'id' in cmd_result:
                    entity_data['id'] = cmd_result['id']
                if 'charge_id' in cmd_result:
                    entity_data['charge'] = {'id': cmd_result['charge_id']}

    def __make_fit(self) -> Fit:
        # It is supposed to be called after command has been added
        index = len(self._commands) - 1
        data = {'id': f'#{index}'}
        self._ret_datas[index] = data
        return Fit(client=self._client, data=data, sol_id=self._sol_id)

    def __make_item(self) -> Item:
        # It is supposed to be called after command has been added
        index = len(self._commands) - 1
        data = {'id': f'#{index}', 'charge': {'id': f'#{index}c'}}
        self._ret_datas[index] = data
        return Item(client=self._client, data=data, sol_id=self._sol_id)

    # Fit
    def create_fit(
            self, *,
            fleet_id: str | type[Absent] = Absent,
            sec_status: float | type[Absent] = Absent,
            rah_incoming_dps: DpsProfile | type[Absent] = Absent,
    ) -> Fit | None:
        command = SolFitAddCmd(
            fleet_id=fleet_id,
            sec_status=sec_status,
            rah_incoming_dps=rah_incoming_dps)
        self._commands.append(command)
        return self.__make_fit()

    def remove_fit(self, *, fit_id: str) -> None:
        command = SolFitRemoveCmd(fit_id=fit_id)
        self._commands.append(command)

    # Item
    def remove_item(
            self, *,
            item_id: str,
            rm_mode: ApiModRmMode | type[Absent] = Absent,
    ) -> None:
        command = SolItemRemoveCmd(
            item_id=item_id,
            rm_mode=rm_mode)
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
        command = SolBoosterAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            side_effects=process_effect_map_request(effect_map=side_effects),
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
        return self.__make_item()

    # Item - character
    def set_character(
            self, *,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolCharacterSetCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
        return self.__make_item()

    # Item - drone
    def add_drone(
            self, *,
            fit_id: str,
            type_id: int,
            state: ApiMinionState = ApiMinionState.in_bay,
            mutation: MutaAdd | type[Absent] = Absent,
            npc_prop: ApiNpcProp | type[Absent] = Absent,
            proj_item_ids: list[str] | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolDroneAddCmd(
            fit_id=fit_id,
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

    # Item - implant
    def add_implant(
            self, *,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolImplantAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
        return self.__make_item()

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
            spool: str | type[Absent] = Absent,
            optional_reload: ApiOptionalReload | type[Absent] = Absent,
            proj_item_ids: list[str] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolModuleAddCmd(
            fit_id=fit_id,
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
        command = SolModuleChangeCmd(
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

    # Item - implant
    def add_rig(
            self, *,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolRigAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
        return self.__make_item()

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
        command = SolShipSetCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            coordinates=coordinates,
            movement=movement,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
        return self.__make_item()

    # Item - skill
    def add_skill(
            self, *,
            fit_id: str,
            type_id: int,
            level: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolSkillAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            level=level,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
        return self.__make_item()

    # Item - stance
    def set_stance(
            self, *,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> Item:
        command = SolStanceSetCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
        return self.__make_item()
