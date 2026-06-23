import typing

from fw.api.commands import (
    FitBoosterAddCmd,
    FitCharacterSetCmd,
    FitImplantAddCmd,
    FitShipSetCmd,
    FitSkillAddCmd,
    FitStanceSetCmd,
)
from fw.api.types.helpers import process_effect_map_request
from fw.api.types.item import Item
from fw.util import Absent

if typing.TYPE_CHECKING:
    from types import TracebackType

    from fw.api import ApiClient
    from fw.api.commands import BaseCommand
    from fw.consts import ApiEffMode, ApiFitInfoMode, ApiItemInfoMode
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
    ) -> None:
        self._client = client
        self._fit = fit
        self._sol_id = sol_id
        self._fit_id = fit_id
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
        resp = self._client._execute_fit_commands(  # noqa: SLF001
            sol_id=self._sol_id,
            fit_id=self._fit_id,
            commands=self._commands,
            fit_info_mode=self._fit_info_mode,
            item_info_mode=self._item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=self._status_code)
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
        data = {'id': fr'\{index}', 'charge': {'id': fr'\{index}c'}}
        self._ret_datas[index] = data
        return Item(client=self._client, data=data, sol_id=self._sol_id)

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
