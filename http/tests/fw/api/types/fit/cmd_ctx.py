import typing

from fw.api.commands import FitBoosterAddCmd
from fw.api.types.helpers import process_effect_map_request
from fw.util import Absent

if typing.TYPE_CHECKING:
    from types import TracebackType
    from typing_extensions import Self

    from fw.api import ApiClient
    from fw.api.commands import BaseCommand
    from fw.consts import ApiEffMode, ApiFitInfoMode, ApiItemInfoMode


class FitCmdCtx:

    def __init__(
            self, *,
            client: ApiClient,
            sol_id: str,
            fit_id: str,
            fit_info_mode: ApiFitInfoMode | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> None:
        self._client = client
        self._sol_id = sol_id
        self._fit_id = fit_id
        self._fit_info_mode = fit_info_mode
        self._item_info_mode = item_info_mode
        self._commands: list[BaseCommand] = []

    def __enter__(self) -> Self:
        return self

    def __exit__(
            self,
            exc_type: type[BaseException] | None,
            exc_val: BaseException | None,
            exc_tb: TracebackType | None,
    ) -> None:
        self._client.execute_fit_commands(
            sol_id=self._sol_id,
            fit_id=self._fit_id,
            commands=self._commands,
            fit_info_mode=self._fit_info_mode,
            item_info_mode=self._item_info_mode)

    def add_booster(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            side_effects: dict[int | str, bool] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
    ) -> None:
        command = FitBoosterAddCmd(
            type_id=type_id,
            state=state,
            side_effects=process_effect_map_request(effect_map=side_effects),
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        self._commands.append(command)
