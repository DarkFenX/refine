from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.request import Request
from tests.support.util import conditional_insert
from .base import ApiClientBase

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support.consts import ApiFitInfoMode, ApiItemInfoMode
    from tests.support.util import Absent


class ApiClientFit(ApiClientBase):

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
            url=f'{self._base_url}/sol/{sol_id}/fit/{fit_id}',
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
            url=f'{self._base_url}/sol/{sol_id}/fit',
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
