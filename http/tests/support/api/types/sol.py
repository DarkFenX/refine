from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.consts import ApiFitInfoMode, ApiFleetInfoMode, ApiItemInfoMode, ApiSolInfoMode
from tests.support.util import Absent, AttrDict, AttrHookDef, Default
from .fit import Fit
from .fleet import Fleet
from .item import Item

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support import eve
    from tests.support.api import ApiClient
    from tests.support.request import Request


class SolarSystem(AttrDict):

    def __init__(self, client: ApiClient, data: dict):
        super().__init__(
            data=data,
            hooks={
                'fits': AttrHookDef(
                    func=lambda fs: {f.id: f for f in [Fit(client=client, data=f, sol_id=self.id) for f in fs]}),
                'fleets': AttrHookDef(
                    func=lambda fs: {f.id: f for f in [Fleet(client=client, data=f, sol_id=self.id) for f in fs]})})
        self._client = client

    def update_request(
            self,
            sol_info_mode: Union[ApiSolInfoMode, Type[Absent]],
            fleet_info_mode: Union[ApiFleetInfoMode, Type[Absent]],
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.get_sol_request(
            sol_id=self.id,
            sol_info_mode=sol_info_mode,
            fleet_info_mode=fleet_info_mode,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode)

    def update(
            self,
            sol_info_mode: Union[ApiSolInfoMode, Type[Absent]] = ApiSolInfoMode.full,
            fleet_info_mode: Union[ApiFleetInfoMode, Type[Absent]] = ApiFleetInfoMode.id,
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]] = ApiFitInfoMode.full,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Union[SolarSystem, None]:
        resp = self.update_request(
            sol_info_mode=sol_info_mode,
            fleet_info_mode=fleet_info_mode,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self.id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def change_src_request(
            self,
            data: Union[eve.EveObjects, Type[Absent], Type[Default]],
            sol_info_mode: Union[ApiSolInfoMode, Type[Absent]],
            fleet_info_mode: Union[ApiFleetInfoMode, Type[Absent]],
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.change_sol_src_request(
            sol_id=self.id,
            data=data,
            sol_info_mode=sol_info_mode,
            fleet_info_mode=fleet_info_mode,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode)

    def change_src(
            self,
            data: Union[eve.EveObjects, Type[Absent], Type[Default]] = Default,
            sol_info_mode: Union[ApiSolInfoMode, Type[Absent]] = ApiSolInfoMode.full,
            fleet_info_mode: Union[ApiFleetInfoMode, Type[Absent]] = ApiFleetInfoMode.id,
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]] = ApiFitInfoMode.full,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> SolarSystem:
        resp = self.change_src_request(
            data=data,
            sol_info_mode=sol_info_mode,
            fleet_info_mode=fleet_info_mode,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode,
        ).send()
        self._client.check_sol(sol_id=self.id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
        return self

    def remove_request(self) -> Request:
        return self._client.remove_sol_request(sol_id=self.id)

    def remove(self, status_code: int = 204) -> None:
        resp = self.remove_request().send()
        resp.check(status_code=status_code)
        if resp.status_code == 204:
            self._client.created_sols.remove(self)

    def check(self) -> None:
        self._client.check_sol(sol_id=self.id)

    # Fleet methods
    def get_fleet_request(
            self,
            fleet_id: str,
            fleet_info_mode: Union[ApiFleetInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.get_fleet_request(
            sol_id=self.id,
            fleet_id=fleet_id,
            fleet_info_mode=fleet_info_mode)

    def get_fleet(
            self,
            fleet_id: str,
            fleet_info_mode: Union[ApiFleetInfoMode, Type[Absent]] = ApiFleetInfoMode.full,
            status_code: int = 200,
            json_predicate: Union[dict, None] = None,
    ) -> Union[Fleet, None]:
        resp = self.get_fleet_request(fleet_id=fleet_id, fleet_info_mode=fleet_info_mode).send()
        self.check()
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            fleet = Fleet(client=self._client, data=resp.json(), sol_id=self.id)
            return fleet
        return None

    def create_fleet_request(
            self,
            fleet_info_mode: Union[ApiFleetInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.create_fleet_request(sol_id=self.id, fleet_info_mode=fleet_info_mode)

    def create_fleet(
            self,
            fleet_info_mode: Union[ApiFleetInfoMode, Type[Absent]] = ApiFleetInfoMode.id,
            status_code: int = 201,
    ) -> Union[Fleet, None]:
        resp = self.create_fleet_request(fleet_info_mode=fleet_info_mode).send()
        self.check()
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            fleet = Fleet(client=self._client, data=resp.json(), sol_id=self.id)
            return fleet
        return None

    # Fit methods
    def get_fit_request(
            self,
            fit_id: str,
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.get_fit_request(
            sol_id=self.id,
            fit_id=fit_id,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode)

    def get_fit(
            self,
            fit_id: str,
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]] = ApiFitInfoMode.full,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
            json_predicate: Union[dict, None] = None,
    ) -> Union[Fit, None]:
        resp = self.get_fit_request(fit_id=fit_id, fit_info_mode=fit_info_mode, item_info_mode=item_info_mode).send()
        self.check()
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            fit = Fit(client=self._client, data=resp.json(), sol_id=self.id)
            return fit
        return None

    def create_fit_request(
            self,
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.create_fit_request(
            sol_id=self.id,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode)

    def create_fit(
            self,
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]] = ApiFitInfoMode.id,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = Absent,
            status_code: int = 201,
    ) -> Union[Fit, None]:
        resp = self.create_fit_request(fit_info_mode=fit_info_mode, item_info_mode=item_info_mode).send()
        self.check()
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            fit = Fit(client=self._client, data=resp.json(), sol_id=self.id)
            return fit
        return None

    # Generic item methods
    def get_item_request(
            self,
            item_id: str,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.get_item_request(sol_id=self.id, item_id=item_id, item_info_mode=item_info_mode)

    def get_item(
            self,
            item_id: str,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.full,
            status_code: int = 200,
            json_predicate: Union[dict, None] = None,
    ) -> Union[Item, None]:
        resp = self.get_item_request(item_id=item_id, item_info_mode=item_info_mode).send()
        self.check()
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            return Item(client=self._client, data=resp.json(), sol_id=self.id)
        return None

    # System-wide effect methods
    def add_sw_effect_request(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.add_sw_effect_request(
            sol_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def add_sw_effect(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self.add_sw_effect_request(type_id=type_id, state=state, item_info_mode=item_info_mode).send()
        self.check()
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self.id)
            return item
        return None

    # Projected effect methods
    def add_proj_effect_request(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.add_proj_effect_request(
            sol_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def add_proj_effect(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self.add_proj_effect_request(type_id=type_id, state=state, item_info_mode=item_info_mode).send()
        self.check()
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self.id)
            return item
        return None
