from __future__ import annotations

from abc import ABCMeta, abstractmethod
from typing import TYPE_CHECKING

import requests

from tests.support.consts import ApiRack, ApiState
from tests.support.request import Request
from tests.support.util import Absent, Default, conditional_insert
from tests.support import eve
from .types import SolarSystem

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support.consts import ApiEffMode


class ApiClient(metaclass=ABCMeta):

    def __init__(self, port: int):
        self.__session: requests.Session = requests.Session()
        self.__base_url: str = f'http://localhost:{port}'
        self.__created_data_aliases: set[str] = set()
        self.__created_sss: set[SolarSystem] = set()

    def send_prepared(self, req: Request) -> requests.models.Response:
        return self.__session.send(req)

    # Methods related to EVE data which are needed for API to work
    @property
    @abstractmethod
    def _eve_data_server_base_url(self) -> str:
        ...

    @abstractmethod
    def _setup_eve_data_server(self, data: eve.EveObjects) -> None:
        ...

    @abstractmethod
    def _get_eve_data(self, data: Union[eve.EveObjects, Type[Default]] = Default) -> eve.EveObjects:
        ...

    @property
    @abstractmethod
    def _eve_datas(self) -> dict[str, eve.EveObjects]:
        ...

    # Data source methods
    def create_source_request(
            self,
            data: Union[eve.EveObjects, Type[Default]] = Default,
    ) -> Request:
        data = self._get_eve_data(data=data)
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/source/{data.alias}',
            json={'data_version': '1', 'data_base_url': f'{self._eve_data_server_base_url}/{data.alias}/'})

    def create_source(
            self,
            data: Union[eve.EveObjects, Type[Default]] = Default,
    ) -> None:
        data = self._get_eve_data(data=data)
        self._setup_eve_data_server(data=data)
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

    def create_sources(self) -> None:
        for data in self._eve_datas.values():
            self.create_source(data)

    def cleanup_sources(self) -> None:
        for alias in self.__created_data_aliases.copy():
            self.remove_source(src_alias=alias)

    # Solar system methods
    def create_ss_request(
            self,
            data: Union[eve.EveObjects, Type[Default]] = Default,
    ) -> Request:
        data = self._get_eve_data(data=data)
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
            data: Union[eve.EveObjects, Type[Default]] = Default,
    ) -> SolarSystem:
        data = self._get_eve_data(data=data)
        resp = self.create_ss_request(data=data).send()
        assert resp.status_code == 201
        sol_sys = SolarSystem(client=self, data=resp.json())
        self.__created_sss.add(sol_sys)
        return sol_sys

    @property
    def created_sss(self):
        return self.__created_sss

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
        for ss in self.__created_sss.copy():
            ss.remove()

    # Fit methods
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

    # Generic item methods
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

    # Character methods
    def set_char_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self.__add_simple_item('set_character', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    # Skill methods
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

    # Implant methods
    def add_implant_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self.__add_simple_item('add_implant', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    # Booster methods
    def add_booster_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self.__add_simple_item('add_booster', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    # Ship methods
    def set_ship_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self.__add_simple_item('set_ship', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    # Structure methods
    def set_struct_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self.__add_simple_item('set_structure', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    # Stance methods
    def set_stance_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self.__add_simple_item('set_stance', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    # Subsystem methods
    def add_subsystem_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self.__add_simple_item('add_subsystem', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    # Module methods
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

    # Rig methods
    def add_rig_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self.__add_simple_item('add_rig', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    # Drone methods
    def add_drone_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: str = ApiState.offline,
    ) -> Request:
        return self.__add_simple_item('add_drone', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    # Fighter methods
    def add_fighter_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: str = ApiState.offline,
    ) -> Request:
        return self.__add_simple_item('add_fighter', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    # System-wide effect methods
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

    # Fit-wide effect methods
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

    # Projected effect methods
    def add_proj_effect_request(
            self,
            ss_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        command = {'type': 'add_proj_effect', 'type_id': type_id}
        conditional_insert(command, 'state', state)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json={'commands': [command]})

    # Auxiliary methods
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
