import typing

from fw.consts import ApiFleetInfoMode
from fw.util import AttrDict, AttrHookDef
from .stats import FleetStats

if typing.TYPE_CHECKING:
    from fw.api import ApiClient
    from fw.util import Absent
    from .stats import FleetStatsOptions


class Fleet(AttrDict):

    def __init__(self, *, client: ApiClient, data: dict, sol_id: str) -> None:
        super().__init__(data=data, hooks={'fits': AttrHookDef(func=lambda fits: fits)})
        self._client = client
        self._sol_id = sol_id

    def update(
            self, *,
            fleet_info_mode: ApiFleetInfoMode | type[Absent] = ApiFleetInfoMode.full,
            status_code: int = 200,
    ) -> Fleet | None:
        resp = self._client.get_fleet_request(
            sol_id=self._sol_id,
            fleet_id=self.id,
            fleet_info_mode=fleet_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def change(
            self, *,
            add_fits: list[str] = (),
            remove_fits: list[str] = (),
            fleet_info_mode: ApiFleetInfoMode | type[Absent] = ApiFleetInfoMode.full,
            status_code: int = 200,
    ) -> Fleet | None:
        resp = self._client.change_fleet_request(
            sol_id=self._sol_id,
            fleet_id=self.id,
            add_fits=add_fits,
            remove_fits=remove_fits,
            fleet_info_mode=fleet_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def remove(self, *, status_code: int = 204) -> None:
        resp = self._client.remove_fleet_request(sol_id=self._sol_id, fleet_id=self.id).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)

    def get_stats(
            self, *,
            options: FleetStatsOptions | type[Absent],
            status_code: int = 200,
    ) -> FleetStats | None:
        resp = self._client.get_fleet_stats_request(
            sol_id=self._sol_id,
            fleet_id=self.id,
            options=options).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        return FleetStats(data=resp.json())
