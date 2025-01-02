from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.consts import ApiFitInfoMode, ApiFleetInfoMode, ApiItemInfoMode, ApiSolInfoMode
from tests.support.util import Absent, AttrDict, AttrHookDef, Default
from .dmg_types import DmgTypes
from .fit import Fit
from .fleet import Fleet
from .item import Item

if TYPE_CHECKING:
    from typing import Union

    from tests.support import eve
    from tests.support.api import ApiClient


class SolarSystem(AttrDict):

    def __init__(self, *, client: ApiClient, data: dict):
        super().__init__(data=data, hooks={
            'default_incoming_dmg': AttrHookDef(
                func=lambda dp: DmgTypes(em=dp[0], thermal=dp[1], kinetic=dp[2], explosive=dp[3])),
            'fits': AttrHookDef(
                func=lambda fs: {f.id: f for f in [Fit(client=client, data=f, sol_id=self.id) for f in fs]}),
            'fleets': AttrHookDef(
                func=lambda fs: {f.id: f for f in [Fleet(client=client, data=f, sol_id=self.id) for f in fs]})})
        self._client = client

    def update(
            self, *,
            sol_info_mode: Union[ApiSolInfoMode, type[Absent]] = ApiSolInfoMode.full,
            fleet_info_mode: Union[ApiFleetInfoMode, type[Absent]] = ApiFleetInfoMode.id,
            fit_info_mode: Union[ApiFitInfoMode, type[Absent]] = ApiFitInfoMode.full,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Union[SolarSystem, None]:
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
            data: Union[eve.EveObjects, type[Absent], type[Default]] = Default,
            sol_info_mode: Union[ApiSolInfoMode, type[Absent]] = ApiSolInfoMode.full,
            fleet_info_mode: Union[ApiFleetInfoMode, type[Absent]] = ApiFleetInfoMode.id,
            fit_info_mode: Union[ApiFitInfoMode, type[Absent]] = ApiFitInfoMode.full,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
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

    def change_default_incoming_dmg(
            self, *,
            dmg_profile: Union[tuple[float, float, float, float], type[Absent]],
            sol_info_mode: Union[ApiSolInfoMode, type[Absent]] = ApiSolInfoMode.id,
            fleet_info_mode: Union[ApiFleetInfoMode, type[Absent]] = ApiFleetInfoMode.id,
            fit_info_mode: Union[ApiFitInfoMode, type[Absent]] = ApiFitInfoMode.id,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> SolarSystem:
        resp = self._client.change_sol_default_incoming_dmg_request(
            sol_id=self.id,
            dmg_profile=dmg_profile,
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
            fleet_info_mode: Union[ApiFleetInfoMode, type[Absent]] = ApiFleetInfoMode.full,
            status_code: int = 200,
            json_predicate: Union[dict, None] = None,
    ) -> Union[Fleet, None]:
        resp = self._client.get_fleet_request(
            sol_id=self.id,
            fleet_id=fleet_id,
            fleet_info_mode=fleet_info_mode).send()
        self.check()
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            fleet = Fleet(client=self._client, data=resp.json(), sol_id=self.id)
            return fleet
        return None

    def create_fleet(
            self, *,
            fleet_info_mode: Union[ApiFleetInfoMode, type[Absent]] = ApiFleetInfoMode.id,
            status_code: int = 201,
    ) -> Union[Fleet, None]:
        resp = self._client.create_fleet_request(sol_id=self.id, fleet_info_mode=fleet_info_mode).send()
        self.check()
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            fleet = Fleet(client=self._client, data=resp.json(), sol_id=self.id)
            return fleet
        return None

    # Fit methods
    def get_fit(
            self, *,
            fit_id: str,
            fit_info_mode: Union[ApiFitInfoMode, type[Absent]] = ApiFitInfoMode.full,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
            json_predicate: Union[dict, None] = None,
    ) -> Union[Fit, None]:
        resp = self._client.get_fit_request(
            sol_id=self.id,
            fit_id=fit_id,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self.check()
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            fit = Fit(client=self._client, data=resp.json(), sol_id=self.id)
            return fit
        return None

    def create_fit(
            self, *,
            fit_info_mode: Union[ApiFitInfoMode, type[Absent]] = ApiFitInfoMode.id,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = Absent,
            status_code: int = 201,
    ) -> Union[Fit, None]:
        resp = self._client.create_fit_request(
            sol_id=self.id,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self.check()
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            fit = Fit(client=self._client, data=resp.json(), sol_id=self.id)
            return fit
        return None

    # Item methods
    def get_item(
            self, *,
            item_id: str,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.full,
            status_code: int = 200,
            json_predicate: Union[dict, None] = None,
    ) -> Union[Item, None]:
        resp = self._client.get_item_request(sol_id=self.id, item_id=item_id, item_info_mode=item_info_mode).send()
        self.check()
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            return Item(client=self._client, data=resp.json(), sol_id=self.id)
        return None

    def add_sw_effect(
            self, *,
            type_id: int,
            state: Union[bool, type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self._client.add_sw_effect_request(
            sol_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode).send()
        self.check()
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self.id)
            return item
        return None

    def add_proj_effect(
            self, *,
            type_id: int,
            state: Union[bool, type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self._client.add_proj_effect_request(
            sol_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode).send()
        self.check()
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self.id)
            return item
        return None
