import typing

from fw.api.commands import ItemProjEffectAddCmd, ItemSwEffectAddCmd, SolSolChangeCmd
from fw.api.types.cmd_ctx.sol_ctx import SolCmdCtx
from fw.api.types.dmg_types import DmgTypes
from fw.api.types.fit import Fit
from fw.api.types.fleet import Fleet
from fw.api.types.helpers import process_effect_map_request
from fw.api.types.item import Item
from fw.api.types.validation import FitValResult, SolValResult
from fw.consts import ApiFitInfoMode, ApiFleetInfoMode, ApiItemInfoMode, ApiSolInfoMode, ApiValInfoMode
from fw.util import Absent, AttrDict, AttrHookDef, Default, is_subset

if typing.TYPE_CHECKING:
    from fw import eve
    from fw.api import ApiClient
    from fw.api.aliases import DpsProfile, ReqHook
    from fw.api.types.validation import ValOptions
    from fw.consts import ApiEffMode, ApiNpcProp, ApiOptionalReload, ApiRearmMinion, ApiSecZone
    from fw.response import Response


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

    def commands(
            self, *,
            sol_info_mode: ApiSolInfoMode | type[Absent] = ApiSolInfoMode.id,
            fleet_info_mode: ApiFleetInfoMode | type[Absent] = Absent,
            fit_info_mode: ApiFitInfoMode | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = Absent,
            hook_req: ReqHook | None = None,
            status_code: int = 200,
            json_predicate: dict | None = None,
    ) -> SolCmdCtx:
        return SolCmdCtx(
            client=self._client,
            sol=self,
            sol_id=self.id,
            sol_info_mode=sol_info_mode,
            fleet_info_mode=fleet_info_mode,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode,
            hook_req=hook_req,
            status_code=status_code,
            json_predicate=json_predicate)

    def update(
            self, *,
            sol_info_mode: ApiSolInfoMode | type[Absent] = ApiSolInfoMode.full,
            fleet_info_mode: ApiFleetInfoMode | type[Absent] = ApiFleetInfoMode.id,
            fit_info_mode: ApiFitInfoMode | type[Absent] = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
            json_predicate: dict | None = None,
    ) -> SolarSystem | None:
        resp = self._client.get_sol_request(
            sol_id=self.id,
            sol_info_mode=sol_info_mode,
            fleet_info_mode=fleet_info_mode,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self.id)
        resp.check(status_code=status_code, json_predicate=json_predicate)
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
            json_predicate: dict | None = None,
    ) -> SolarSystem:
        resp = self._client.change_sol_src_request(
            sol_id=self.id,
            data=data,
            sol_info_mode=sol_info_mode,
            fleet_info_mode=fleet_info_mode,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self.id)
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            self._data = resp.json()
        return self

    def remove(
            self, *,
            status_code: int = 204,
            json_predicate: dict | None = None,
    ) -> None:
        resp = self._client.remove_sol_request(sol_id=self.id).send()
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 204:
            self._client.created_sols.remove(self)

    def change(
            self, *,
            sec_zone: ApiSecZone | type[Absent] = Absent,
            default_incoming_dps: DpsProfile | type[Absent] = Absent,
            default_spool: str | type[Absent] = Absent,
            default_npc_prop: ApiNpcProp | type[Absent] = Absent,
            default_optional_reloads: ApiOptionalReload | type[Absent] = Absent,
            default_rearm_minions: ApiRearmMinion | type[Absent] = Absent,
            sol_info_mode: ApiSolInfoMode | type[Absent] = ApiSolInfoMode.id,
            fleet_info_mode: ApiFleetInfoMode | type[Absent] = ApiFleetInfoMode.id,
            fit_info_mode: ApiFitInfoMode | type[Absent] = ApiFitInfoMode.id,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
            json_predicate: dict | None = None,
    ) -> SolarSystem:
        command = SolSolChangeCmd(
            sec_zone=sec_zone,
            default_incoming_dps=default_incoming_dps,
            default_spool=default_spool,
            default_npc_prop=default_npc_prop,
            default_optional_reloads=default_optional_reloads,
            default_rearm_minions=default_rearm_minions)
        resp = self._client.sol_commands_request(
            sol_id=self.id,
            commands=[command],
            sol_info_mode=sol_info_mode,
            fleet_info_mode=fleet_info_mode,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self.check()
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            self._data = resp.json()['solar_system']
        return self

    def validate(
            self, *,
            fit_ids: list[str] | type[Absent | Default] = Default,
            options: ValOptions | type[Absent],
            status_code: int = 200,
            json_predicate: dict | None = None,
    ) -> SolValResult | None:
        resp_simple = self.__validate_sol(
            fit_ids=fit_ids,
            options=options,
            val_info_mode=ApiValInfoMode.simple,
            status_code=status_code,
            json_predicate=json_predicate)
        resp_detailed = self.__validate_sol(
            fit_ids=fit_ids,
            options=options,
            val_info_mode=ApiValInfoMode.detailed,
            status_code=status_code,
            json_predicate=json_predicate)
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
                    status_code=200,
                    json_predicate=None)
                result_fit_simple = FitValResult(data=resp_fit_simple.json())
                assert result_fit_simple.passed is False
                resp_fit_detailed = self.__validate_fit(
                    fit_id=fit_id,
                    options=options,
                    val_info_mode=ApiValInfoMode.detailed,
                    status_code=200,
                    json_predicate=None)
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
            json_predicate: dict | None,
    ) -> Response:
        if fit_ids is Default:
            fit_ids = []
        resp = self._client.validate_sol_request(
            sol_id=self.id,
            fit_ids=fit_ids,
            options=options,
            val_info_mode=val_info_mode).send()
        self._client.check_sol(sol_id=self.id)
        resp.check(status_code=status_code, json_predicate=json_predicate)
        return resp

    def __validate_fit(
            self, *,
            fit_id: str,
            options: ValOptions | type[Absent],
            val_info_mode: ApiValInfoMode | type[Absent],
            status_code: int,
            json_predicate: dict | None,
    ) -> Response:
        resp = self._client.validate_fit_request(
            sol_id=self.id,
            fit_id=fit_id,
            options=options,
            val_info_mode=val_info_mode).send()
        self._client.check_sol(sol_id=self.id)
        resp.check(status_code=status_code, json_predicate=json_predicate)
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
            fit_ids: list[str] | type[Absent] = Absent,
            fleet_info_mode: ApiFleetInfoMode | type[Absent] = ApiFleetInfoMode.id,
            status_code: int = 201,
            json_predicate: dict | None = None,
    ) -> Fleet | None:
        resp = self._client.create_fleet_request(
            sol_id=self.id,
            fit_ids=fit_ids,
            fleet_info_mode=fleet_info_mode).send()
        self.check()
        resp.check(status_code=status_code, json_predicate=json_predicate)
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
            fleet_id: str | type[Absent] = Absent,
            sec_status: float | type[Absent] = Absent,
            rah_incoming_dps: DpsProfile | type[Absent] = Absent,
            fit_info_mode: ApiFitInfoMode | type[Absent] = ApiFitInfoMode.id,
            item_info_mode: ApiItemInfoMode | type[Absent] = Absent,
            status_code: int = 201,
            json_predicate: dict | None = None,
    ) -> Fit | None:
        resp = self._client.create_fit_request(
            sol_id=self.id,
            fleet_id=fleet_id,
            sec_status=sec_status,
            rah_incoming_dps=rah_incoming_dps,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self.check()
        resp.check(status_code=status_code, json_predicate=json_predicate)
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
            proj_item_ids: list[str] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
            json_predicate: dict | None = None,
    ) -> Item | None:
        command = ItemProjEffectAddCmd(
            type_id=type_id,
            state=state,
            proj_item_ids=proj_item_ids,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_add_request(
            sol_id=self.id,
            command=command,
            item_info_mode=item_info_mode).send()
        self.check()
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self.id)
        return None

    def add_sw_effect(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
            json_predicate: dict | None = None,
    ) -> Item | None:
        command = ItemSwEffectAddCmd(
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_add_request(
            sol_id=self.id,
            command=command,
            item_info_mode=item_info_mode).send()
        self.check()
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self.id)
        return None

    # Development-specific methods
    def check(self) -> None:
        self._client.check_sol(sol_id=self.id)

    def benchmark(self, *, command: dict, status_code: int = 200) -> None:
        resp = self._client.benchmark_sol_request(sol_id=self.id, command=command).send()
        resp.check(status_code=status_code)
