from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.api.exception import ApiRequestError
from tests.support.consts import ApiFitInfoMode, ApiFleetInfoMode, ApiItemInfoMode
from tests.support.util import Absent, AttrDict, AttrHookDef
from .fit import Fit
from .fleet import Fleet
from .item import Item

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support.api import ApiClient
    from tests.support.request import Request


class SolarSystem(AttrDict):

    def __init__(self, client: ApiClient, data: dict):
        super().__init__(
            data=data,
            hooks={
                'fits': AttrHookDef(
                    func=lambda fs: {f.id: f for f in [Fit(client=client, data=f, sol_id=self.id) for f in fs]},
                    default={}),
                'fleets': AttrHookDef(
                    func=lambda fs: {f.id: f for f in [Fleet(client=client, data=f, sol_id=self.id) for f in fs]},
                    default={})})
        self._client = client

    def update_request(self) -> Request:
        return self._client.get_sol_request(sol_id=self.id)

    def update(self, status_code: int = 200) -> Union[SolarSystem, None]:
        resp = self.update_request().send()
        self._client.check_sol(sol_id=self.id)
        if resp.status_code != status_code:
            raise ApiRequestError(expected_code=status_code, received_code=resp.status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def remove_request(self) -> Request:
        return self._client.remove_sol_request(sol_id=self.id)

    def remove(self, status_code: int = 204) -> None:
        resp = self.remove_request().send()
        if resp.status_code != status_code:
            raise ApiRequestError(expected_code=status_code, received_code=resp.status_code)
        if resp.status_code == 204:
            self._client.created_sols.remove(self)

    def check(self) -> None:
        self._client.check_sol(sol_id=self.id)

    # Fleet methods
    def get_fleet_request(
            self,
            fleet_id: str,
            fleet_info_mode: ApiFleetInfoMode = ApiFleetInfoMode.full,
    ) -> Request:
        return self._client.get_fleet_request(
            sol_id=self.id,
            fleet_id=fleet_id,
            fleet_info_mode=fleet_info_mode)

    def get_fleet(
            self,
            fleet_id: str,
            fleet_info_mode: ApiFleetInfoMode = ApiFleetInfoMode.full,
            status_code: int = 200,
    ) -> Union[Fleet, None]:
        resp = self.get_fleet_request(fleet_id=fleet_id, fleet_info_mode=fleet_info_mode).send()
        self.check()
        if resp.status_code != status_code:
            raise ApiRequestError(expected_code=status_code, received_code=resp.status_code)
        if resp.status_code == 200:
            fleet = Fleet(client=self._client, data=resp.json(), sol_id=self.id)
            return fleet
        return None

    def create_fleet_request(self) -> Request:
        return self._client.create_fleet_request(sol_id=self.id)

    def create_fleet(self, status_code: int = 201) -> Union[Fleet, None]:
        resp = self.create_fleet_request().send()
        self.check()
        if resp.status_code != status_code:
            raise ApiRequestError(expected_code=status_code, received_code=resp.status_code)
        if resp.status_code == 201:
            fleet = Fleet(client=self._client, data=resp.json(), sol_id=self.id)
            return fleet
        return None

    # Fit methods
    def get_fit_request(
            self,
            fit_id: str,
            fit_info_mode: ApiFitInfoMode = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.full,
    ) -> Request:
        return self._client.get_fit_request(
            sol_id=self.id,
            fit_id=fit_id,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode)

    def get_fit(
            self,
            fit_id: str,
            fit_info_mode: ApiFitInfoMode = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.full,
            status_code: int = 200,
    ) -> Union[Fit, None]:
        resp = self.get_fit_request(fit_id=fit_id, fit_info_mode=fit_info_mode, item_info_mode=item_info_mode).send()
        self.check()
        if resp.status_code != status_code:
            raise ApiRequestError(expected_code=status_code, received_code=resp.status_code)
        if resp.status_code == 200:
            fit = Fit(client=self._client, data=resp.json(), sol_id=self.id)
            return fit
        return None

    def create_fit_request(self) -> Request:
        return self._client.create_fit_request(sol_id=self.id)

    def create_fit(self, status_code: int = 201) -> Union[Fit, None]:
        resp = self.create_fit_request().send()
        self.check()
        if resp.status_code != status_code:
            raise ApiRequestError(expected_code=status_code, received_code=resp.status_code)
        if resp.status_code == 201:
            fit = Fit(client=self._client, data=resp.json(), sol_id=self.id)
            return fit
        return None

    # Generic item methods
    def get_item_request(
            self,
            item_id: str,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.full,
    ) -> Request:
        return self._client.get_item_request(sol_id=self.id, item_id=item_id, item_info_mode=item_info_mode)

    def get_item(self, item_id: str, status_code: int = 200) -> Union[Item, None]:
        resp = self.get_item_request(item_id=item_id).send()
        self.check()
        if resp.status_code != status_code:
            raise ApiRequestError(expected_code=status_code, received_code=resp.status_code)
        if resp.status_code == 200:
            return Item(client=self._client, data=resp.json(), sol_id=self.id)
        return None

    # System-wide effect methods
    def add_sw_effect_request(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self._client.add_sw_effect_request(sol_id=self.id, type_id=type_id, state=state)

    def add_sw_effect(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self.add_sw_effect_request(type_id=type_id, state=state).send()
        self.check()
        if resp.status_code != status_code:
            raise ApiRequestError(expected_code=status_code, received_code=resp.status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self.id)
            return item
        return None

    def change_sw_effect_request(
            self,
            item_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self._client.change_sw_effect_request(sol_id=self.id, item_id=item_id, state=state)

    # Projected effect methods
    def add_proj_effect_request(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self._client.add_proj_effect_request(sol_id=self.id, type_id=type_id, state=state)

    def add_proj_effect(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self.add_proj_effect_request(type_id=type_id, state=state).send()
        self.check()
        if resp.status_code != status_code:
            raise ApiRequestError(expected_code=status_code, received_code=resp.status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self.id)
            return item
        return None
