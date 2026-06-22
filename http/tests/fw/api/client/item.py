import typing

from fw.api.commands import (
    ItemAutochargeChangeCmd,
    ItemBoosterAddCmd,
    ItemBoosterChangeCmd,
    ItemCharacterChangeCmd,
    ItemCharacterSetCmd,
    ItemChargeChangeCmd,
    ItemDroneAddCmd,
    ItemDroneChangeCmd,
    ItemFighterAddCmd,
    ItemFighterChangeCmd,
    ItemFwEffectAddCmd,
    ItemFwEffectChangeCmd,
    ItemImplantAddCmd,
    ItemImplantChangeCmd,
    ItemModuleAddCmd,
    ItemModuleChangeCmd,
    ItemProjEffectAddCmd,
    ItemProjEffectChangeCmd,
    ItemRigAddCmd,
    ItemRigChangeCmd,
    ItemServiceAddCmd,
    ItemServiceChangeCmd,
    ItemShipChangeCmd,
    ItemShipSetCmd,
    ItemSkillAddCmd,
    ItemSkillChangeCmd,
    ItemStanceChangeCmd,
    ItemStanceSetCmd,
    ItemSubsystemAddCmd,
    ItemSubsystemChangeCmd,
    ItemSwEffectAddCmd,
    ItemSwEffectChangeCmd,
)
from fw.api.types import ItemStatsOptions
from fw.request import Request
from fw.util import Absent, conditional_insert
from .base import ApiClientBase

if typing.TYPE_CHECKING:
    from fw.api.aliases import MutaAdd, MutaChange
    from fw.consts import (
        ApiEffMode,
        ApiItemInfoMode,
        ApiMinionState,
        ApiModAddMode,
        ApiModRmMode,
        ApiModuleState,
        ApiNpcProp,
        ApiOptionalReload,
        ApiRack,
        ApiRearmMinion,
        ApiServiceState,
    )


