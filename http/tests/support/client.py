from __future__ import annotations

from typing import TYPE_CHECKING

import requests

from tests.support.consts import ApiRack, ApiState
from tests.support.eve_data import EveDataClient
from tests.support.request import Request
from tests.support.util import Absent, Default, conditional_insert
from tests.support import api_data, eve_data

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support.consts import ApiEffMode

data_id: int = 10000000  # pylint: disable=C0103


class TestClient(EveDataClient):

    def __init__(self, data_server, port: int):
        super().__init__()
        self.__data_server = data_server
        self.__session: requests.Session = requests.Session()
        self.__base_url: str = f'http://localhost:{port}'
        self.__created_data_aliases: set[str] = set()
        self.created_sss: set[api_data.SolarSystem] = set()

    def send_prepared(self, req: Request) -> requests.models.Response:
        return self.__session.send(req)

    # Data source-related methods
    def create_source_request(
            self,
            data: Union[eve_data.EveObjects, Type[Default]] = Default,
    ) -> Request:
        if data is Default:
            data = self._default_data
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/source/{data.alias}',
            json={'data_version': '1', 'data_base_url': f'http://localhost:{self.__data_server.port}/{data.alias}/'})

    def create_source(
            self,
            data: Union[eve_data.EveObjects, Type[Default]] = Default,
    ) -> None:
        if data is Default:
            data = self._default_data
        # Set up server with local data
        str_data = data.render()
        suffix_cont_map = {
            'fsd_binary/types.json': str_data.types,
            'fsd_binary/groups.json': str_data.groups,
            'fsd_binary/dogmaattributes.json': str_data.dogmaattributes,
            'fsd_binary/typedogma.json': str_data.typedogma,
            'fsd_binary/dogmaeffects.json': str_data.dogmaeffects,
            'fsd_lite/fighterabilities.json': str_data.fighterabilities,
            'fsd_lite/fighterabilitiesbytype.json': str_data.fighterabilitiesbytype,
            'fsd_lite/dbuffcollections.json': str_data.dbuffcollections,
            'fsd_binary/requiredskillsfortypes.json': str_data.requiredskillsfortypes,
            'fsd_binary/dynamicitemattributes.json': str_data.dynamicitemattributes}
        for suffix, container in suffix_cont_map.items():
            self.__setup_handler(f'/{data.alias}/{suffix}', container)
        # Get request and send it
        resp = self.create_source_request(data=data).send()
        assert resp.status_code == 201
        self.__created_data_aliases.add(data.alias)

    def remove_source_request(self, src_alias: str) -> Request:
        return Request(
            self,
            method='DELETE',
            url=f'{self.__base_url}/source/{src_alias}')

    def remove_source(self, src_alias: str) -> None:
        resp = self.remove_source_request(src_alias=src_alias).send()
        assert resp.status_code == 204
        self.__created_data_aliases.remove(src_alias)

    def __setup_handler(self, url: str, data: str) -> None:
        self.__data_server.expect_request(url).respond_with_data(data)

    def create_sources(self) -> None:
        for data in self._datas.values():
            self.create_source(data)

    def cleanup_sources(self) -> None:
        for alias in self.__created_data_aliases.copy():
            self.remove_source(src_alias=alias)

    # Solar system-related methods
    def create_ss_request(
            self,
            data: Union[eve_data.EveObjects, Type[Default]] = Default,
    ) -> Request:
        if data is Default:
            data = self._default_data
        body = {}
        if data is not Absent:
            body['src_alias'] = data.alias
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/solar_system',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json=body)

    def create_ss(
            self,
            data: Union[eve_data.EveObjects, Type[Default]] = Default,
    ) -> api_data.SolarSystem:
        if data is Default:
            data = self._default_data
        resp = self.create_ss_request(data=data).send()
        assert resp.status_code == 201
        sol_sys = api_data.SolarSystem(client=self, data=resp.json())
        self.created_sss.add(sol_sys)
        return sol_sys

    def update_ss_request(self, ss_id: str) -> Request:
        return Request(
            self,
            method='GET',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'})

    def remove_ss_request(self, ss_id: str) -> Request:
        return Request(
            self,
            method='DELETE',
            url=f'{self.__base_url}/solar_system/{ss_id}')

    def cleanup_sss(self) -> None:
        for ss in self.created_sss.copy():
            ss.remove()

    # Fit-related methods
    def create_fit_request(
            self,
            ss_id: str
    ) -> Request:
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/solar_system/{ss_id}/fit',
            params={'fit': 'full', 'item': 'id'})

    def update_fit_request(
            self,
            ss_id: str,
            fit_id: str
    ) -> Request:
        return Request(
            self,
            method='GET',
            url=f'{self.__base_url}/solar_system/{ss_id}/fit/{fit_id}',
            params={'fit': 'full', 'item': 'id'})

    def remove_fit_request(
            self,
            ss_id: str,
            fit_id: str
    ) -> Request:
        return Request(
            self,
            method='DELETE',
            url=f'{self.__base_url}/solar_system/{ss_id}/fit/{fit_id}')

    # Item-related methods
    def get_item_request(
            self,
            ss_id: str,
            item_id: str
    ) -> Request:
        return Request(
            self,
            method='GET',
            url=f'{self.__base_url}/solar_system/{ss_id}/item/{item_id}',
            params={'item': 'full'})

    def remove_item_request(
            self,
            ss_id: str,
            item_id: str
    ) -> Request:
        return Request(
            self,
            method='DELETE',
            url=f'{self.__base_url}/solar_system/{ss_id}/item/{item_id}')

    def set_char_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int
    ) -> Request:
        payload = {'commands': [{'type': 'set_character', 'fit_id': fit_id, 'type_id': type_id}]}
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json=payload)

    def add_skill_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            level: int,
            state: Union[bool, Type[Absent]] = Absent
    ) -> Request:
        command = {
            'type': 'add_skill',
            'fit_id': fit_id,
            'type_id': type_id,
            'level': level}
        conditional_insert(command, 'state', state)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json={'commands': [command]})

    def change_skill_request(
            self,
            ss_id: str,
            item_id: str,
            level: Union[int, Type[Absent]] = Absent,
            state: Union[bool, Type[Absent]] = Absent,
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]] = Absent,
    ) -> Request:
        command = {'type': 'change_skill', 'item_id': item_id}
        conditional_insert(command, 'level', level)
        conditional_insert(command, 'state', state)
        conditional_insert(command, 'effect_modes', effect_modes)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json={'commands': [command]})

    def add_implant_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self.__add_simple_item('add_implant', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    def set_ship_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
    ) -> Request:
        payload = {'commands': [{'type': 'set_ship', 'fit_id': fit_id, 'type_id': type_id}]}
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json=payload)

    def set_struct_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
    ) -> Request:
        payload = {'commands': [{'type': 'set_structure', 'fit_id': fit_id, 'type_id': type_id}]}
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json=payload)

    def add_subsystem_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self.__add_simple_item('add_subsystem', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    def add_mod_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            rack: ApiRack = ApiRack.high,
            state: str = ApiState.offline,
            charge_type_id: Union[int, Type[Absent]] = Absent,
            mode: str = 'equip',
    ) -> Request:
        command = {
            'type': 'add_module',
            'fit_id': fit_id,
            'rack': rack,
            'add_mode': mode,
            'type_id': type_id,
            'state': state}
        conditional_insert(command, 'charge_type_id', charge_type_id)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json={'commands': [command]})

    def change_mod_request(
            self,
            ss_id: str,
            item_id: str,
            state: Union[ApiState, Type[Absent]] = Absent,
            charge: Union[int, None, Type[Absent]] = Absent,
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]] = Absent,
    ) -> Request:
        command = {'type': 'change_module', 'item_id': item_id}
        conditional_insert(command, 'state', state)
        conditional_insert(command, 'charge', charge)
        conditional_insert(command, 'effect_modes', effect_modes)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json={'commands': [command]})

    def add_rig_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self.__add_simple_item('add_rig', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    def add_drone_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: str = ApiState.offline,
    ) -> Request:
        return self.__add_simple_item('add_drone', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    def add_sw_effect_request(
            self,
            ss_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        command = {'type': 'add_sw_effect', 'type_id': type_id}
        conditional_insert(command, 'state', state)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json={'commands': [command]})

    def change_sw_effect_request(
            self,
            ss_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self.__change_simple_item('change_sw_effect', ss_id=ss_id, item_id=item_id, state=state)

    def add_fw_effect_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self.__add_simple_item('add_fw_effect', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    def change_fw_effect_request(
            self,
            ss_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self.__change_simple_item('change_fw_effect', ss_id=ss_id, item_id=item_id, state=state)

    def __add_simple_item(
            self,
            cmd_name: str,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, str, Type[Absent]],
    ) -> Request:
        command = {
            'type': cmd_name,
            'fit_id': fit_id,
            'type_id': type_id}
        conditional_insert(command, 'state', state)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json={'commands': [command]})

    def __change_simple_item(
            self,
            cmd_name: str,
            ss_id: str,
            item_id: int,
            state: Union[bool, str, Type[Absent]],
    ) -> Request:
        command = {'type': cmd_name, 'item_id': item_id}
        conditional_insert(command, 'state', state)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json={'commands': [command]})
