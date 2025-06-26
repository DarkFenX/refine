from __future__ import annotations

import typing

if typing.TYPE_CHECKING:
    import pytest_httpserver

    from .containers import EveObjects


class EveDataServer:

    def __init__(self, *, data_server: pytest_httpserver.HTTPServer, **kwargs) -> None:
        super().__init__(**kwargs)
        self.__data_server = data_server

    def _setup_eve_data_server(self, *, data: EveObjects) -> None:
        str_data = data.render()
        suffix_cont_map = {
            'fsd_built/types.json': str_data.types,
            'fsd_built/groups.json': str_data.groups,
            'fsd_built/typelist.json': str_data.typelist,
            'fsd_built/dogmaattributes.json': str_data.dogmaattributes,
            'fsd_built/typedogma.json': str_data.typedogma,
            'fsd_built/dogmaeffects.json': str_data.dogmaeffects,
            'fsd_lite/fighterabilities.json': str_data.fighterabilities,
            'fsd_lite/fighterabilitiesbytype.json': str_data.fighterabilitiesbytype,
            'fsd_lite/dbuffcollections.json': str_data.dbuffcollections,
            'fsd_built/spacecomponentsbytype.json': str_data.spacecomponentsbytype,
            'fsd_built/requiredskillsfortypes.json': str_data.requiredskillsfortypes,
            'fsd_built/dynamicitemattributes.json': str_data.dynamicitemattributes}
        for suffix, container in suffix_cont_map.items():
            self.__setup_handler(url=f'/{data.alias}/{suffix}', data=container)

    def __setup_handler(self, *, url: str, data: str) -> None:
        self.__data_server.expect_request(url).respond_with_data(data)

    @property
    def _eve_data_server_base_url(self) -> str:
        return f'http://localhost:{self.__data_server.port}'
