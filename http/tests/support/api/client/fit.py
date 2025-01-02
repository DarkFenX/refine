from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.request import Request
from tests.support.util import conditional_insert
from .base import ApiClientBase

if TYPE_CHECKING:
    from typing import Union

    from tests.support.consts import ApiFitInfoMode, ApiItemInfoMode
    from tests.support.util import Absent


class ApiClientFit(ApiClientBase):

    def get_fit_request(
            self, *,
            sol_id: str,
            fit_id: str,
            fit_info_mode: Union[ApiFitInfoMode, type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, type[Absent]],
    ) -> Request:
        params = {}
        conditional_insert(container=params, key='fit', value=fit_info_mode)
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='GET',
            url=f'{self._base_url}/sol/{sol_id}/fit/{fit_id}',
            params=params)

    def create_fit_request(
            self, *,
            sol_id: str,
            rah_incoming_dmg: Union[tuple[float, float, float, float], type[Absent]],
            fit_info_mode: Union[ApiFitInfoMode, type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, type[Absent]],
    ) -> Request:
        body = {}
        conditional_insert(container=body, key='rah_incoming_dmg', value=rah_incoming_dmg)
        params = {}
        conditional_insert(container=params, key='fit', value=fit_info_mode)
        conditional_insert(container=params, key='item', value=item_info_mode)
        if body:
            return Request(
                self,
                method='POST',
                url=f'{self._base_url}/sol/{sol_id}/fit',
                params=params,
                json=body)
        # Intentionally send request without body when we don't need it, to test case when the
        # server receives no content-type header
        return Request(
            self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/fit',
            params=params)

    def set_fit_fleet_request(
            self, *,
            sol_id: str,
            fit_id: str,
            fleet_id: Union[str, None],
            fit_info_mode: Union[ApiFitInfoMode, type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, type[Absent]],
    ) -> Request:
        command = {'type': 'set_fleet', 'fleet_id': fleet_id}
        params = {}
        conditional_insert(container=params, key='fit', value=fit_info_mode)
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='PATCH',
            url=f'{self._base_url}/sol/{sol_id}/fit/{fit_id}',
            params=params,
            json={'commands': [command]})

    def set_fit_rah_incoming_dmg_request(
            self, *,
            sol_id: str,
            fit_id: str,
            dmg_profile: Union[tuple[float, float, float, float], None, type[Absent]],
            fit_info_mode: Union[ApiFitInfoMode, type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, type[Absent]],
    ) -> Request:
        command = {'type': 'set_rah_incoming_dmg'}
        conditional_insert(container=command, key='dmg_profile', value=dmg_profile)
        params = {}
        conditional_insert(container=params, key='fit', value=fit_info_mode)
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
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
            self,
            method='DELETE',
            url=f'{self._base_url}/sol/{sol_id}/fit/{fit_id}')
