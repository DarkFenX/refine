import typing

from fw.api.commands import FleetFleetAddCmd, FleetFleetChangeCmd
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
            fit_ids: list[str] | type[Absent],
            fleet_info_mode: ApiFleetInfoMode | type[Absent],
    ) -> Request:
        # Body
        body = FleetFleetAddCmd(fit_ids=fit_ids).serialize()
        # Params
        params = {}
        conditional_insert(container=params, path=['fleet'], value=fleet_info_mode)
        # Make request
        kwargs = {
            'method': 'POST',
            'url': f'{self._base_url}/sol/{sol_id}/fleet',
            'params': params}
        # Intentionally send request without body when we don't need it, to test case when the
        # server receives no content-type header
        if body:
            kwargs['json'] = body
        return Request(client=self, **kwargs)

    def change_fleet_request(
            self, *,
            sol_id: str,
            fleet_id: str,
            add_fit_ids: list[str] | type[Absent],
            rm_fit_ids: list[str] | type[Absent],
            fleet_info_mode: ApiFleetInfoMode | type[Absent],
    ) -> Request:
        body = FleetFleetChangeCmd(
            add_fit_ids=add_fit_ids,
            rm_fit_ids=rm_fit_ids).serialize()
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
