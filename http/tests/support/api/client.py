from __future__ import annotations

from typing import TYPE_CHECKING

import pytest
import requests

from tests.support import eve
from tests.support.consts import ApiFitInfoMode, ApiFleetInfoMode, ApiItemInfoMode, ApiSolInfoMode
from tests.support.log import LogEntryNotFound
from tests.support.request import Request
from tests.support.response import Response
from tests.support.util import Absent, Default, conditional_insert
from .exception import ApiSolCheckError
from .types import SolarSystem

if TYPE_CHECKING:
    from collections.abc import Iterable
    from typing import Tuple, Type, Union

    from tests.support.consts import ApiEffMode, ApiModAddMode, ApiRack, ApiState
    from tests.support.log import LogReader


class ApiClient(eve.EveDataManager, eve.EveDataServer):

    def __init__(self, *, port: int, log_reader: LogReader, **kwargs):
        super().__init__(**kwargs)
        self.__session: requests.Session = requests.Session()
        self.__base_url: str = f'http://localhost:{port}'
        self.__created_data_aliases: set[str] = set()
        self.__created_sols: set[SolarSystem] = set()
        self.__log_reader = log_reader

    def send_prepared(self, *, req: Request) -> Response:
        response = self.__session.send(req)
        return Response(response=response)

    # Data source methods
    def create_source_request(
            self, *,
            data: Union[eve.EveObjects, Type[Default]],
    ) -> Request:
        if data is Default:
            data = self._get_default_eve_data()
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/src/{data.alias}',
            json={'data_version': '1', 'data_base_url': f'{self._eve_data_server_base_url}/{data.alias}/'})

    def create_source(
            self, *,
            data: Union[eve.EveObjects, Type[Default]] = Default,
    ) -> None:
        if data is Default:
            data = self._get_default_eve_data()
        self._setup_eve_data_server(data=data)
        resp = self.create_source_request(data=data).send()
        assert resp.status_code == 201
        self.__created_data_aliases.add(data.alias)

    def remove_source_request(self, *, src_alias: str) -> Request:
        return Request(
            self,
            method='DELETE',
            url=f'{self.__base_url}/src/{src_alias}')

    def remove_source(self, *, src_alias: str) -> None:
        resp = self.remove_source_request(src_alias=src_alias).send()
        assert resp.status_code == 204
        self.__created_data_aliases.remove(src_alias)

    def create_sources(self, log_check: bool = True) -> None:
        # If no data was created, create default one
        if not self._eve_datas:
            self._get_default_eve_data()
        if log_check:
            with self.__log_reader.get_collector() as log_collector:
                for data in self._eve_datas.values():
                    self.create_source(data=data)
                with pytest.raises(LogEntryNotFound):
                    # Timeout of zero is not reliable, but don't want to slow tests down much
                    log_collector.wait_log_entry(msg='re:cleaned .+', level='INFO', span='src-new:adg', timeout=0)
        else:
            for data in self._eve_datas.values():
                self.create_source(data=data)

    def cleanup_sources(self) -> None:
        for alias in self.__created_data_aliases.copy():
            self.remove_source(src_alias=alias)

    # Solar system methods
    def create_sol_request(
            self, *,
            data: Union[eve.EveObjects, Type[Absent], Type[Default]],
            sol_info_mode: Union[ApiSolInfoMode, Type[Absent]],
            fleet_info_mode: Union[ApiFleetInfoMode, Type[Absent]],
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        params = {}
        conditional_insert(container=params, key='sol', value=sol_info_mode)
        conditional_insert(container=params, key='fleet', value=fleet_info_mode)
        conditional_insert(container=params, key='fit', value=fit_info_mode)
        conditional_insert(container=params, key='item', value=item_info_mode)
        body = {}
        if data is not Absent:
            if data is Default:
                data = self._get_default_eve_data()
            body['src_alias'] = data.alias
        return Request(self, method='POST', url=f'{self.__base_url}/sol', params=params, json=body)

    def create_sol(
            self, *,
            data: Union[eve.EveObjects, Type[Absent], Type[Default]] = Default,
            sol_info_mode: Union[ApiSolInfoMode, Type[Absent]] = ApiSolInfoMode.id,
            fleet_info_mode: Union[ApiFleetInfoMode, Type[Absent]] = Absent,
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = Absent,
    ) -> SolarSystem:
        if data is Default:
            data = self._get_default_eve_data()
        resp = self.create_sol_request(
            data=data,
            sol_info_mode=sol_info_mode,
            fleet_info_mode=fleet_info_mode,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        assert resp.status_code == 201
        sol_sys = SolarSystem(client=self, data=resp.json())
        self.__created_sols.add(sol_sys)
        return sol_sys

    @property
    def created_sols(self):
        return self.__created_sols

    def get_sol(
            self, *,
            sol_id: str,
            sol_info_mode: Union[ApiSolInfoMode, Type[Absent]],
            fleet_info_mode: Union[ApiFleetInfoMode, Type[Absent]],
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
            status_code: int = 200,
            json_predicate: Union[dict, None] = None,
    ) -> Union[SolarSystem, None]:
        resp = self.get_sol_request(
            sol_id=sol_id,
            sol_info_mode=sol_info_mode,
            fleet_info_mode=fleet_info_mode,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            return SolarSystem(client=self, data=resp.json())
        return None

    def get_sol_request(
            self, *,
            sol_id: str,
            sol_info_mode: Union[ApiSolInfoMode, Type[Absent]],
            fleet_info_mode: Union[ApiFleetInfoMode, Type[Absent]],
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        params = {}
        conditional_insert(container=params, key='sol', value=sol_info_mode)
        conditional_insert(container=params, key='fleet', value=fleet_info_mode)
        conditional_insert(container=params, key='fit', value=fit_info_mode)
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='GET',
            url=f'{self.__base_url}/sol/{sol_id}',
            params=params)

    def change_sol_src_request(
            self, *,
            sol_id: str,
            data: Union[eve.EveObjects, Type[Absent], Type[Default]],
            sol_info_mode: Union[ApiSolInfoMode, Type[Absent]],
            fleet_info_mode: Union[ApiFleetInfoMode, Type[Absent]],
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {}
        if data is not Absent:
            if data is Default:
                data = self._get_default_eve_data()
            body['src_alias'] = data.alias
        params = {}
        conditional_insert(container=params, key='sol', value=sol_info_mode)
        conditional_insert(container=params, key='fleet', value=fleet_info_mode)
        conditional_insert(container=params, key='fit', value=fit_info_mode)
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/sol/{sol_id}/src',
            params=params,
            json=body)

    def remove_sol_request(self, *, sol_id: str) -> Request:
        return Request(
            self,
            method='DELETE',
            url=f'{self.__base_url}/sol/{sol_id}')

    def check_sol(self, *, sol_id: str) -> None:
        resp = self.check_sol_request(sol_id=sol_id).send()
        if resp.status_code != 200:
            raise ApiSolCheckError

    def check_sol_request(self, *, sol_id: str) -> Request:
        return Request(
            self,
            method='GET',
            url=f'{self.__base_url}/sol/{sol_id}/check')

    def cleanup_sols(self) -> None:
        for sol in self.__created_sols.copy():
            sol.remove()

    # Fleet methods
    def get_fleet_request(
            self, *,
            sol_id: str,
            fleet_id: str,
            fleet_info_mode: Union[ApiFleetInfoMode, Type[Absent]],
    ) -> Request:
        params = {}
        conditional_insert(container=params, key='fleet', value=fleet_info_mode)
        return Request(
            self,
            method='GET',
            url=f'{self.__base_url}/sol/{sol_id}/fleet/{fleet_id}',
            params=params)

    def create_fleet_request(
            self, *,
            sol_id: str,
            fleet_info_mode: Union[ApiFleetInfoMode, Type[Absent]],
    ) -> Request:
        params = {}
        conditional_insert(container=params, key='fleet', value=fleet_info_mode)
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/sol/{sol_id}/fleet',
            params=params)

    def change_fleet_request(
            self, *,
            sol_id: str,
            fleet_id: str,
            add_fits: list[str],
            remove_fits: list[str],
            fleet_info_mode: Union[ApiFleetInfoMode, Type[Absent]],
    ) -> Request:
        body = {}
        conditional_insert(container=body, key='add_fits', value=add_fits)
        conditional_insert(container=body, key='remove_fits', value=remove_fits)
        params = {}
        conditional_insert(container=params, key='fleet', value=fleet_info_mode)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/sol/{sol_id}/fleet/{fleet_id}',
            params=params,
            json=body)

    def remove_fleet_request(
            self, *,
            sol_id: str,
            fleet_id: str,
    ) -> Request:
        return Request(
            self,
            method='DELETE',
            url=f'{self.__base_url}/sol/{sol_id}/fleet/{fleet_id}')

    # Fit methods
    def get_fit_request(
            self, *,
            sol_id: str,
            fit_id: str,
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        params = {}
        conditional_insert(container=params, key='fit', value=fit_info_mode)
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='GET',
            url=f'{self.__base_url}/sol/{sol_id}/fit/{fit_id}',
            params=params)

    def create_fit_request(
            self, *,
            sol_id: str,
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        params = {}
        conditional_insert(container=params, key='fit', value=fit_info_mode)
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/sol/{sol_id}/fit',
            params=params)

    def set_fit_fleet_request(
            self, *,
            sol_id: str,
            fit_id: str,
            fleet_id: Union[str, None],
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        command = {'type': 'set_fleet', 'fleet_id': fleet_id}
        params = {}
        conditional_insert(container=params, key='fit', value=fit_info_mode)
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/sol/{sol_id}/fit/{fit_id}',
            params=params,
            json={'commands': [command]})

    def remove_fit_request(
            self, *,
            sol_id: str,
            fit_id: str,
    ) -> Request:
        return Request(
            self,
            method='DELETE',
            url=f'{self.__base_url}/sol/{sol_id}/fit/{fit_id}')

    # Generic item methods
    def get_item_request(
            self, *,
            sol_id: str,
            item_id: str,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='GET',
            url=f'{self.__base_url}/sol/{sol_id}/item/{item_id}',
            params=params)

    def remove_item_request(
            self, *,
            sol_id: str,
            item_id: str,
    ) -> Request:
        return Request(
            self,
            method='DELETE',
            url=f'{self.__base_url}/sol/{sol_id}/item/{item_id}')

    # Character methods
    def set_char_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__add_simple_item_request(
            cmd_name='character',
            sol_id=sol_id,
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def change_char_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='character',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            item_info_mode=item_info_mode)

    # Skill methods
    def add_skill_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            level: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {
            'type': 'skill',
            'fit_id': fit_id,
            'type_id': type_id,
            'level': level}
        conditional_insert(container=body, key='state', value=state)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def change_skill_request(
            self, *,
            sol_id: str,
            item_id: str,
            level: Union[int, Type[Absent]],
            state: Union[bool, Type[Absent]],
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {'type': 'skill'}
        conditional_insert(container=body, key='level', value=level)
        conditional_insert(container=body, key='state', value=state)
        conditional_insert(container=body, key='effect_modes', value=effect_modes)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/sol/{sol_id}/item/{item_id}',
            params=params,
            json=body)

    # Implant methods
    def add_implant_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__add_simple_item_request(
            cmd_name='implant',
            sol_id=sol_id,
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def change_implant_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='implant',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            item_info_mode=item_info_mode)

    # Booster methods
    def add_booster_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]],
            side_effects: Union[dict[int, bool], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {
            'type': 'booster',
            'fit_id': fit_id,
            'type_id': type_id}
        conditional_insert(container=body, key='state', value=state)
        conditional_insert(container=body, key='side_effects', value=side_effects)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def change_booster_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]],
            side_effects: Union[dict[int, bool], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {'type': 'booster'}
        conditional_insert(container=body, key='state', value=state)
        conditional_insert(container=body, key='side_effects', value=side_effects)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/sol/{sol_id}/item/{item_id}',
            params=params,
            json=body)

    # Ship methods
    def set_ship_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__add_simple_item_request(
            cmd_name='ship',
            sol_id=sol_id,
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def change_ship_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='ship',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            item_info_mode=item_info_mode)

    # Stance methods
    def set_stance_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__add_simple_item_request(
            cmd_name='stance',
            sol_id=sol_id,
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    # Subsystem methods
    def add_subsystem_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__add_simple_item_request(
            cmd_name='subsystem',
            sol_id=sol_id,
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    # Module methods
    def add_mod_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            rack: ApiRack,
            state: ApiState,
            mutation: Union[int, Tuple[int, dict[int, dict]], Type[Absent]],
            charge_type_id: Union[int, Type[Absent]],
            mode: ApiModAddMode,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {
            'type': 'module',
            'fit_id': fit_id,
            'rack': rack,
            'add_mode': mode,
            'type_id': type_id,
            'state': state}
        conditional_insert(container=body, key='mutation', value=mutation)
        conditional_insert(container=body, key='charge_type_id', value=charge_type_id)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def change_mod_request(
            self, *,
            sol_id: str,
            item_id: str,
            state: Union[ApiState, Type[Absent]],
            mutation: Union[int, Tuple[int, dict[int, dict]], dict[int, dict], None, Type[Absent]],
            charge: Union[int, None, Type[Absent]],
            add_projs: Union[Iterable[(str, Union[float, None])], Type[Absent]],
            change_projs: Union[Iterable[(str, Union[float, None])], Type[Absent]],
            rm_projs: Union[Iterable[str], Type[Absent]],
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {'type': 'module'}
        conditional_insert(container=body, key='state', value=state)
        conditional_insert(container=body, key='mutation', value=mutation)
        conditional_insert(container=body, key='charge', value=charge)
        conditional_insert(container=body, key='add_projs', value=add_projs)
        conditional_insert(container=body, key='change_projs', value=change_projs)
        conditional_insert(container=body, key='rm_projs', value=rm_projs)
        conditional_insert(container=body, key='effect_modes', value=effect_modes)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/sol/{sol_id}/item/{item_id}',
            params=params,
            json=body)

    # Rig methods
    def add_rig_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__add_simple_item_request(
            cmd_name='rig',
            sol_id=sol_id,
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def change_rig_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='rig',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            item_info_mode=item_info_mode)

    # Drone methods
    def add_drone_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: ApiState,
            mutation: Union[int, Tuple[int, dict[int, dict[str, float]]], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {
            'type': 'drone',
            'fit_id': fit_id,
            'type_id': type_id}
        conditional_insert(container=body, key='state', value=state)
        conditional_insert(container=body, key='mutation', value=mutation)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def change_drone_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: Union[ApiState, Type[Absent]],
            add_projs: Union[Iterable[(str, Union[float, None])], Type[Absent]],
            change_projs: Union[Iterable[(str, Union[float, None])], Type[Absent]],
            rm_projs: Union[Iterable[str], Type[Absent]],
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {'type': 'drone', 'item_id': item_id}
        conditional_insert(container=body, key='state', value=state)
        conditional_insert(container=body, key='add_projs', value=add_projs)
        conditional_insert(container=body, key='change_projs', value=change_projs)
        conditional_insert(container=body, key='rm_projs', value=rm_projs)
        conditional_insert(container=body, key='effect_modes', value=effect_modes)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/sol/{sol_id}/item/{item_id}',
            params=params,
            json=body)

    # Fighter methods
    def add_fighter_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: ApiState,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__add_simple_item_request(
            cmd_name='fighter',
            sol_id=sol_id,
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def change_fighter_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: Union[ApiState, Type[Absent]],
            add_projs: Union[Iterable[(str, Union[float, None])], Type[Absent]],
            change_projs: Union[Iterable[(str, Union[float, None])], Type[Absent]],
            rm_projs: Union[Iterable[str], Type[Absent]],
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {'type': 'fighter', 'item_id': item_id}
        conditional_insert(container=body, key='state', value=state)
        conditional_insert(container=body, key='add_projs', value=add_projs)
        conditional_insert(container=body, key='change_projs', value=change_projs)
        conditional_insert(container=body, key='rm_projs', value=rm_projs)
        conditional_insert(container=body, key='effect_modes', value=effect_modes)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/sol/{sol_id}/item/{item_id}',
            params=params,
            json=body)

    # Charge methods
    def change_charge_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='charge',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            item_info_mode=item_info_mode)

    # Autocharge methods
    def change_autocharge_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='autocharge',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            item_info_mode=item_info_mode)

    # System-wide effect methods
    def add_sw_effect_request(
            self, *,
            sol_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {'type': 'sw_effect', 'type_id': type_id}
        conditional_insert(container=body, key='state', value=state)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def change_sw_effect_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='sw_effect',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            item_info_mode=item_info_mode)

    # Fit-wide effect methods
    def add_fw_effect_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__add_simple_item_request(
            cmd_name='fw_effect',
            sol_id=sol_id,
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def change_fw_effect_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='fw_effect',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            item_info_mode=item_info_mode)

    # Projected effect methods
    def add_proj_effect_request(
            self, *,
            sol_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {'type': 'proj_effect', 'type_id': type_id}
        conditional_insert(container=body, key='state', value=state)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def change_proj_effect_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]],
            add_projs: Union[Iterable[str], Type[Absent]],
            rm_projs: Union[Iterable[str], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {'type': 'proj_effect', 'item_id': item_id}
        conditional_insert(container=body, key='state', value=state)
        conditional_insert(container=body, key='add_projs', value=add_projs)
        conditional_insert(container=body, key='rm_projs', value=rm_projs)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/sol/{sol_id}/item/{item_id}',
            params=params,
            json=body)

    # Auxiliary methods
    def __add_simple_item_request(
            self, *,
            cmd_name: str,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, str, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {
            'type': cmd_name,
            'fit_id': fit_id,
            'type_id': type_id}
        conditional_insert(container=body, key='state', value=state)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def __change_simple_item_request(
            self, *,
            cmd_name: str,
            sol_id: str,
            item_id: int,
            state: Union[bool, str, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {'type': cmd_name}
        conditional_insert(container=body, key='state', value=state)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/sol/{sol_id}/item/{item_id}',
            params=params,
            json=body)
