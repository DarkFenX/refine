from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.api.exception import ApiRequestError
from tests.support.consts import ApiFleetInfoMode
from tests.support.util import AttrDict, AttrHookDef

if TYPE_CHECKING:
    from typing import Union

    from tests.support.api import ApiClient
    from tests.support.request import Request


class Fleet(AttrDict):

    def __init__(self, client: ApiClient, data: dict, sol_id: str):
        super().__init__(
            data=data,
            hooks={'fits': AttrHookDef(func=lambda fits: fits, default=())})
        self._client = client
        self._sol_id = sol_id

    def update_request(self, fleet_info_mode: ApiFleetInfoMode) -> Request:
        return self._client.get_fleet_request(
            sol_id=self._sol_id,
            fleet_id=self.id,
            fleet_info_mode=fleet_info_mode)

    def update(
            self,
            fleet_info_mode: ApiFleetInfoMode = ApiFleetInfoMode.full,
            status_code: int = 200
    ) -> Union[Fleet, None]:
        resp = self.update_request(fleet_info_mode=fleet_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        if resp.status_code != status_code:
            raise ApiRequestError(expected_code=status_code, received_code=resp.status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def change_request(
            self,
            add_fits: list[str],
            remove_fits: list[str],
            fleet_info_mode: ApiFleetInfoMode,
    ) -> Request:
        return self._client.change_fleet_request(
            sol_id=self._sol_id,
            fleet_id=self.id,
            add_fits=add_fits,
            remove_fits=remove_fits,
            fleet_info_mode=fleet_info_mode)

    def change(
            self,
            add_fits: list[str] = (),
            remove_fits: list[str] = (),
            fleet_info_mode: ApiFleetInfoMode = ApiFleetInfoMode.full,
            status_code: int = 200,
    ) -> Union[Fleet, None]:
        resp = self.change_request(add_fits=add_fits, remove_fits=remove_fits, fleet_info_mode=fleet_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        if resp.status_code != status_code:
            raise ApiRequestError(expected_code=status_code, received_code=resp.status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def remove_request(self) -> Request:
        return self._client.remove_fleet_request(sol_id=self._sol_id, fleet_id=self.id)

    def remove(self, status_code: int = 204) -> None:
        resp = self.remove_request().send()
        self._client.check_sol(sol_id=self._sol_id)
        if resp.status_code != status_code:
            raise ApiRequestError(expected_code=status_code, received_code=resp.status_code)
