from __future__ import annotations

import typing

from tests.fw import eve
from tests.fw.api.types import SolarSystem
from tests.fw.consts import ApiSolInfoMode
from tests.fw.request import Request
from tests.fw.util import Absent, Default, conditional_insert
from .base import ApiClientBase

if typing.TYPE_CHECKING:
    from tests.fw.api.aliases import DpsProfile
    from tests.fw.consts import ApiFitInfoMode, ApiFleetInfoMode, ApiItemInfoMode, ApiSecZone


class ApiSolCheckError(Exception):
    pass


class ApiClientSol(ApiClientBase, eve.EveDataManager):

    def __init__(self, **kwargs) -> None:
        super().__init__(**kwargs)
        self.__created_sols: set[SolarSystem] = set()

    def create_sol_request(
            self, *,
            data: eve.EveObjects | type[Absent | Default],
            sec_zone: ApiSecZone | type[Absent],
            default_incoming_dps: DpsProfile | type[Absent],
            sol_info_mode: ApiSolInfoMode | type[Absent],
            fleet_info_mode: ApiFleetInfoMode | type[Absent],
            fit_info_mode: ApiFitInfoMode | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        params = {}
        conditional_insert(container=params, path=['sol'], value=sol_info_mode)
        conditional_insert(container=params, path=['fleet'], value=fleet_info_mode)
        conditional_insert(container=params, path=['fit'], value=fit_info_mode)
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        body = {}
        if data is not Absent:
            if data is Default:
                data = self._get_default_eve_data()
            body['src_alias'] = data.alias
        conditional_insert(container=body, path=['sec_zone'], value=sec_zone)
        conditional_insert(container=body, path=['default_incoming_dps'], value=default_incoming_dps)
        kwargs = {
            'method': 'POST',
            'url': f'{self._base_url}/sol',
            'params': params}
        # Intentionally send request without body when we don't need it, to test case when the
        # server receives no content-type header
        if body:
            kwargs['json'] = body
        return Request(client=self, **kwargs)

    def create_sol(
            self, *,
            data: eve.EveObjects | type[Absent | Default] = Default,
            sec_zone: ApiSecZone | type[Absent] = Absent,
            default_incoming_dps: DpsProfile | type[Absent] = Absent,
            sol_info_mode: ApiSolInfoMode | type[Absent] = ApiSolInfoMode.id,
            fleet_info_mode: ApiFleetInfoMode | type[Absent] = Absent,
            fit_info_mode: ApiFitInfoMode | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = Absent,
    ) -> SolarSystem:
        if data is Default:
            data = self._get_default_eve_data()
        resp = self.create_sol_request(
            data=data,
            sec_zone=sec_zone,
            default_incoming_dps=default_incoming_dps,
            sol_info_mode=sol_info_mode,
            fleet_info_mode=fleet_info_mode,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        assert resp.status_code == 201
        sol_sys = SolarSystem(client=self, data=resp.json())
        self.__created_sols.add(sol_sys)
        return sol_sys

    @property
    def created_sols(self) -> set[SolarSystem]:
        return self.__created_sols

    def get_sol(
            self, *,
            sol_id: str,
            sol_info_mode: ApiSolInfoMode | type[Absent],
            fleet_info_mode: ApiFleetInfoMode | type[Absent],
            fit_info_mode: ApiFitInfoMode | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
            status_code: int = 200,
            json_predicate: dict | None = None,
    ) -> SolarSystem | None:
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
            sol_info_mode: ApiSolInfoMode | type[Absent],
            fleet_info_mode: ApiFleetInfoMode | type[Absent],
            fit_info_mode: ApiFitInfoMode | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        params = {}
        conditional_insert(container=params, path=['sol'], value=sol_info_mode)
        conditional_insert(container=params, path=['fleet'], value=fleet_info_mode)
        conditional_insert(container=params, path=['fit'], value=fit_info_mode)
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='GET',
            url=f'{self._base_url}/sol/{sol_id}',
            params=params)

    def change_sol_src_request(
            self, *,
            sol_id: str,
            data: eve.EveObjects | type[Absent | Default],
            sol_info_mode: ApiSolInfoMode | type[Absent],
            fleet_info_mode: ApiFleetInfoMode | type[Absent],
            fit_info_mode: ApiFitInfoMode | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = {}
        if data is not Absent:
            if data is Default:
                data = self._get_default_eve_data()
            body['src_alias'] = data.alias
        params = {}
        conditional_insert(container=params, path=['sol'], value=sol_info_mode)
        conditional_insert(container=params, path=['fleet'], value=fleet_info_mode)
        conditional_insert(container=params, path=['fit'], value=fit_info_mode)
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='PATCH',
            url=f'{self._base_url}/sol/{sol_id}/src',
            params=params,
            json=body)

    def remove_sol_request(self, *, sol_id: str) -> Request:
        return Request(
            client=self,
            method='DELETE',
            url=f'{self._base_url}/sol/{sol_id}')

    def cleanup_sols(self) -> None:
        for sol in self.__created_sols.copy():
            sol.remove()

    def change_sol_request(
            self, *,
            sol_id: str,
            sec_zone: ApiSecZone | type[Absent],
            default_incoming_dps: DpsProfile | type[Absent],
            sol_info_mode: ApiSolInfoMode | type[Absent],
            fleet_info_mode: ApiFleetInfoMode | type[Absent],
            fit_info_mode: ApiFitInfoMode | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        command = {'type': 'change_sol'}
        conditional_insert(container=command, path=['sec_zone'], value=sec_zone)
        conditional_insert(container=command, path=['default_incoming_dps'], value=default_incoming_dps)
        params = {}
        conditional_insert(container=params, path=['sol'], value=sol_info_mode)
        conditional_insert(container=params, path=['fleet'], value=fleet_info_mode)
        conditional_insert(container=params, path=['fit'], value=fit_info_mode)
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='PATCH',
            url=f'{self._base_url}/sol/{sol_id}',
            params=params,
            json={'commands': [command]})

    # Development-specific requests
    def check_sol(self, *, sol_id: str) -> None:
        resp = self.check_sol_request(sol_id=sol_id).send()
        if resp.status_code != 200:
            raise ApiSolCheckError

    def check_sol_request(self, *, sol_id: str) -> Request:
        return Request(
            client=self,
            method='GET',
            url=f'{self._base_url}/sol/{sol_id}/check')

    def benchmark_sol_request(self, *, sol_id: str, command: dict) -> Request:
        return Request(
            client=self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/benchmark',
            json=command)
