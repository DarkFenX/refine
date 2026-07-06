import typing

from fw.api.commands import (
    FitCharacterUnsetCmd,
    FitFitChangeCmd,
    FitShipUnsetCmd,
    FitStanceUnsetCmd,
    ItemBoosterAddCmd,
    ItemCharacterSetCmd,
    ItemDroneAddCmd,
    ItemFighterAddCmd,
    ItemFwEffectAddCmd,
    ItemImplantAddCmd,
    ItemModuleAddCmd,
    ItemRigAddCmd,
    ItemServiceAddCmd,
    ItemShipSetCmd,
    ItemSkillAddCmd,
    ItemStanceSetCmd,
    ItemSubsystemAddCmd,
)
from fw.api.types.cmd_ctx import FitCmdCtx
from fw.api.types.dmg_types import DmgTypes
from fw.api.types.helpers import process_effect_map_request, process_muta_add_request
from fw.api.types.item import Item
from fw.api.types.stats import FitStats
from fw.api.types.validation import FitValResult, SolValResult
from fw.consts import (
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
from fw.util import Absent, AttrDict, AttrHookDef, is_subset

if typing.TYPE_CHECKING:
    from fw.api import ApiClient
    from fw.api.aliases import DpsProfile, MutaAdd, ReqHook
    from fw.api.types.stats import FitStatsOptions
    from fw.api.types.validation import ValOptions
    from fw.consts import ApiEffMode, ApiNpcProp, ApiOptionalReload, ApiRearmMinion
    from fw.response import Response


class Fit(AttrDict):

    def __init__(self, *, client: ApiClient, data: dict, sol_id: str) -> None:
        super().__init__(data=data, hooks={
            'rah_incoming_dps': AttrHookDef(
                func=lambda dp: DmgTypes(em=dp[0], thermal=dp[1], kinetic=dp[2], explosive=dp[3]))})
        self._client = client
        self._sol_id = sol_id

    def commands(
            self, *,
            fit_info_mode: ApiFitInfoMode | type[Absent] = ApiFitInfoMode.id,
            item_info_mode: ApiItemInfoMode | type[Absent] = Absent,
            hook_req: ReqHook | None = None,
            status_code: int = 200,
            json_predicate: dict | None = None,
    ) -> FitCmdCtx:
        return FitCmdCtx(
            client=self._client,
            fit=self,
            sol_id=self._sol_id,
            fit_id=self.id,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode,
            hook_req=hook_req,
            status_code=status_code,
            json_predicate=json_predicate)

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

    def get_stats(
            self, *,
            options: FitStatsOptions | type[Absent],
            status_code: int = 200,
    ) -> FitStats | None:
        resp = self._client.get_fit_stats_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            options=options).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        return FitStats(data=resp.json())

    def validate(
            self, *,
            options: ValOptions | type[Absent],
            status_code: int = 200,
    ) -> FitValResult | None:
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
            resp_sol_detailed = self.__validate_sol(
                fit_ids=[self.id],
                options=options,
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
                    fit_ids=[self.id],
                    options=options,
                    val_info_mode=ApiValInfoMode.simple,
                    status_code=200)
                result_sol_simple = SolValResult(data=resp_sol_simple.json())
                assert result_sol_simple.passed is False
            return result_detailed
        return None

    def __validate_fit(
            self, *,
            options: ValOptions | type[Absent],
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
            fit_ids: list[str],
            options: ValOptions | type[Absent],
            val_info_mode: ApiValInfoMode | type[Absent],
            status_code: int,
    ) -> Response:
        resp = self._client.validate_sol_request(
            sol_id=self._sol_id,
            fit_ids=fit_ids,
            options=options,
            val_info_mode=val_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        return resp

    def try_fit_items(
            self, *,
            type_ids: list[int],
            options: ValOptions | type[Absent],
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
            fleet_id: str | type[Absent] | None = Absent,
            sec_status: float | type[Absent] = Absent,
            rah_incoming_dps: DpsProfile | type[Absent] | None = Absent,
            fit_info_mode: ApiFitInfoMode | type[Absent] = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Fit | None:
        command = FitFitChangeCmd(
            fleet_id=fleet_id,
            sec_status=sec_status,
            rah_incoming_dps=rah_incoming_dps)
        resp = self._client.fit_commands_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            commands=[command],
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
            rm_mode: ApiModRmMode | type[Absent] = Absent,
            status_code: int = 204,
    ) -> None:
        resp = self._client.remove_item_request(sol_id=self._sol_id, item_id=item_id, rm_mode=rm_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)

    def add_booster(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            side_effects: dict[int | str, bool] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        command = ItemBoosterAddCmd(
            fit_id=self.id,
            type_id=type_id,
            state=state,
            side_effects=process_effect_map_request(effect_map=side_effects),
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_add_request(
            sol_id=self._sol_id,
            command=command,
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
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        command = ItemCharacterSetCmd(
            fit_id=self.id,
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_add_request(
            sol_id=self._sol_id,
            command=command,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
        return None

    def unset_character(
            self, *,
            fit_info_mode: ApiFitInfoMode | type[Absent] = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Fit | None:
        command = FitCharacterUnsetCmd()
        resp = self._client.fit_commands_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            commands=[command],
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
            npc_prop: ApiNpcProp | type[Absent] = Absent,
            proj_item_ids: list[str] | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        command = ItemDroneAddCmd(
            fit_id=self.id,
            type_id=type_id,
            state=state,
            mutation=process_muta_add_request(mutation=mutation),
            npc_prop=npc_prop,
            proj_item_ids=proj_item_ids,
            coordinates=coordinates,
            movement=movement,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_add_request(
            sol_id=self._sol_id,
            command=command,
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
            count: int | type[Absent] = Absent,
            abilities: dict[int, bool] | type[Absent] = Absent,
            rearm_minion: ApiRearmMinion | type[Absent] = Absent,
            proj_item_ids: list[str] | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        command = ItemFighterAddCmd(
            fit_id=self.id,
            type_id=type_id,
            state=state,
            count=count,
            abilities=abilities,
            rearm_minion=rearm_minion,
            proj_item_ids=proj_item_ids,
            coordinates=coordinates,
            movement=movement,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_add_request(
            sol_id=self._sol_id,
            command=command,
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
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        command = ItemFwEffectAddCmd(
            fit_id=self.id,
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_add_request(
            sol_id=self._sol_id,
            command=command,
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
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        command = ItemImplantAddCmd(
            fit_id=self.id,
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_add_request(
            sol_id=self._sol_id,
            command=command,
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
            add_mode: ApiModAddMode | dict[ApiModAddMode, int] | type[Absent] = ApiModAddMode.equip,
            state: ApiModuleState = ApiModuleState.offline,
            mutation: MutaAdd | type[Absent] = Absent,
            charge_type_id: int | type[Absent] = Absent,
            spool: str | type[Absent] = Absent,
            optional_reload: ApiOptionalReload | type[Absent] = Absent,
            proj_item_ids: list[str] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
            json_predicate: dict | None = None,
    ) -> Item | None:
        command = ItemModuleAddCmd(
            fit_id=self.id,
            type_id=type_id,
            rack=rack,
            add_mode=add_mode,
            state=state,
            mutation=process_muta_add_request(mutation=mutation),
            charge_type_id=charge_type_id,
            spool=spool,
            optional_reload=optional_reload,
            proj_item_ids=proj_item_ids,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_add_request(
            sol_id=self._sol_id,
            command=command,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
        return None

    def add_rig(
            self, *,
            type_id: int,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        command = ItemRigAddCmd(
            fit_id=self.id,
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_add_request(
            sol_id=self._sol_id,
            command=command,
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
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        command = ItemServiceAddCmd(
            fit_id=self.id,
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_add_request(
            sol_id=self._sol_id,
            command=command,
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
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        command = ItemShipSetCmd(
            fit_id=self.id,
            type_id=type_id,
            state=state,
            coordinates=coordinates,
            movement=movement,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_add_request(
            sol_id=self._sol_id,
            command=command,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
        return None

    def unset_ship(
            self, *,
            fit_info_mode: ApiFitInfoMode | type[Absent] = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Fit | None:
        command = FitShipUnsetCmd()
        resp = self._client.fit_commands_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            commands=[command],
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
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
            json_predicate: dict | None = None,
    ) -> Item | None:
        command = ItemSkillAddCmd(
            fit_id=self.id,
            type_id=type_id,
            level=level,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_add_request(
            sol_id=self._sol_id,
            command=command,
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
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        command = ItemStanceSetCmd(
            fit_id=self.id,
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_add_request(
            sol_id=self._sol_id,
            command=command,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
        return None

    def unset_stance(
            self, *,
            fit_info_mode: ApiFitInfoMode | type[Absent] = ApiFitInfoMode.full,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Fit | None:
        command = FitStanceUnsetCmd()
        resp = self._client.fit_commands_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            commands=[command],
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
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Item | None:
        command = ItemSubsystemAddCmd(
            fit_id=self.id,
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_add_request(
            sol_id=self._sol_id,
            command=command,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            return Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
        return None
