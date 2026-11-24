import typing

from fw.api.types.batches.src import SrcCmdBatchCtx
from fw.api.types.helpers import (
    src_items_http_to_fw,
    src_mutated_attr_rolls_http_to_fw,
    src_mutated_items_http_to_fw,
)
from fw.consts import ApiSrcInfoMode
from fw.util import Absent, AttrDict

if typing.TYPE_CHECKING:
    from fw.aliases import JsonPredicate, ReqHook
    from fw.api import ApiClient


class Src(AttrDict):

    def __init__(self, *, client: ApiClient, data: dict) -> None:
        super().__init__(data=data)
        self._client = client

    def update(
            self, *,
            src_info_mode: ApiSrcInfoMode | type[Absent] = ApiSrcInfoMode.full,
            status_code: int = 200,
            json_predicate: JsonPredicate = None,
    ) -> Src | None:
        resp = self._client.get_source_request(src_alias=self.alias, src_info_mode=src_info_mode).send()
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def get_items(
            self, *,
            type_ids: list[int],
            hook_req: ReqHook | None = None,
            status_code: int = 200,
            json_predicate: JsonPredicate = None,
    ) -> dict[int, AttrDict] | None:
        req = self._client.get_source_items_request(src_alias=self.alias, type_ids=type_ids)
        if hook_req is not None:
            hook_req(req)
        resp = req.send()
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            return src_items_http_to_fw(data=resp.json())
        return None

    def get_mutated_items(
            self, *,
            mutations: list[tuple[int, int]],
            hook_req: ReqHook | None = None,
            status_code: int = 200,
            json_predicate: JsonPredicate = None,
    ) -> dict[int, dict[int, AttrDict]] | None:
        req = self._client.get_source_mutated_items_request(src_alias=self.alias, mutations=mutations)
        if hook_req is not None:
            hook_req(req)
        resp = req.send()
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            return src_mutated_items_http_to_fw(data=resp.json())
        return None

    def get_mutated_attr_rolls(
            self, *,
            mutation: tuple[int, int],
            values: dict[int | str, float],
            hook_req: ReqHook | None = None,
            status_code: int = 200,
            json_predicate: JsonPredicate = None,
    ) -> dict[int | str, float] | None:
        req = self._client.get_source_mutated_attr_rolls_request(
            src_alias=self.alias,
            mutation=mutation,
            values=values)
        if hook_req is not None:
            hook_req(req)
        resp = req.send()
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            return src_mutated_attr_rolls_http_to_fw(data=resp.json())
        return None

    def batch(
            self, *,
            hook_req: ReqHook | None = None,
            status_code: int = 200,
            json_predicate: JsonPredicate = None,
    ) -> SrcCmdBatchCtx:
        return SrcCmdBatchCtx(
            client=self._client,
            src_alias=self.alias,
            hook_req=hook_req,
            status_code=status_code,
            json_predicate=json_predicate)
