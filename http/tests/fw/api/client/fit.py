from __future__ import annotations

import typing

from tests.fw.request import Request
from tests.fw.util import conditional_insert
from .base import ApiClientBase

if typing.TYPE_CHECKING:
    from tests.fw.api.types.validation import ValOptions
    from tests.fw.consts import ApiFitInfoMode, ApiItemInfoMode, ApiValInfoMode
    from tests.fw.util import Absent


class ApiClientFit(ApiClientBase):

    def get_fit_request(
            self, *,
            sol_id: str,
            fit_id: str,
            fit_info_mode: ApiFitInfoMode | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        params = {}
        conditional_insert(container=params, key='fit', value=fit_info_mode)
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            client=self,
            method='GET',
            url=f'{self._base_url}/sol/{sol_id}/fit/{fit_id}',
            params=params)

    def validate_fit_request(
            self, *,
            sol_id: str,
            fit_id: str,
            options: ValOptions,
            val_info_mode: ApiValInfoMode | type[Absent],
    ) -> Request:
        params = {}
        conditional_insert(container=params, key='validation', value=val_info_mode)
        body = options.to_dict()
        kwargs = {
            'method': 'POST',
            'url': f'{self._base_url}/sol/{sol_id}/fit/{fit_id}/validate',
            'params': params}
        # Intentionally send request without body when we don't need it, to test case when the
        # server receives no content-type header
        if body:
            kwargs['json'] = body
        return Request(client=self, **kwargs)

    def create_fit_request(
            self, *,
            sol_id: str,
            rah_incoming_dmg: tuple[float, float, float, float] | type[Absent],
            fit_info_mode: ApiFitInfoMode | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        params = {}
        conditional_insert(container=params, key='fit', value=fit_info_mode)
        conditional_insert(container=params, key='item', value=item_info_mode)
        body = {}
        conditional_insert(container=body, key='rah_incoming_dmg', value=rah_incoming_dmg)
        kwargs = {
            'method': 'POST',
            'url': f'{self._base_url}/sol/{sol_id}/fit',
            'params': params}
        # Intentionally send request without body when we don't need it, to test case when the
        # server receives no content-type header
        if body:
            kwargs['json'] = body
        return Request(client=self, **kwargs)

    def set_fit_fleet_request(
            self, *,
            sol_id: str,
            fit_id: str,
            fleet_id: str | None,
            fit_info_mode: ApiFitInfoMode | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        command = {'type': 'set_fleet', 'fleet_id': fleet_id}
        params = {}
        conditional_insert(container=params, key='fit', value=fit_info_mode)
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            client=self,
            method='PATCH',
            url=f'{self._base_url}/sol/{sol_id}/fit/{fit_id}',
            params=params,
            json={'commands': [command]})

    def set_fit_rah_incoming_dmg_request(
            self, *,
            sol_id: str,
            fit_id: str,
            dmg_profile: tuple[float, float, float, float] | None | type[Absent],
            fit_info_mode: ApiFitInfoMode | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        command = {'type': 'set_rah_incoming_dmg'}
        conditional_insert(container=command, key='dmg_profile', value=dmg_profile)
        params = {}
        conditional_insert(container=params, key='fit', value=fit_info_mode)
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            client=self,
            method='PATCH',
            url=f'{self._base_url}/sol/{sol_id}/fit/{fit_id}',
            params=params,
            json={'commands': [command]})

    def remove_fit_request(
            self, *,
            sol_id: str,
            fit_id: str,
    ) -> Request:
        return Request(
            client=self,
            method='DELETE',
            url=f'{self._base_url}/sol/{sol_id}/fit/{fit_id}')
