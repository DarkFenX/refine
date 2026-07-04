import typing

from fw import eve
from fw.consts import ApiSrcInfoMode
from fw.request import Request
from fw.util import Default, conditional_insert
from .base import ApiClientBase

if typing.TYPE_CHECKING:
    from fw.eve.aliases import DataPrimHook, DataStrHook
    from fw.util import Absent


class ApiClientSrc(ApiClientBase, eve.EveDataManager, eve.EveDataServer):

    def __init__(self, **kwargs) -> None:
        super().__init__(**kwargs)
        self.__created_data_aliases: set[str] = set()

    def create_source_request(
            self, *,
            data: eve.EveObjects | type[Default],
            src_info_mode: ApiSrcInfoMode | type[Absent],
    ) -> Request:
        if data is Default:
            data = self._get_default_eve_data()
        params = {}
        conditional_insert(container=params, path=['src'], value=src_info_mode)
        return Request(
            client=self,
            method='POST',
            url=f'{self._base_url}/src/{data.alias}',
            params=params,
            json={'data_version': '1', 'data_base_url': f'{self._eve_data_server_base_url}/{data.alias}/'})

    def create_source(
            self, *,
            data: eve.EveObjects | type[Default] = Default,
            src_info_mode: ApiSrcInfoMode | type[Absent] = ApiSrcInfoMode.full,
            status_code: int = 201,
            cleanup_check: bool = True,
            json_predicate: dict | None = None,
            hook_data_prim: DataPrimHook | None = None,
            hook_data_str: DataStrHook | None = None,
    ) -> None:
        if data is Default:
            data = self._get_default_eve_data()
        self._setup_eve_data_server(
            data=data,
            hook_data_prim=hook_data_prim,
            hook_data_str=hook_data_str)
        resp = self.create_source_request(
            data=data,
            src_info_mode=src_info_mode).send()
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if status_code == 201:
            if cleanup_check:
                assert len(resp.json().get('warnings', {}).get('adg_cleanup', ())) == 0
            self.__created_data_aliases.add(data.alias)

    def remove_source_request(self, *, src_alias: str) -> Request:
        return Request(
            client=self,
            method='DELETE',
            url=f'{self._base_url}/src/{src_alias}')

    def remove_source(self, *, src_alias: str) -> None:
        resp = self.remove_source_request(src_alias=src_alias).send()
        assert resp.status_code == 204
        self.__created_data_aliases.remove(src_alias)

    def create_sources(
            self, *,
            src_info_mode: ApiSrcInfoMode | type[Absent] = ApiSrcInfoMode.full,
            status_code: int = 201,
            cleanup_check: bool = True,
            json_predicate: dict | None = None,
            hook_data_prim: DataPrimHook | None = None,
            hook_data_str: DataStrHook | None = None,
    ) -> None:
        # If no data was created, create default one
        if not self._eve_datas:
            self._get_default_eve_data()
        for data in self._eve_datas.values():
            self.create_source(
                data=data,
                src_info_mode=src_info_mode,
                status_code=status_code,
                cleanup_check=cleanup_check,
                json_predicate=json_predicate,
                hook_data_prim=hook_data_prim,
                hook_data_str=hook_data_str)

    def cleanup_sources(self) -> None:
        for alias in self.__created_data_aliases.copy():
            self.remove_source(src_alias=alias)
