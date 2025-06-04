from __future__ import annotations

import typing

from tests.fw.consts import (
    ApiFitInfoMode,
    ApiItemInfoMode,
    ApiMinionState,
    ApiModAddMode,
    ApiModRmMode,
    ApiModuleState,
    ApiRack,
    ApiServiceState,
    ApiValInfoMode,
)
from tests.fw.util import Absent, AttrDict, AttrHookDef, is_subset
from .dmg_types import DmgTypes
from .item import Item
from .validation import FitValResult, SolValOptions, SolValResult

if typing.TYPE_CHECKING:
    from tests.fw.api import ApiClient
    from tests.fw.api.aliases import DpsProfile, MutaAdd
    from tests.fw.response import Response
    from .validation import FitValOptions


class Fit(AttrDict):

    def __init__(self, *, client: ApiClient, data: dict, sol_id: str) -> None:
        super().__init__(data=data, hooks={
            'rah_incoming_dps': AttrHookDef(
                func=lambda dp: DmgTypes(em=dp[0], thermal=dp[1], kinetic=dp[2], explosive=dp[3]))})
        self._client = client
        self._sol_id = sol_id

    def update(
            self, *,
            fit_info_mode: ApiFitInfoMode | type[Absent] = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Fit | None:
        resp = self._client.get_fit_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def remove(self, *, status_code: int = 204) -> None:
        resp = self._client.remove_fit_request(sol_id=self._sol_id, fit_id=self.id).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)

    def validate(
            self, *,
            options: FitValOptions,
            status_code: int = 200,
            flip_order: bool = False,
    ) -> FitValResult | None:
        if flip_order:
            resp_detailed = self.__validate_fit(
                options=options,
                val_info_mode=ApiValInfoMode.detailed,
                status_code=status_code)
            resp_simple = self.__validate_fit(
                options=options,
                val_info_mode=ApiValInfoMode.simple,
                status_code=status_code)
        else:
            resp_simple = self.__validate_fit(
                options=options,
                val_info_mode=ApiValInfoMode.simple,
                status_code=status_code)
            resp_detailed = self.__validate_fit(
                options=options,
                val_info_mode=ApiValInfoMode.detailed,
                status_code=status_code)
        if resp_simple.status_code == 200 and resp_detailed.status_code == 200:
            # Ensure simple results are consistent with full results
            result_simple = FitValResult(data=resp_simple.json())
            result_detailed = FitValResult(data=resp_detailed.json())
            assert result_simple.passed is result_detailed.passed
            assert is_subset(smaller=result_simple.get_raw(), larger=result_detailed.get_raw()) is True
            # Ensure sol validation results are consistent with fit validation results
            sol_options = SolValOptions.from_fit_options(fit_options=options, fits=[self.id])
            resp_sol_detailed = self.__validate_sol(
                options=sol_options,
                val_info_mode=ApiValInfoMode.detailed,
                status_code=200)
            result_sol_detailed = SolValResult(data=resp_sol_detailed.json())
            # If fit validation passed, fit shouldn't be in results for detailed sol validation
            # results.
            if result_detailed.passed:
                assert self.id not in result_sol_detailed.fits
            # If fit validation failed, data in sol validation should match to data in fit
            # validation, and fast solar system validation should also fail
            else:
                assert result_sol_detailed.fits[self.id].compare(other=result_detailed.details) is True
                resp_sol_simple = self.__validate_sol(
                    options=sol_options,
                    val_info_mode=ApiValInfoMode.simple,
                    status_code=200)
                result_sol_simple = SolValResult(data=resp_sol_simple.json())
                assert result_sol_simple.passed is False
            return result_detailed
        return None

    def __validate_fit(
            self, *,
            options: FitValOptions,
            val_info_mode: ApiValInfoMode | type[Absent],
            status_code: int,
    ) -> Response:
        resp = self._client.validate_fit_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            options=options,
            val_info_mode=val_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        return resp

    def __validate_sol(
            self, *,
            options: SolValOptions,
            val_info_mode: ApiValInfoMode | type[Absent],
            status_code: int,
    ) -> Response:
        resp = self._client.validate_sol_request(
            sol_id=self._sol_id,
            options=options,
            val_info_mode=val_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        return resp

    def try_fit_items(
            self, *,
            type_ids: list[int],
            options: FitValOptions,
            status_code: int = 200,
    ) -> list[int] | None:
        resp = self._client.try_fit_items_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            options=options,
            type_ids=type_ids).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            return resp.json()
        return None

    def change(
            self, *,
            fleet_id: str | None | type[Absent] = Absent,
            sec_status: float | type[Absent] = Absent,
            rah_incoming_dps: DpsProfile | None | type[Absent] = Absent,
            fit_info_mode: ApiFitInfoMode | type[Absent] = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Fit | None:
        resp = self._client.change_fit_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            fleet_id=fleet_id,
            sec_status=sec_status,
            rah_incoming_dps=rah_incoming_dps,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()['fit']
            return self
        return None

    # Item methods
    def remove_item(
            self, *,
            item_id: str,
            mode: ApiModRmMode | type[Absent] = Absent,
            status_code: int = 204,
    ) -> None:
        resp = self._client.remove_item_request(sol_id=self._sol_id, item_id=item_id, mode=mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)

    def add_booster(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            side_effects: dict[str, bool] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        resp = self._client.add_booster_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            side_effects=side_effects,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
        return None

    def set_character(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        resp = self._client.set_character_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
        return None

    def remove_character(
            self, *,
            fit_info_mode: ApiFitInfoMode | type[Absent] = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Fit | None:
        resp = self._client.remove_fit_character_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()['fit']
            return self
        return None

    def add_drone(
            self, *,
            type_id: int,
            state: ApiMinionState = ApiMinionState.in_bay,
            mutation: MutaAdd | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        resp = self._client.add_drone_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            mutation=mutation,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
        return None

    def add_fighter(
            self, *,
            type_id: int,
            state: ApiMinionState = ApiMinionState.in_bay,
            count: int | None | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        resp = self._client.add_fighter_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            count=count,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
        return None

    def add_fw_effect(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        resp = self._client.add_fw_effect_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
        return None

    def add_implant(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        resp = self._client.add_implant_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
        return None

    def add_module(
            self, *,
            type_id: int,
            rack: ApiRack = ApiRack.high,
            state: ApiModuleState = ApiModuleState.offline,
            mutation: MutaAdd | type[Absent] = Absent,
            charge_type_id: int | type[Absent] = Absent,
            mode: ApiModAddMode | dict[ApiModAddMode, int] | type[Absent] = ApiModAddMode.equip,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
            text_predicate: str | None = None,
    ) -> Item | None:
        resp = self._client.add_mod_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            rack=rack,
            type_id=type_id,
            state=state,
            mutation=mutation,
            charge_type_id=charge_type_id,
            mode=mode,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code, text_predicate=text_predicate)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
        return None

    def add_rig(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        resp = self._client.add_rig_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
        return None

    def add_service(
            self, *,
            type_id: int,
            state: ApiServiceState = ApiServiceState.offline,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        resp = self._client.add_service_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
        return None

    def set_ship(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        resp = self._client.set_ship_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
        return None

    def remove_ship(
            self, *,
            fit_info_mode: ApiFitInfoMode | type[Absent] = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Fit | None:
        resp = self._client.remove_fit_ship_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()['fit']
            return self
        return None

    def add_skill(
            self, *,
            type_id: int,
            level: int,
            state: bool | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
            json_predicate: dict | None = None,
    ) -> Item | None:
        resp = self._client.add_skill_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            level=level,
            state=state,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
        return None

    def set_stance(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        resp = self._client.set_stance_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
        return None

    def remove_stance(
            self, *,
            fit_info_mode: ApiFitInfoMode | type[Absent] = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Fit | None:
        resp = self._client.remove_fit_stance_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()['fit']
            return self
        return None

    def add_subsystem(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        resp = self._client.add_subsystem_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
        return None