class ApiClientItem(ApiClientBase):

    # Generic item methods
    def get_item_request(
            self, *,
            sol_id: str,
            item_id: str,
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        params = {}
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='GET',
            url=f'{self._base_url}/sol/{sol_id}/item/{item_id}',
            params=params)

    def remove_item_request(
            self, *,
            sol_id: str,
            item_id: str,
            mode: ApiModRmMode | type[Absent],
    ) -> Request:
        body = {}
        conditional_insert(container=body, path=['rm_mode'], value=mode)
        kwargs = {'method': 'DELETE', 'url': f'{self._base_url}/sol/{sol_id}/item/{item_id}'}
        # Intentionally send request without body when we don't need it, to test case when the
        # server receives no content-type header
        if body:
            kwargs['json'] = body
        return Request(client=self, **kwargs)

    def get_item_stats_request(
            self, *,
            sol_id: str,
            item_id: str,
            options: ItemStatsOptions | type[Absent],
    ) -> Request:
        kwargs = {
            'method': 'POST',
            'url': f'{self._base_url}/sol/{sol_id}/item/{item_id}/stats'}
        # Intentionally send request without body when we don't need it, to test case when the
        # server receives no content-type header
        if isinstance(options, ItemStatsOptions):
            kwargs['json'] = options.to_dict()
        return Request(client=self, **kwargs)

    # Autocharge methods
    def change_autocharge_request(
            self, *,
            sol_id: str,
            item_id: str,
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemAutochargeChangeCmd(
            state=state,
            effect_modes=effect_modes).serialize()
        return self.__change_item_request(
            sol_id=sol_id,
            item_id=item_id,
            body=body,
            item_info_mode=item_info_mode)

    # Booster methods
    def add_booster_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent],
            side_effects: dict[str, bool] | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemBoosterAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            side_effects=side_effects,
            effect_modes=effect_modes).serialize()
        return self.__add_item_request(
            sol_id=sol_id,
            body=body,
            item_info_mode=item_info_mode)

    def change_booster_request(
            self, *,
            sol_id: str,
            item_id: str,
            type_id: int | type[Absent],
            state: bool | type[Absent],
            side_effects: dict[str, bool] | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemBoosterChangeCmd(
            type_id=type_id,
            state=state,
            side_effects=side_effects,
            effect_modes=effect_modes).serialize()
        return self.__change_item_request(
            sol_id=sol_id,
            item_id=item_id,
            body=body,
            item_info_mode=item_info_mode)

    # Character methods
    def set_character_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemCharacterSetCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes).serialize()
        return self.__add_item_request(
            sol_id=sol_id,
            body=body,
            item_info_mode=item_info_mode)

    def change_character_request(
            self, *,
            sol_id: str,
            item_id: str,
            type_id: int | type[Absent],
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemCharacterChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=effect_modes).serialize()
        return self.__change_item_request(
            sol_id=sol_id,
            item_id=item_id,
            body=body,
            item_info_mode=item_info_mode)

    # Charge methods
    def change_charge_request(
            self, *,
            sol_id: str,
            item_id: str,
            type_id: int | type[Absent],
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemChargeChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=effect_modes).serialize()
        return self.__change_item_request(
            sol_id=sol_id,
            item_id=item_id,
            body=body,
            item_info_mode=item_info_mode)

    # Drone methods
    def add_drone_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: ApiMinionState,
            mutation: MutaAdd | type[Absent],
            npc_prop: ApiNpcProp | type[Absent],
            projs: list[str] | type[Absent],
            coordinates: tuple[float, float, float] | type[Absent],
            movement: tuple[float, float, float] | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemDroneAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            mutation=mutation,
            npc_prop=npc_prop,
            projs=projs,
            coordinates=coordinates,
            movement=movement,
            effect_modes=effect_modes).serialize()
        return self.__add_item_request(
            sol_id=sol_id,
            body=body,
            item_info_mode=item_info_mode)

    def change_drone_request(
            self, *,
            sol_id: str,
            item_id: str,
            type_id: int | type[Absent],
            state: ApiMinionState | type[Absent],
            mutation: MutaAdd | MutaChange | type[Absent] | None,
            npc_prop: ApiNpcProp | type[Absent] | None,
            add_projs: list[str] | type[Absent],
            rm_projs: list[str] | type[Absent],
            coordinates: tuple[float, float, float] | type[Absent],
            movement: tuple[float, float, float] | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemDroneChangeCmd(
            type_id=type_id,
            state=state,
            mutation=mutation,
            npc_prop=npc_prop,
            add_projs=add_projs,
            rm_projs=rm_projs,
            coordinates=coordinates,
            movement=movement,
            effect_modes=effect_modes).serialize()
        return self.__change_item_request(
            sol_id=sol_id,
            item_id=item_id,
            body=body,
            item_info_mode=item_info_mode)

    # Fighter methods
    def add_fighter_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: ApiMinionState,
            count: int | type[Absent] | None,
            abilities: dict[int, bool] | type[Absent],
            rearm_minion: ApiRearmMinion | type[Absent],
            projs: list[str] | type[Absent],
            coordinates: tuple[float, float, float] | type[Absent],
            movement: tuple[float, float, float] | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemFighterAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            count=count,
            abilities=abilities,
            rearm_minion=rearm_minion,
            projs=projs,
            coordinates=coordinates,
            movement=movement,
            effect_modes=effect_modes).serialize()
        return self.__add_item_request(
            sol_id=sol_id,
            body=body,
            item_info_mode=item_info_mode)

    def change_fighter_request(
            self, *,
            sol_id: str,
            item_id: str,
            type_id: int | type[Absent],
            state: ApiMinionState | type[Absent],
            count: int | type[Absent] | None,
            abilities: dict[int, bool] | type[Absent],
            rearm_minion: ApiRearmMinion | type[Absent] | None,
            add_projs: list[str] | type[Absent],
            rm_projs: list[str] | type[Absent],
            coordinates: tuple[float, float, float] | type[Absent],
            movement: tuple[float, float, float] | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemFighterChangeCmd(
            type_id=type_id,
            state=state,
            count=count,
            abilities=abilities,
            rearm_minion=rearm_minion,
            add_projs=add_projs,
            rm_projs=rm_projs,
            coordinates=coordinates,
            movement=movement,
            effect_modes=effect_modes).serialize()
        return self.__change_item_request(
            sol_id=sol_id,
            item_id=item_id,
            body=body,
            item_info_mode=item_info_mode)

    # Fit-wide effect methods
    def add_fw_effect_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemFwEffectAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes).serialize()
        return self.__add_item_request(
            sol_id=sol_id,
            body=body,
            item_info_mode=item_info_mode)

    def change_fw_effect_request(
            self, *,
            sol_id: str,
            item_id: str,
            type_id: int | type[Absent],
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemFwEffectChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=effect_modes).serialize()
        return self.__change_item_request(
            sol_id=sol_id,
            item_id=item_id,
            body=body,
            item_info_mode=item_info_mode)

    # Implant methods
    def add_implant_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemImplantAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes).serialize()
        return self.__add_item_request(
            sol_id=sol_id,
            body=body,
            item_info_mode=item_info_mode)

    def change_implant_request(
            self, *,
            sol_id: str,
            item_id: str,
            type_id: int | type[Absent],
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemImplantChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=effect_modes).serialize()
        return self.__change_item_request(
            sol_id=sol_id,
            item_id=item_id,
            body=body,
            item_info_mode=item_info_mode)

    # Module methods
    def add_mod_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            rack: ApiRack,
            add_mode: ApiModAddMode | dict[ApiModAddMode, int] | type[Absent],
            state: ApiModuleState,
            mutation: MutaAdd | type[Absent],
            charge_type_id: int | type[Absent],
            spool: str | type[Absent],
            optional_reload: ApiOptionalReload | type[Absent],
            projs: list[str] | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemModuleAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            rack=rack,
            add_mode=add_mode,
            state=state,
            mutation=mutation,
            charge_type_id=charge_type_id,
            spool=spool,
            optional_reload=optional_reload,
            projs=projs,
            effect_modes=effect_modes).serialize()
        return self.__add_item_request(
            sol_id=sol_id,
            body=body,
            item_info_mode=item_info_mode)

    def change_mod_request(
            self, *,
            sol_id: str,
            item_id: str,
            type_id: int | type[Absent],
            state: ApiModuleState | type[Absent],
            mutation: MutaAdd | MutaChange | type[Absent] | None,
            charge_type_id: int | type[Absent] | None,
            spool: str | type[Absent] | None,
            optional_reload: ApiOptionalReload | type[Absent] | None,
            add_projs: list[str] | type[Absent],
            rm_projs: list[str] | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemModuleChangeCmd(
            type_id=type_id,
            state=state,
            mutation=mutation,
            charge_type_id=charge_type_id,
            spool=spool,
            optional_reload=optional_reload,
            add_projs=add_projs,
            rm_projs=rm_projs,
            effect_modes=effect_modes).serialize()
        return self.__change_item_request(
            sol_id=sol_id,
            item_id=item_id,
            body=body,
            item_info_mode=item_info_mode)

    # Projected effect methods
    def add_proj_effect_request(
            self, *,
            sol_id: str,
            type_id: int,
            state: bool | type[Absent],
            projs: list[str] | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemProjEffectAddCmd(
            type_id=type_id,
            state=state,
            projs=projs,
            effect_modes=effect_modes).serialize()
        return self.__add_item_request(
            sol_id=sol_id,
            body=body,
            item_info_mode=item_info_mode)

    def change_proj_effect_request(
            self, *,
            sol_id: str,
            item_id: str,
            type_id: int | type[Absent],
            state: bool | type[Absent],
            add_projs: list[str] | type[Absent],
            rm_projs: list[str] | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemProjEffectChangeCmd(
            type_id=type_id,
            state=state,
            add_projs=add_projs,
            rm_projs=rm_projs,
            effect_modes=effect_modes).serialize()
        return self.__change_item_request(
            sol_id=sol_id,
            item_id=item_id,
            body=body,
            item_info_mode=item_info_mode)

    # Rig methods
    def add_rig_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemRigAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes).serialize()
        return self.__add_item_request(
            sol_id=sol_id,
            body=body,
            item_info_mode=item_info_mode)

    def change_rig_request(
            self, *,
            sol_id: str,
            item_id: str,
            type_id: int | type[Absent],
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemRigChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=effect_modes).serialize()
        return self.__change_item_request(
            sol_id=sol_id,
            item_id=item_id,
            body=body,
            item_info_mode=item_info_mode)

    # Service methods
    def add_service_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: ApiServiceState,
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemServiceAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes).serialize()
        return self.__add_item_request(
            sol_id=sol_id,
            body=body,
            item_info_mode=item_info_mode)

    def change_service_request(
            self, *,
            sol_id: str,
            item_id: str,
            type_id: int | type[Absent],
            state: ApiServiceState | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemServiceChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=effect_modes).serialize()
        return self.__change_item_request(
            sol_id=sol_id,
            item_id=item_id,
            body=body,
            item_info_mode=item_info_mode)

    # Ship methods
    def set_ship_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent],
            coordinates: tuple[float, float, float] | type[Absent],
            movement: tuple[float, float, float] | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemShipSetCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            coordinates=coordinates,
            movement=movement,
            effect_modes=effect_modes).serialize()
        return self.__add_item_request(
            sol_id=sol_id,
            body=body,
            item_info_mode=item_info_mode)

    def change_ship_request(
            self, *,
            sol_id: str,
            item_id: str,
            type_id: int | type[Absent],
            state: bool | type[Absent],
            coordinates: tuple[float, float, float] | type[Absent],
            movement: tuple[float, float, float] | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemShipChangeCmd(
            type_id=type_id,
            state=state,
            coordinates=coordinates,
            movement=movement,
            effect_modes=effect_modes).serialize()
        return self.__change_item_request(
            sol_id=sol_id,
            item_id=item_id,
            body=body,
            item_info_mode=item_info_mode)

    # Skill methods
    def add_skill_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            level: int,
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemSkillAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            level=level,
            state=state,
            effect_modes=effect_modes).serialize()
        return self.__add_item_request(
            sol_id=sol_id,
            body=body,
            item_info_mode=item_info_mode)

    def change_skill_request(
            self, *,
            sol_id: str,
            item_id: str,
            type_id: int | type[Absent],
            level: int | type[Absent],
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemSkillChangeCmd(
            type_id=type_id,
            level=level,
            state=state,
            effect_modes=effect_modes).serialize()
        return self.__change_item_request(
            sol_id=sol_id,
            item_id=item_id,
            body=body,
            item_info_mode=item_info_mode)

    # Stance methods
    def set_stance_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemStanceSetCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes).serialize()
        return self.__add_item_request(
            sol_id=sol_id,
            body=body,
            item_info_mode=item_info_mode)

    def change_stance_request(
            self, *,
            sol_id: str,
            item_id: str,
            type_id: int | type[Absent],
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemStanceChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=effect_modes).serialize()
        return self.__change_item_request(
            sol_id=sol_id,
            item_id=item_id,
            body=body,
            item_info_mode=item_info_mode)

    # Subsystem methods
    def add_subsystem_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemSubsystemAddCmd(
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes).serialize()
        return self.__add_item_request(
            sol_id=sol_id,
            body=body,
            item_info_mode=item_info_mode)

    def change_subsystem_request(
            self, *,
            sol_id: str,
            item_id: str,
            type_id: int | type[Absent],
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemSubsystemChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=effect_modes).serialize()
        return self.__change_item_request(
            sol_id=sol_id,
            item_id=item_id,
            body=body,
            item_info_mode=item_info_mode)

    # System-wide effect methods
    def add_sw_effect_request(
            self, *,
            sol_id: str,
            type_id: int,
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemSwEffectAddCmd(
            type_id=type_id,
            state=state,
            effect_modes=effect_modes).serialize()
        return self.__add_item_request(
            sol_id=sol_id,
            body=body,
            item_info_mode=item_info_mode)

    def change_sw_effect_request(
            self, *,
            sol_id: str,
            item_id: str,
            type_id: int | type[Absent],
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = ItemSwEffectChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=effect_modes).serialize()
        return self.__change_item_request(
            sol_id=sol_id,
            item_id=item_id,
            body=body,
            item_info_mode=item_info_mode)

    # Auxiliary methods
    def __add_item_request(
            self, *,
            sol_id: str,
            body: dict,
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        params = {}
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def __change_item_request(
            self, *,
            sol_id: str,
            item_id: str,
            body: dict,
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        params = {}
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='PATCH',
            url=f'{self._base_url}/sol/{sol_id}/item/{item_id}',
            params=params,
            json=body)
