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
        conditional_insert(container=params, path=['fit'], value=fit_info_mode)
        conditional_insert(container=params, path=['item'], value=item_info_mode)
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
        conditional_insert(container=params, path=['validation'], value=val_info_mode)
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
            sec_status: float | type[Absent],
            rah_incoming_dmg: tuple[float, float, float, float] | type[Absent],
            fit_info_mode: ApiFitInfoMode | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        params = {}
        conditional_insert(container=params, path=['fit'], value=fit_info_mode)
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        body = {}
        conditional_insert(container=body, path=['sec_status'], value=sec_status)
        conditional_insert(container=body, path=['rah_incoming_dmg'], value=rah_incoming_dmg)
        kwargs = {
            'method': 'POST',
            'url': f'{self._base_url}/sol/{sol_id}/fit',
            'params': params}
        # Intentionally send request without body when we don't need it, to test case when the
        # server receives no content-type header
        if body:
            kwargs['json'] = body
        return Request(client=self, **kwargs)

    def change_fit_request(
            self, *,
            sol_id: str,
            fit_id: str,
            fleet_id: str | None | type[Absent],
            sec_status: float | type[Absent],
            rah_incoming_dmg: tuple[float, float, float, float] | None | type[Absent],
            fit_info_mode: ApiFitInfoMode | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        command = {'type': 'change_fit'}
        conditional_insert(container=command, path=['fleet_id'], value=fleet_id)
        conditional_insert(container=command, path=['sec_status'], value=sec_status)
        conditional_insert(container=command, path=['rah_incoming_dmg'], value=rah_incoming_dmg)
        params = {}
        conditional_insert(container=params, path=['fit'], value=fit_info_mode)
        conditional_insert(container=params, path=['item'], value=item_info_mode)
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

    def remove_fit_character_request(
            self, *,
            sol_id: str,
            fit_id: str,
            fit_info_mode: ApiFitInfoMode | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__simple_remove_fit_item_request(
            cmd_name='remove_character',
            sol_id=sol_id,
            fit_id=fit_id,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode)

    def remove_fit_ship_request(
            self, *,
            sol_id: str,
            fit_id: str,
            fit_info_mode: ApiFitInfoMode | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__simple_remove_fit_item_request(
            cmd_name='remove_ship',
            sol_id=sol_id,
            fit_id=fit_id,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode)

    def remove_fit_stance_request(
            self, *,
            sol_id: str,
            fit_id: str,
            fit_info_mode: ApiFitInfoMode | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__simple_remove_fit_item_request(
            cmd_name='remove_stance',
            sol_id=sol_id,
            fit_id=fit_id,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode)

    def __simple_remove_fit_item_request(
            self, *,
            cmd_name: str,
            sol_id: str,
            fit_id: str,
            fit_info_mode: ApiFitInfoMode | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        command = {'type': cmd_name}
        params = {}
        conditional_insert(container=params, path=['fit'], value=fit_info_mode)
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='PATCH',
            url=f'{self._base_url}/sol/{sol_id}/fit/{fit_id}',
            params=params,
            json={'commands': [command]})
