import typing

from fw.api.commands import SrcInfoItemsCmd, SrcInfoMutatedAttrRollsCmd, SrcInfoMutatedItemsCmd
from fw.api.types.helpers import (
    src_items_http_to_fw,
    src_mutated_attr_rolls_http_to_fw,
    src_mutated_items_http_to_fw,
)

if typing.TYPE_CHECKING:
    from collections.abc import Callable
    from types import TracebackType

    from fw.aliases import JsonPredicate, ReqHook
    from fw.api import ApiClient
    from fw.api.commands import BaseCommand
    from fw.util import Absent, AttrDict


class SrcCmdBatchCtx:

    def __init__(
            self, *,
            client: ApiClient,
            src_alias: str,
            hook_req: ReqHook | None,
            status_code: int,
            json_predicate: JsonPredicate,
    ) -> None:
        self._client = client
        self._src_alias = src_alias
        self._hook_req = hook_req
        self._status_code = status_code
        self._json_predicate = json_predicate
        self._commands: list[BaseCommand] = []
        # Results are filled into those containers once the batch is sent
        self._ret_datas: list[tuple[dict, Callable[[dict], dict]]] = []

    def __enter__(self) -> typing.Self:
        return self

    def __exit__(
            self,
            exc_type: type[BaseException] | None,
            exc_val: BaseException | None,
            exc_tb: TracebackType | None,
    ) -> None:
        req = self._client.source_command_batch_request(src_alias=self._src_alias, commands=self._commands)
        if self._hook_req is not None:
            self._hook_req(req)
        resp = req.send()
        resp.check(status_code=self._status_code, json_predicate=self._json_predicate)
        if resp.status_code == 200:
            for (ret_data, converter), cmd_result in zip(self._ret_datas, resp.json(), strict=True):
                ret_data.update(converter(cmd_result))

    def get_items(self, *, type_ids: list[int] | type[Absent]) -> dict[int, AttrDict]:
        self._commands.append(SrcInfoItemsCmd(type_ids=type_ids))
        ret_data = {}
        self._ret_datas.append((ret_data, src_items_http_to_fw))
        return ret_data

    def get_mutated_items(
            self, *,
            mutations: list[tuple[int, int]] | type[Absent],
    ) -> dict[int, dict[int, AttrDict]]:
        self._commands.append(SrcInfoMutatedItemsCmd(mutations=mutations))
        ret_data = {}
        self._ret_datas.append((ret_data, src_mutated_items_http_to_fw))
        return ret_data

    def get_mutated_attr_rolls(
            self, *,
            mutation: tuple[int, int],
            values: dict[int | str, float],
    ) -> dict[int | str, float]:
        self._commands.append(SrcInfoMutatedAttrRollsCmd(mutation=mutation, values=values))
        ret_data = {}
        self._ret_datas.append((ret_data, src_mutated_attr_rolls_http_to_fw))
        return ret_data
