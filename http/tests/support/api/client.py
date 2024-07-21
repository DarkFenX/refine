from __future__ import annotations

from typing import TYPE_CHECKING

import requests

from tests.support import eve
from tests.support.consts import (
    ApiFitInfoMode, ApiFleetInfoMode, ApiItemInfoMode, ApiModAddMode, ApiRack, ApiSolInfoMode, ApiState)
from tests.support.request import Request
from tests.support.response import Response
from tests.support.util import Absent, Default, conditional_insert
from .exception import ApiSolCheckError
from .types import SolarSystem

if TYPE_CHECKING:
    from collections.abc import Iterable
    from typing import Type, Union

    from tests.support.consts import ApiEffMode


class ApiClient(eve.EveDataManager, eve.EveDataServer):

    def __init__(self, port: int, **kwargs):
        super().__init__(**kwargs)
        self.__session: requests.Session = requests.Session()
        self.__base_url: str = f'http://localhost:{port}'
        self.__created_data_aliases: set[str] = set()
        self.__created_sols: set[SolarSystem] = set()

    def send_prepared(self, req: Request) -> Response:
        response = self.__session.send(req)
        return Response(response)

    # Data source methods
    def create_source_request(
            self,
            data: Union[eve.EveObjects, Type[Default]] = Default,
    ) -> Request:
        data = self._get_eve_data(data=data)
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/src/{data.alias}',
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
            url=f'{self.__base_url}/src/{src_alias}')

    def remove_source(self, src_alias: str) -> None:
        resp = self.remove_source_request(src_alias=src_alias).send()
        assert resp.status_code == 204
        self.__created_data_aliases.remove(src_alias)

    def create_sources(self) -> None:
        # If no data was created, create default one
        if not self._eve_datas:
            self._get_eve_data()
        for data in self._eve_datas.values():
            self.create_source(data)

    def cleanup_sources(self) -> None:
        for alias in self.__created_data_aliases.copy():
            self.remove_source(src_alias=alias)

    # Solar system methods
    def create_sol_request(
            self,
            data: Union[eve.EveObjects, Type[Default]] = Default,
            sol_info_mode: ApiSolInfoMode = ApiSolInfoMode.full,
            fleet_info_mode: ApiFleetInfoMode = ApiFleetInfoMode.id,
            fit_info_mode: ApiFitInfoMode = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        data = self._get_eve_data(data=data)
        body = {}
        if data is not Absent:
            body['src_alias'] = data.alias
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/sol',
            params={'sol': sol_info_mode, 'fleet': fleet_info_mode, 'fit': fit_info_mode, 'item': item_info_mode},
            json=body)

    def create_sol(
            self,
            data: Union[eve.EveObjects, Type[Default]] = Default,
    ) -> SolarSystem:
        data = self._get_eve_data(data=data)
        resp = self.create_sol_request(data=data).send()
        assert resp.status_code == 201
        sol_sys = SolarSystem(client=self, data=resp.json())
        self.__created_sols.add(sol_sys)
        return sol_sys

    @property
    def created_sols(self):
        return self.__created_sols

    def get_sol_request(
            self,
            sol_id: str,
            sol_info_mode: ApiSolInfoMode = ApiSolInfoMode.full,
            fleet_info_mode: ApiFleetInfoMode = ApiFleetInfoMode.id,
            fit_info_mode: ApiFitInfoMode = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        return Request(
            self,
            method='GET',
            url=f'{self.__base_url}/sol/{sol_id}',
            params={'sol': sol_info_mode, 'fleet': fleet_info_mode, 'fit': fit_info_mode, 'item': item_info_mode})

    def remove_sol_request(self, sol_id: str) -> Request:
        return Request(
            self,
            method='DELETE',
            url=f'{self.__base_url}/sol/{sol_id}')

    def check_sol(self, sol_id: str) -> None:
        resp = self.check_sol_request(sol_id=sol_id).send()
        if resp.status_code != 200:
            raise ApiSolCheckError

    def check_sol_request(self, sol_id: str) -> Request:
        return Request(
            self,
            method='GET',
            url=f'{self.__base_url}/sol/{sol_id}/check')

    def cleanup_sols(self) -> None:
        for sol in self.__created_sols.copy():
            sol.remove()

    # Fleet methods
    def get_fleet_request(
            self,
            sol_id: str,
            fleet_id: str,
            fleet_info_mode: ApiFleetInfoMode = ApiFleetInfoMode.full,
    ) -> Request:
        return Request(
            self,
            method='GET',
            url=f'{self.__base_url}/sol/{sol_id}/fleet/{fleet_id}',
            params={'fleet': fleet_info_mode})

    def create_fleet_request(
            self,
            sol_id: str,
            fleet_info_mode: ApiFleetInfoMode = ApiFleetInfoMode.full,
    ) -> Request:
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/sol/{sol_id}/fleet',
            params={'fleet': fleet_info_mode})

    def change_fleet_request(
            self,
            sol_id: str,
            fleet_id: str,
            add_fits: list[str] = (),
            remove_fits: list[str] = (),
            fleet_info_mode: ApiFleetInfoMode = ApiFleetInfoMode.full,
    ) -> Request:
        command = {}
        conditional_insert(command, 'add_fits', add_fits)
        conditional_insert(command, 'remove_fits', remove_fits)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/sol/{sol_id}/fleet/{fleet_id}',
            params={'fleet': fleet_info_mode},
            json=command)

    def remove_fleet_request(
            self,
            sol_id: str,
            fleet_id: str,
    ) -> Request:
        return Request(
            self,
            method='DELETE',
            url=f'{self.__base_url}/sol/{sol_id}/fleet/{fleet_id}')

    # Fit methods
    def get_fit_request(
            self,
            sol_id: str,
            fit_id: str,
            fit_info_mode: ApiFitInfoMode = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.full,
    ) -> Request:
        return Request(
            self,
            method='GET',
            url=f'{self.__base_url}/sol/{sol_id}/fit/{fit_id}',
            params={'fit': fit_info_mode, 'item': item_info_mode})

    def create_fit_request(
            self,
            sol_id: str,
            fit_info_mode: ApiFitInfoMode = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/sol/{sol_id}/fit',
            params={'fit': fit_info_mode, 'item': item_info_mode})

    def set_fit_fleet_request(
            self,
            sol_id: str,
            fit_id: str,
            fleet_id: Union[str, None],
            fit_info_mode: ApiFitInfoMode = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        command = {'type': 'set_fleet', 'fleet_id': fleet_id}
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/sol/{sol_id}/fit/{fit_id}',
            params={'fit': fit_info_mode, 'item': item_info_mode},
            json={'commands': [command]})

    def remove_fit_request(
            self,
            sol_id: str,
            fit_id: str,
    ) -> Request:
        return Request(
            self,
            method='DELETE',
            url=f'{self.__base_url}/sol/{sol_id}/fit/{fit_id}')

    # Generic item methods
    def get_item_request(
            self,
            sol_id: str,
            item_id: str,
            item_info_mode: ApiItemInfoMode,
    ) -> Request:
        return Request(
            self,
            method='GET',
            url=f'{self.__base_url}/sol/{sol_id}/item/{item_id}',
            params={'item': item_info_mode})

    def remove_item_request(
            self,
            sol_id: str,
            item_id: str,
    ) -> Request:
        return Request(
            self,
            method='DELETE',
            url=f'{self.__base_url}/sol/{sol_id}/item/{item_id}')

    # Character methods
    def set_char_request(
            self,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        return self.__add_simple_item(
            cmd_name='character', sol_id=sol_id, fit_id=fit_id, type_id=type_id, state=state,
            item_info_mode=item_info_mode)

    def change_char_request(
            self,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        return self.__change_simple_item(
            cmd_name='character', sol_id=sol_id, item_id=item_id, state=state,
            item_info_mode=item_info_mode)

    # Skill methods
    def add_skill_request(
            self,
            sol_id: str,
            fit_id: str,
            type_id: int,
            level: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        command = {
            'type': 'skill',
            'fit_id': fit_id,
            'type_id': type_id,
            'level': level}
        conditional_insert(command, 'state', state)
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/sol/{sol_id}/item',
            params={'item': item_info_mode},
            json=command)

    def change_skill_request(
            self,
            sol_id: str,
            item_id: str,
            level: Union[int, Type[Absent]] = Absent,
            state: Union[bool, Type[Absent]] = Absent,
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]] = Absent,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        command = {'type': 'skill'}
        conditional_insert(command, 'level', level)
        conditional_insert(command, 'state', state)
        conditional_insert(command, 'effect_modes', effect_modes)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/sol/{sol_id}/item/{item_id}',
            params={'item': item_info_mode},
            json=command)

    # Implant methods
    def add_implant_request(
            self,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        return self.__add_simple_item(
            cmd_name='implant', sol_id=sol_id, fit_id=fit_id, type_id=type_id, state=state,
            item_info_mode=item_info_mode)

    def change_implant_request(
            self,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        return self.__change_simple_item(
            cmd_name='implant', sol_id=sol_id, item_id=item_id, state=state,
            item_info_mode=item_info_mode)

    # Booster methods
    def add_booster_request(
            self,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        return self.__add_simple_item(
            cmd_name='booster', sol_id=sol_id, fit_id=fit_id, type_id=type_id, state=state,
            item_info_mode=item_info_mode)

    # Ship methods
    def set_ship_request(
            self,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        return self.__add_simple_item(
            cmd_name='ship', sol_id=sol_id, fit_id=fit_id, type_id=type_id, state=state,
            item_info_mode=item_info_mode)

    def change_ship_request(
            self,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        return self.__change_simple_item(
            cmd_name='ship', sol_id=sol_id, item_id=item_id, state=state,
            item_info_mode=item_info_mode)

    # Stance methods
    def set_stance_request(
            self,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        return self.__add_simple_item(
            cmd_name='stance', sol_id=sol_id, fit_id=fit_id, type_id=type_id, state=state,
            item_info_mode=item_info_mode)

    # Subsystem methods
    def add_subsystem_request(
            self,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        return self.__add_simple_item(
            cmd_name='subsystem', sol_id=sol_id, fit_id=fit_id, type_id=type_id, state=state,
            item_info_mode=item_info_mode)

    # Module methods
    def add_mod_request(
            self,
            sol_id: str,
            fit_id: str,
            type_id: int,
            rack: ApiRack = ApiRack.high,
            state: ApiState = ApiState.offline,
            charge_type_id: Union[int, Type[Absent]] = Absent,
            mode: ApiModAddMode = ApiModAddMode.equip,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        command = {
            'type': 'module',
            'fit_id': fit_id,
            'rack': rack,
            'add_mode': mode,
            'type_id': type_id,
            'state': state}
        conditional_insert(command, 'charge_type_id', charge_type_id)
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/sol/{sol_id}/item',
            params={'item': item_info_mode},
            json=command)

    def change_mod_request(
            self,
            sol_id: str,
            item_id: str,
            state: Union[ApiState, Type[Absent]] = Absent,
            charge: Union[int, None, Type[Absent]] = Absent,
            add_projs: Union[Iterable[(str, Union[float, None])], Type[Absent]] = Absent,
            change_projs: Union[Iterable[(str, Union[float, None])], Type[Absent]] = Absent,
            rm_projs: Union[Iterable[str], Type[Absent]] = Absent,
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]] = Absent,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        command = {'type': 'module'}
        conditional_insert(command, 'state', state)
        conditional_insert(command, 'charge', charge)
        conditional_insert(command, 'add_projs', add_projs)
        conditional_insert(command, 'change_projs', change_projs)
        conditional_insert(command, 'rm_projs', rm_projs)
        conditional_insert(command, 'effect_modes', effect_modes)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/sol/{sol_id}/item/{item_id}',
            params={'item': item_info_mode},
            json=command)

    # Rig methods
    def add_rig_request(
            self,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        return self.__add_simple_item(
            cmd_name='rig', sol_id=sol_id, fit_id=fit_id, type_id=type_id, state=state,
            item_info_mode=item_info_mode)

    def change_rig_request(
            self,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        return self.__change_simple_item(
            cmd_name='rig', sol_id=sol_id, item_id=item_id, state=state,
            item_info_mode=item_info_mode)

    # Drone methods
    def add_drone_request(
            self,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: ApiState = ApiState.offline,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        return self.__add_simple_item(
            cmd_name='drone', sol_id=sol_id, fit_id=fit_id, type_id=type_id, state=state,
            item_info_mode=item_info_mode)

    # Fighter methods
    def add_fighter_request(
            self,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: ApiState = ApiState.offline,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        return self.__add_simple_item(
            cmd_name='fighter', sol_id=sol_id, fit_id=fit_id, type_id=type_id, state=state,
            item_info_mode=item_info_mode)

    # System-wide effect methods
    def add_sw_effect_request(
            self,
            sol_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        command = {'type': 'sw_effect', 'type_id': type_id}
        conditional_insert(command, 'state', state)
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/sol/{sol_id}/item',
            params={'item': item_info_mode},
            json=command)

    def change_sw_effect_request(
            self,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        return self.__change_simple_item(
            cmd_name='sw_effect', sol_id=sol_id, item_id=item_id, state=state,
            item_info_mode=item_info_mode)

    # Fit-wide effect methods
    def add_fw_effect_request(
            self,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        return self.__add_simple_item(
            cmd_name='fw_effect', sol_id=sol_id, fit_id=fit_id, type_id=type_id, state=state,
            item_info_mode=item_info_mode)

    def change_fw_effect_request(
            self,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        return self.__change_simple_item(
            cmd_name='fw_effect', sol_id=sol_id, item_id=item_id, state=state,
            item_info_mode=item_info_mode)

    # Projected effect methods
    def add_proj_effect_request(
            self,
            sol_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        command = {'type': 'proj_effect', 'type_id': type_id}
        conditional_insert(command, 'state', state)
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/sol/{sol_id}/item',
            params={'item': item_info_mode},
            json=command)

    def change_proj_effect_request(
            self,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            add_projs: Union[Iterable[str], Type[Absent]] = Absent,
            rm_projs: Union[Iterable[str], Type[Absent]] = Absent,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        command = {'type': 'proj_effect', 'item_id': item_id}
        conditional_insert(command, 'state', state)
        conditional_insert(command, 'add_projs', add_projs)
        conditional_insert(command, 'rm_projs', rm_projs)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/sol/{sol_id}/item/{item_id}',
            params={'item': item_info_mode},
            json=command)

    # Auxiliary methods
    def __add_simple_item(
            self,
            cmd_name: str,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, str, Type[Absent]],
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        command = {
            'type': cmd_name,
            'fit_id': fit_id,
            'type_id': type_id}
        conditional_insert(command, 'state', state)
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/sol/{sol_id}/item',
            params={'item': item_info_mode},
            json=command)

    def __change_simple_item(
            self,
            cmd_name: str,
            sol_id: str,
            item_id: int,
            state: Union[bool, str, Type[Absent]],
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.id,
    ) -> Request:
        command = {'type': cmd_name}
        conditional_insert(command, 'state', state)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/sol/{sol_id}/item/{item_id}',
            params={'item': item_info_mode},
            json=command)
