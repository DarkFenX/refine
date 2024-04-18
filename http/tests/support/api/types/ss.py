from __future__ import annotations

from typing import TYPE_CHECKING

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
                    func=lambda fs: {f.id: f for f in [Fit(client=client, data=f, ss_id=self.id) for f in fs]},
                    default={}),
                'fleets': AttrHookDef(
                    func=lambda fs: {f.id: f for f in [Fleet(client=client, data=f, ss_id=self.id) for f in fs]},
                    default={})})
        self._client = client

    def update_request(self) -> Request:
        return self._client.get_ss_request(ss_id=self.id)

    def update(self) -> SolarSystem:
        resp = self.update_request().send()
        assert resp.status_code == 200
        self._client.check_ss(ss_id=self.id)
        self._data = resp.json()
        return self

    def remove_request(self) -> Request:
        return self._client.remove_ss_request(ss_id=self.id)

    def remove(self) -> None:
        resp = self.remove_request().send()
        assert resp.status_code == 204
        self._client.created_sss.remove(self)

    def check(self) -> None:
        self._client.check_ss(ss_id=self.id)

    # Fleet methods
    def get_fleet_request(
            self,
            fleet_id: str,
            fleet_info_mode: ApiFleetInfoMode = ApiFleetInfoMode.full,
    ) -> Request:
        return self._client.get_fleet_request(
            ss_id=self.id,
            fleet_id=fleet_id,
            fleet_info_mode=fleet_info_mode)

    def get_fleet(
            self,
            fleet_id: str,
            fleet_info_mode: ApiFleetInfoMode = ApiFleetInfoMode.full,
    ) -> Fleet:
        resp = self.get_fleet_request(fleet_id=fleet_id, fleet_info_mode=fleet_info_mode).send()
        assert resp.status_code == 200
        self._client.check_ss(ss_id=self.id)
        fleet = Fleet(client=self._client, data=resp.json(), ss_id=self.id)
        return fleet

    def create_fleet_request(self) -> Request:
        return self._client.create_fleet_request(ss_id=self.id)

    def create_fleet(self) -> Fleet:
        resp = self.create_fleet_request().send()
        assert resp.status_code == 201
        self._client.check_ss(ss_id=self.id)
        fleet = Fleet(client=self._client, data=resp.json(), ss_id=self.id)
        return fleet

    # Fit methods
    def get_fit_request(
            self,
            fit_id: str,
            fit_info_mode: ApiFitInfoMode = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.full,
    ) -> Request:
        return self._client.get_fit_request(
            ss_id=self.id,
            fit_id=fit_id,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode)

    def get_fit(
            self,
            fit_id: str,
            fit_info_mode: ApiFitInfoMode = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.full,
    ) -> Fit:
        resp = self.get_fit_request(fit_id=fit_id, fit_info_mode=fit_info_mode, item_info_mode=item_info_mode).send()
        assert resp.status_code == 200
        self._client.check_ss(ss_id=self.id)
        fit = Fit(client=self._client, data=resp.json(), ss_id=self.id)
        return fit

    def create_fit_request(self) -> Request:
        return self._client.create_fit_request(ss_id=self.id)

    def create_fit(self) -> Fit:
        resp = self.create_fit_request().send()
        assert resp.status_code == 201
        self._client.check_ss(ss_id=self.id)
        fit = Fit(client=self._client, data=resp.json(), ss_id=self.id)
        return fit

    # Generic item methods
    def get_item_request(
            self,
            item_id: str,
            item_info_mode: ApiItemInfoMode = ApiItemInfoMode.full,
    ) -> Request:
        return self._client.get_item_request(ss_id=self.id, item_id=item_id, item_info_mode=item_info_mode)

    def get_item(self, item_id: str) -> Item:
        resp = self.get_item_request(item_id=item_id).send()
        assert resp.status_code == 200
        self._client.check_ss(ss_id=self.id)
        return Item(client=self._client, data=resp.json(), ss_id=self.id)

    # System-wide effect methods
    def add_sw_effect_request(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self._client.add_sw_effect_request(ss_id=self.id, type_id=type_id, state=state)

    def add_sw_effect(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Item:
        resp = self.add_sw_effect_request(type_id=type_id, state=state).send()
        assert resp.status_code == 201
        self._client.check_ss(ss_id=self.id)
        item = Item(client=self._client, data=resp.json(), ss_id=self.id)
        return item

    def change_sw_effect_request(
            self,
            item_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self._client.change_sw_effect_request(ss_id=self.id, item_id=item_id, state=state)

    # Projected effect methods
    def add_proj_effect_request(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self._client.add_proj_effect_request(ss_id=self.id, type_id=type_id, state=state)

    def add_proj_effect(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Item:
        resp = self.add_proj_effect_request(type_id=type_id, state=state).send()
        assert resp.status_code == 201
        self._client.check_ss(ss_id=self.id)
        item = Item(client=self._client, data=resp.json(), ss_id=self.id)
        return item
