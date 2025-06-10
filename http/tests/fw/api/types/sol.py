from __future__ import annotations

import typing

from tests.fw.consts import ApiFitInfoMode, ApiFleetInfoMode, ApiItemInfoMode, ApiSolInfoMode, ApiValInfoMode
from tests.fw.util import Absent, AttrDict, AttrHookDef, Default, is_subset
from .dmg_types import DmgTypes
from .fit import Fit
from .fleet import Fleet
from .item import Item
from .validation import FitValResult, SolValResult

if typing.TYPE_CHECKING:
    from tests.fw import eve
    from tests.fw.api import ApiClient
    from tests.fw.api.aliases import DpsProfile
    from tests.fw.consts import ApiSecZone
    from tests.fw.response import Response
    from .validation import ValOptions


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

    def change(
            self, *,
            sec_zone: ApiSecZone | type[Absent] = Absent,
            default_incoming_dps: DpsProfile | type[Absent] = Absent,
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

    def validate(
            self, *,
            fit_ids: list[str] | type[Absent | Default] = Default,
            options: ValOptions | type[Absent],
            status_code: int = 200,
            flip_order: bool = False,
    ) -> SolValResult | None:
        if flip_order:
            resp_detailed = self.__validate_sol(
                fit_ids=fit_ids,
                options=options,
                val_info_mode=ApiValInfoMode.detailed,
                status_code=status_code)
            resp_simple = self.__validate_sol(
                fit_ids=fit_ids,
                options=options,
                val_info_mode=ApiValInfoMode.simple,
                status_code=status_code)
        else:
            resp_simple = self.__validate_sol(
                fit_ids=fit_ids,
                options=options,
                val_info_mode=ApiValInfoMode.simple,
                status_code=status_code)
            resp_detailed = self.__validate_sol(
                fit_ids=fit_ids,
                options=options,
                val_info_mode=ApiValInfoMode.detailed,
                status_code=status_code)
        # Ensure simple results are consistent with full results
        if resp_simple.status_code == 200 and resp_detailed.status_code == 200:
            result_simple = SolValResult(data=resp_simple.json())
            result_detailed = SolValResult(data=resp_detailed.json())
            assert result_simple.passed is result_detailed.passed
            assert is_subset(smaller=result_simple.get_raw(), larger=result_detailed.get_raw()) is True
            # If there are any fit failures, compare results with per-fit requests
            for fit_id, fit_details in result_detailed.fits.items():
                resp_fit_simple = self.__validate_fit(
                    fit_id=fit_id,
                    options=options,
                    val_info_mode=ApiValInfoMode.simple,
                    status_code=200)
                result_fit_simple = FitValResult(data=resp_fit_simple.json())
                assert result_fit_simple.passed is False
                resp_fit_detailed = self.__validate_fit(
                    fit_id=fit_id,
                    options=options,
                    val_info_mode=ApiValInfoMode.detailed,
                    status_code=200)
                result_fit_detailed = FitValResult(data=resp_fit_detailed.json())
                assert result_fit_detailed.passed is False
                assert fit_details.compare(other=result_fit_detailed.details) is True
            return result_detailed
        return None

    def __validate_sol(
            self, *,
            fit_ids: list[str] | type[Absent | Default],
            options: ValOptions | type[Absent],
            val_info_mode: ApiValInfoMode | type[Absent],
            status_code: int,
    ) -> Response:
        if fit_ids is Default:
            fit_ids = []
        resp = self._client.validate_sol_request(
            sol_id=self.id,
            fit_ids=fit_ids,
            options=options,
            val_info_mode=val_info_mode).send()
        self._client.check_sol(sol_id=self.id)
        resp.check(status_code=status_code)
        return resp

    def __validate_fit(
            self, *,
            fit_id: str,
            options: ValOptions | type[Absent],
            val_info_mode: ApiValInfoMode | type[Absent],
            status_code: int,
    ) -> Response:
        resp = self._client.validate_fit_request(
            sol_id=self.id,
            fit_id=fit_id,
            options=options,
            val_info_mode=val_info_mode).send()
        self._client.check_sol(sol_id=self.id)
        resp.check(status_code=status_code)
        return resp

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
            rah_incoming_dps: DpsProfile | type[Absent] = Absent,
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

    # Development-specific methods
    def check(self) -> None:
        self._client.check_sol(sol_id=self.id)

    def benchmark(self, command: dict, status_code: int = 200) -> None:
        resp = self._client.benchmark_sol_request(sol_id=self.id, command=command).send()
        resp.check(status_code=status_code)
