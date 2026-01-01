import typing

from fw.api.types import FleetStatsOptions
from fw.request import Request
from fw.util import Absent, conditional_insert
from .base import ApiClientBase

if typing.TYPE_CHECKING:
    from fw.consts import ApiFleetInfoMode


class ApiClientFleet(ApiClientBase):

    def get_fleet_request(
            self, *,
            sol_id: str,
            fleet_id: str,
            fleet_info_mode: ApiFleetInfoMode | type[Absent],
    ) -> Request:
        params = {}
        conditional_insert(container=params, path=['fleet'], value=fleet_info_mode)
        return Request(
            client=self,
            method='GET',
            url=f'{self._base_url}/sol/{sol_id}/fleet/{fleet_id}',
            params=params)

    def get_fleet_stats_request(
            self, *,
            sol_id: str,
            fleet_id: str,
            options: FleetStatsOptions | type[Absent],
    ) -> Request:
        kwargs = {
            'method': 'POST',
            'url': f'{self._base_url}/sol/{sol_id}/fleet/{fleet_id}/stats'}
        # Intentionally send request without body when we don't need it, to test case when the
        # server receives no content-type header
        if isinstance(options, FleetStatsOptions):
            kwargs['json'] = options.to_dict()
        return Request(client=self, **kwargs)

    def create_fleet_request(
            self, *,
            sol_id: str,
            fleet_info_mode: ApiFleetInfoMode | type[Absent],
    ) -> Request:
        params = {}
        conditional_insert(container=params, path=['fleet'], value=fleet_info_mode)
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
        conditional_insert(container=body, path=['add_fits'], value=add_fits)
        conditional_insert(container=body, path=['remove_fits'], value=remove_fits)
        params = {}
        conditional_insert(container=params, path=['fleet'], value=fleet_info_mode)
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
