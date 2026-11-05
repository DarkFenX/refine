import typing

from fw.api.types import RefineInfo
from fw.request import Request
from fw.util import Absent, conditional_insert
from .base import ApiClientBase

if typing.TYPE_CHECKING:
    from fw.aliases import JsonPredicate
    from fw.consts import ApiFitInfoMode, ApiFleetInfoMode, ApiItemInfoMode, ApiSolInfoMode, ApiSrcInfoMode


class ApiClientRefine(ApiClientBase):

    def get_refine_info_request(
            self, *,
            src_info_mode: ApiSrcInfoMode | type[Absent],
            sol_info_mode: ApiSolInfoMode | type[Absent],
            fleet_info_mode: ApiFleetInfoMode | type[Absent],
            fit_info_mode: ApiFitInfoMode | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        params = {}
        conditional_insert(container=params, path=['src'], value=src_info_mode)
        conditional_insert(container=params, path=['sol'], value=sol_info_mode)
        conditional_insert(container=params, path=['fleet'], value=fleet_info_mode)
        conditional_insert(container=params, path=['fit'], value=fit_info_mode)
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='GET',
            url=f'{self._base_url}/',
            params=params)

    def get_refine_info(
            self, *,
            src_info_mode: ApiSrcInfoMode | type[Absent] = Absent,
            sol_info_mode: ApiSolInfoMode | type[Absent] = Absent,
            fleet_info_mode: ApiFleetInfoMode | type[Absent] = Absent,
            fit_info_mode: ApiFitInfoMode | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = Absent,
            status_code: int = 200,
            json_predicate: JsonPredicate = None,
    ) -> RefineInfo | None:
        resp = self.get_refine_info_request(
            src_info_mode=src_info_mode,
            sol_info_mode=sol_info_mode,
            fleet_info_mode=fleet_info_mode,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            return RefineInfo(client=self, data=resp.json())
        return None
