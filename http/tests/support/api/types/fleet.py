from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.util import AttrDict

if TYPE_CHECKING:
    from tests.support.api import ApiClient
    from tests.support.request import Request


class Fleet(AttrDict):

    def __init__(self, client: ApiClient, data: dict, ss_id: str):
        super().__init__(data=data)
        self._client = client
        self._ss_id = ss_id

    def update_request(self) -> Request:
        return self._client.update_fleet_request(ss_id=self._ss_id, fleet_id=self.id)

    def update(self) -> Fleet:
        resp = self.update_request().send()
        assert resp.status_code == 200
        self._data = resp.json()
        return self

    def remove_request(self) -> Request:
        return self._client.remove_fleet_request(ss_id=self._ss_id, fleet_id=self.id)

    def remove(self) -> None:
        resp = self.remove_request().send()
        assert resp.status_code == 204
