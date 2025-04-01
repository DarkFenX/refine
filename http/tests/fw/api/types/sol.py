from __future__ import annotations

import typing

from tests.fw.consts import ApiFitInfoMode, ApiFleetInfoMode, ApiItemInfoMode, ApiSolInfoMode
from tests.fw.util import Absent, AttrDict, AttrHookDef, Default
from .dmg_types import DmgTypes
from .fit import Fit
from .fleet import Fleet
from .item import Item

if typing.TYPE_CHECKING:
    from tests.fw import eve
    from tests.fw.api import ApiClient
    from tests.fw.consts import ApiSecZone


class SolarSystem(AttrDict):

    def __init__(self, *, client: ApiClient, data: dict) -> None:
        super().__init__(data=data, hooks={
            'default_incoming_dps': AttrHookDef(
                func=lambda dp: DmgTypes(em=dp[0], thermal=dp[1], kinetic=dp[2], explosive=dp[3])),
            'fits': AttrHookDef(
                func=lambda fs: {f.id: f for f in [Fit(client=client, data=f, sol_id=self.id) for f in fs]}),
            'fleets': AttrHookDef(
                func=lambda fs: {f.id: f for f in [Fleet(client=client, data=f, sol_id=self.id) for f in fs]})})
        self._client = client

    def update(
            self, *,
            sol_info_mode: ApiSolInfoMode | type[Absent] = ApiSolInfoMode.full,
            fleet_info_mode: ApiFleetInfoMode | type[Absent] = ApiFleetInfoMode.id,
            fit_info_mode: ApiFitInfoMode | type[Absent] = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> SolarSystem | None:
        resp = self._client.get_sol_request(
            sol_id=self.id,
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

    def change_src(
            self, *,
            data: eve.EveObjects | type[Absent | Default] = Default,
            sol_info_mode: ApiSolInfoMode | type[Absent] = ApiSolInfoMode.full,
            fleet_info_mode: ApiFleetInfoMode | type[Absent] = ApiFleetInfoMode.id,
            fit_info_mode: ApiFitInfoMode | type[Absent] = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> SolarSystem:
        resp = self._client.change_sol_src_request(
            sol_id=self.id,
            data=data,
            sol_info_mode=sol_info_mode,
            fleet_info_mode=fleet_info_mode,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self.id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
        return self

    def remove(self, *, status_code: int = 204) -> None:
        resp = self._client.remove_sol_request(sol_id=self.id).send()
        resp.check(status_code=status_code)
        if resp.status_code == 204:
            self._client.created_sols.remove(self)

    def check(self) -> None:
        self._client.check_sol(sol_id=self.id)

    def change(
            self, *,
            sec_zone: ApiSecZone | type[Absent] = Absent,
            default_incoming_dps: tuple[float, float, float, float] | type[Absent] = Absent,
            sol_info_mode: ApiSolInfoMode | type[Absent] = ApiSolInfoMode.id,
            fleet_info_mode: ApiFleetInfoMode | type[Absent] = ApiFleetInfoMode.id,
            fit_info_mode: ApiFitInfoMode | type[Absent] = ApiFitInfoMode.id,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> SolarSystem:
        resp = self._client.change_sol_request(
            sol_id=self.id,
            sec_zone=sec_zone,
            default_incoming_dps=default_incoming_dps,
            sol_info_mode=sol_info_mode,
            fleet_info_mode=fleet_info_mode,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self.check()
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()['solar_system']
        return self

    # Fleet methods
    def get_fleet(
            self, *,
            fleet_id: str,
            fleet_info_mode: ApiFleetInfoMode | type[Absent] = ApiFleetInfoMode.full,
            status_code: int = 200,
            json_predicate: dict | None = None,
    ) -> Fleet | None:
        resp = self._client.get_fleet_request(
            sol_id=self.id,
            fleet_id=fleet_id,
            fleet_info_mode=fleet_info_mode).send()
        self.check()
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            return Fleet(client=self._client, data=resp.json(), sol_id=self.id)
        return None

    def create_fleet(
            self, *,
            fleet_info_mode: ApiFleetInfoMode | type[Absent] = ApiFleetInfoMode.id,
            status_code: int = 201,
    ) -> Fleet | None:
        resp = self._client.create_fleet_request(sol_id=self.id, fleet_info_mode=fleet_info_mode).send()
        self.check()
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            return Fleet(client=self._client, data=resp.json(), sol_id=self.id)
        return None

    # Fit methods
    def get_fit(
            self, *,
            fit_id: str,
            fit_info_mode: ApiFitInfoMode | type[Absent] = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
            json_predicate: dict | None = None,
    ) -> Fit | None:
        resp = self._client.get_fit_request(
            sol_id=self.id,
            fit_id=fit_id,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self.check()
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            return Fit(client=self._client, data=resp.json(), sol_id=self.id)
        return None

    def create_fit(
            self, *,
            sec_status: float | type[Absent] = Absent,
            rah_incoming_dps: tuple[float, float, float, float] | type[Absent] = Absent,
            fit_info_mode: ApiFitInfoMode | type[Absent] = ApiFitInfoMode.id,
            item_info_mode: ApiItemInfoMode | type[Absent] = Absent,
            status_code: int = 201,
    ) -> Fit | None:
        resp = self._client.create_fit_request(
            sol_id=self.id,
            sec_status=sec_status,
            rah_incoming_dps=rah_incoming_dps,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self.check()
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            return Fit(client=self._client, data=resp.json(), sol_id=self.id)
        return None

    # Item methods
    def get_item(
            self, *,
            item_id: str,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.full,
            status_code: int = 200,
            json_predicate: dict | None = None,
    ) -> Item | None:
        resp = self._client.get_item_request(sol_id=self.id, item_id=item_id, item_info_mode=item_info_mode).send()
        self.check()
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            return Item(client=self._client, data=resp.json(), sol_id=self.id)
        return None

    def add_proj_effect(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        resp = self._client.add_proj_effect_request(
            sol_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode).send()
        self.check()
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self.id)
        return None

    def add_sw_effect(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        resp = self._client.add_sw_effect_request(
            sol_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode).send()
        self.check()
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self.id)
        return None
