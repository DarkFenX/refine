import typing

from fw.api.types.fit import Fit
from fw.consts import ApiFitInfoMode, ApiFleetInfoMode, ApiItemInfoMode
from fw.util import Absent, AttrDict, AttrHookDef
from .stats import FleetStats

if typing.TYPE_CHECKING:
    from fw.api import ApiClient
    from .stats import FleetStatsOptions


class Fleet(AttrDict):

    def __init__(self, *, client: ApiClient, data: dict, sol_id: str) -> None:
        super().__init__(data=data, hooks={
            'fits': AttrHookDef(
                func=lambda fs: {f.id: f for f in [Fit(client=client, data=f, sol_id=self.id) for f in fs]})})
        self._client = client
        self._sol_id = sol_id

    def update(
            self, *,
            fleet_info_mode: ApiFleetInfoMode | type[Absent] = ApiFleetInfoMode.full,
            fit_info_mode: ApiFitInfoMode | type[Absent] = ApiFitInfoMode.id,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
            json_predicate: dict | None = None,
    ) -> Fleet | None:
        resp = self._client.get_fleet_request(
            sol_id=self._sol_id,
            fleet_id=self.id,
            fleet_info_mode=fleet_info_mode,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def change(
            self, *,
            add_fit_ids: list[str] | type[Absent] = Absent,
            rm_fit_ids: list[str] | type[Absent] = Absent,
            fleet_info_mode: ApiFleetInfoMode | type[Absent] = ApiFleetInfoMode.full,
            status_code: int = 200,
            json_predicate: dict | None = None,
    ) -> Fleet | None:
        resp = self._client.change_fleet_request(
            sol_id=self._sol_id,
            fleet_id=self.id,
            add_fit_ids=add_fit_ids,
            rm_fit_ids=rm_fit_ids,
            fleet_info_mode=fleet_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def remove(
            self, *,
            status_code: int = 204,
            json_predicate: dict | None = None,
    ) -> None:
        resp = self._client.remove_fleet_request(sol_id=self._sol_id, fleet_id=self.id).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code, json_predicate=json_predicate)

    def get_stats(
            self, *,
            options: FleetStatsOptions | type[Absent],
            status_code: int = 200,
            json_predicate: dict | None = None,
    ) -> FleetStats | None:
        resp = self._client.get_fleet_stats_request(
            sol_id=self._sol_id,
            fleet_id=self.id,
            options=options).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            return FleetStats(data=resp.json()['fleet'])
        return None
