from __future__ import annotations

import typing

from tests.fw.request import Request
from tests.fw.util import conditional_insert
from .base import ApiClientBase

if typing.TYPE_CHECKING:
    from tests.fw.consts import ApiFleetInfoMode
    from tests.fw.util import Absent


class ApiClientFleet(ApiClientBase):

    def get_fleet_request(
            self, *,
            sol_id: str,
            fleet_id: str,
            fleet_info_mode: ApiFleetInfoMode | type[Absent],
    ) -> Request:
        params = {}
        conditional_insert(container=params, key='fleet', value=fleet_info_mode)
        return Request(
            client=self,
            method='GET',
            url=f'{self._base_url}/sol/{sol_id}/fleet/{fleet_id}',
            params=params)

    def create_fleet_request(
            self, *,
            sol_id: str,
            fleet_info_mode: ApiFleetInfoMode | type[Absent],
    ) -> Request:
        params = {}
        conditional_insert(container=params, key='fleet', value=fleet_info_mode)
        return Request(
            client=self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/fleet',
            params=params)

    def change_fleet_request(
            self, *,
            sol_id: str,
            fleet_id: str,
            add_fits: list[str],
            remove_fits: list[str],
            fleet_info_mode: ApiFleetInfoMode | type[Absent],
    ) -> Request:
        body = {}
        conditional_insert(container=body, key='add_fits', value=add_fits)
        conditional_insert(container=body, key='remove_fits', value=remove_fits)
        params = {}
        conditional_insert(container=params, key='fleet', value=fleet_info_mode)
        return Request(
            client=self,
            method='PATCH',
            url=f'{self._base_url}/sol/{sol_id}/fleet/{fleet_id}',
            params=params,
            json=body)

    def remove_fleet_request(
            self, *,
            sol_id: str,
            fleet_id: str,
    ) -> Request:
        return Request(
            client=self,
            method='DELETE',
            url=f'{self._base_url}/sol/{sol_id}/fleet/{fleet_id}')
