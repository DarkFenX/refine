from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.consts import ApiFleetInfoMode
from tests.support.util import AttrDict, AttrHookDef

if TYPE_CHECKING:
    from tests.support.api import ApiClient
    from tests.support.request import Request


class Fleet(AttrDict):

    def __init__(self, client: ApiClient, data: dict, ss_id: str):
        super().__init__(
            data=data,
            hooks={'fits': AttrHookDef(func=lambda fits: fits, default=())})
        self._client = client
        self._ss_id = ss_id

    def update_request(self) -> Request:
        return self._client.get_fleet_request(ss_id=self._ss_id, fleet_id=self.id)

    def update(self) -> Fleet:
        resp = self.update_request().send()
        assert resp.status_code == 200
        self._data = resp.json()
        return self

    def change_request(
            self,
            add_fits: list[str] = (),
            remove_fits: list[str] = (),
            fleet_info_mode: ApiFleetInfoMode = ApiFleetInfoMode.full,
    ) -> Request:
        return self._client.change_fleet_request(
            ss_id=self._ss_id,
            fleet_id=self.id,
            add_fits=add_fits,
            remove_fits=remove_fits,
            fleet_info_mode=fleet_info_mode)

    def change(self, add_fits: list[str] = (), remove_fits: list[str] = ()):
        resp = self.change_request(add_fits=add_fits, remove_fits=remove_fits).send()
        assert resp.status_code == 200
        self._data = resp.json()
        return self

    def remove_request(self) -> Request:
        return self._client.remove_fleet_request(ss_id=self._ss_id, fleet_id=self.id)

    def remove(self) -> None:
        resp = self.remove_request().send()
        assert resp.status_code == 204
