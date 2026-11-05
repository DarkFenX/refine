import typing

from fw.consts import ApiSrcInfoMode
from fw.util import Absent, AttrDict

if typing.TYPE_CHECKING:
    from fw.aliases import JsonPredicate
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
