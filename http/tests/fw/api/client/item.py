import typing

from fw.api.commands import ItemItemRemoveCmd
from fw.api.types import ItemStatsOptions
from fw.request import Request
from fw.util import conditional_insert
from .base import ApiClientBase

if typing.TYPE_CHECKING:
    from fw.api.commands import BaseCommand
    from fw.consts import ApiItemInfoMode, ApiModRmMode
    from fw.util import Absent


class ApiClientItem(ApiClientBase):

    # Generic item methods
    def get_item_request(
            self, *,
            sol_id: str,
            item_id: str,
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        params = {}
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='GET',
            url=f'{self._base_url}/sol/{sol_id}/item/{item_id}',
            params=params)

    def remove_item_request(
            self, *,
            sol_id: str,
            item_id: str,
            rm_mode: ApiModRmMode | type[Absent],
    ) -> Request:
        body = ItemItemRemoveCmd(rm_mode=rm_mode).serialize()
        kwargs = {'method': 'DELETE', 'url': f'{self._base_url}/sol/{sol_id}/item/{item_id}'}
        # Intentionally send request without body when we don't need it, to test case when the
        # server receives no content-type header
        if body:
            kwargs['json'] = body
        return Request(client=self, **kwargs)

    def get_item_stats_request(
            self, *,
            sol_id: str,
            item_id: str,
            options: ItemStatsOptions | type[Absent],
    ) -> Request:
        kwargs = {
            'method': 'POST',
            'url': f'{self._base_url}/sol/{sol_id}/item/{item_id}/stats'}
        # Intentionally send request without body when we don't need it, to test case when the
        # server receives no content-type header
        if isinstance(options, ItemStatsOptions):
            kwargs['json'] = options.to_dict()
        return Request(client=self, **kwargs)

    # Auxiliary methods
    def item_command_add_request(
            self, *,
            sol_id: str,
            command: BaseCommand,
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        params = {}
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/item',
            params=params,
            json=command.serialize())

    def item_command_change_request(
            self, *,
            sol_id: str,
            item_id: str,
            command: BaseCommand,
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        params = {}
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='PATCH',
            url=f'{self._base_url}/sol/{sol_id}/item/{item_id}',
            params=params,
            json=command.serialize())
