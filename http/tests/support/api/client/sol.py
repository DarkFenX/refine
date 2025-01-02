from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support import eve
from tests.support.api.types import SolarSystem
from tests.support.consts import ApiSolInfoMode
from tests.support.request import Request
from tests.support.util import Absent, Default, conditional_insert
from .base import ApiClientBase

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support.consts import ApiFitInfoMode, ApiFleetInfoMode, ApiItemInfoMode


class ApiSolCheckError(Exception):
    pass


class ApiClientSol(ApiClientBase, eve.EveDataManager):

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.__created_sols: set[SolarSystem] = set()

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
        return Request(self, method='POST', url=f'{self._base_url}/sol', params=params, json=body)

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
            url=f'{self._base_url}/sol/{sol_id}',
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
            url=f'{self._base_url}/sol/{sol_id}/src',
            params=params,
            json=body)

    def remove_sol_request(self, *, sol_id: str) -> Request:
        return Request(
            self,
            method='DELETE',
            url=f'{self._base_url}/sol/{sol_id}')

    def check_sol(self, *, sol_id: str) -> None:
        resp = self.check_sol_request(sol_id=sol_id).send()
        if resp.status_code != 200:
            raise ApiSolCheckError

    def check_sol_request(self, *, sol_id: str) -> Request:
        return Request(
            self,
            method='GET',
            url=f'{self._base_url}/sol/{sol_id}/check')

    def cleanup_sols(self) -> None:
        for sol in self.__created_sols.copy():
            sol.remove()

    def change_sol_default_incoming_dmg_request(
            self, *,
            sol_id: str,
            dmg_profile: Union[tuple[float, float, float, float], Type[Absent]],
            sol_info_mode: Union[ApiSolInfoMode, Type[Absent]],
            fleet_info_mode: Union[ApiFleetInfoMode, Type[Absent]],
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        command = {'type': 'change_default_incoming_dmg'}
        conditional_insert(container=command, key='dmg_profile', value=dmg_profile)
        params = {}
        conditional_insert(container=params, key='sol', value=sol_info_mode)
        conditional_insert(container=params, key='fleet', value=fleet_info_mode)
        conditional_insert(container=params, key='fit', value=fit_info_mode)
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='PATCH',
            url=f'{self._base_url}/sol/{sol_id}',
            params=params,
            json={'commands': [command]})
