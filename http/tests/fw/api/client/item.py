import typing

from fw.api.commands import (
    ItemBoosterAddCmd,
    ItemBoosterChangeCmd,
    ItemDroneAddCmd,
    ItemDroneChangeCmd,
    ItemFighterAddCmd,
    ItemFighterChangeCmd,
    ItemFwEffectAddCmd,
    ItemFwEffectChangeCmd,
    ItemImplantAddCmd,
    ItemImplantChangeCmd,
    ItemRigAddCmd,
    ItemRigChangeCmd,
    ItemServiceAddCmd,
    ItemServiceChangeCmd,
    ItemSkillAddCmd,
    ItemSkillChangeCmd,
    ItemSubsystemAddCmd,
    ItemSubsystemChangeCmd,
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
        return self.__change_simple_item_request(
            cmd_name='autocharge',
            sol_id=sol_id,
            item_id=item_id,
            type_id=Absent,
            state=state,
            effect_modes=effect_modes,
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
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__add_simple_item_request(
            cmd_name='character',
            sol_id=sol_id,
            fit_id=fit_id,
            type_id=type_id,
            state=state,
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
        return self.__change_simple_item_request(
            cmd_name='character',
            sol_id=sol_id,
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes,
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
        return self.__change_simple_item_request(
            cmd_name='charge',
            sol_id=sol_id,
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes,
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
            state: ApiModuleState,
            mutation: MutaAdd | type[Absent],
            charge_type_id: int | type[Absent],
            spool: str | type[Absent],
            optional_reload: ApiOptionalReload | type[Absent],
            mode: ApiModAddMode | dict[ApiModAddMode, int] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = {
            'type': 'module',
            'fit_id': fit_id,
            'rack': rack,
            'type_id': type_id,
            'state': state}
        conditional_insert(container=body, path=['mutation'], value=mutation)
        conditional_insert(container=body, path=['charge_type_id'], value=charge_type_id)
        conditional_insert(container=body, path=['spool'], value=spool)
        conditional_insert(container=body, path=['optional_reload'], value=optional_reload)
        conditional_insert(container=body, path=['add_mode'], value=mode)
        params = {}
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

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
        body = {'type': 'module'}
        conditional_insert(container=body, path=['type_id'], value=type_id)
        conditional_insert(container=body, path=['state'], value=state)
        conditional_insert(container=body, path=['mutation'], value=mutation)
        conditional_insert(container=body, path=['charge_type_id'], value=charge_type_id)
        conditional_insert(container=body, path=['spool'], value=spool)
        conditional_insert(container=body, path=['optional_reload'], value=optional_reload)
        conditional_insert(container=body, path=['add_projs'], value=add_projs)
        conditional_insert(container=body, path=['rm_projs'], value=rm_projs)
        conditional_insert(container=body, path=['effect_modes'], value=effect_modes)
        params = {}
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='PATCH',
            url=f'{self._base_url}/sol/{sol_id}/item/{item_id}',
            params=params,
            json=body)

    # Projected effect methods
    def add_proj_effect_request(
            self, *,
            sol_id: str,
            type_id: int,
            state: bool | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = {'type': 'proj_effect', 'type_id': type_id}
        conditional_insert(container=body, path=['state'], value=state)
        params = {}
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def change_proj_effect_request(
            self, *,
            sol_id: str,
            item_id: str,
            type_id: int | type[Absent],
            state: bool | type[Absent],
            add_projs: list[str] | type[Absent],
            rm_projs: list[str] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = {'type': 'proj_effect'}
        conditional_insert(container=body, path=['type_id'], value=type_id)
        conditional_insert(container=body, path=['state'], value=state)
        conditional_insert(container=body, path=['add_projs'], value=add_projs)
        conditional_insert(container=body, path=['rm_projs'], value=rm_projs)
        params = {}
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='PATCH',
            url=f'{self._base_url}/sol/{sol_id}/item/{item_id}',
            params=params,
            json=body)

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
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = {
            'type': 'ship',
            'fit_id': fit_id,
            'type_id': type_id}
        conditional_insert(container=body, path=['state'], value=state)
        conditional_insert(container=body, path=['coordinates'], value=coordinates)
        conditional_insert(container=body, path=['movement'], value=movement)
        params = {}
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

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
        body = {'type': 'ship'}
        conditional_insert(container=body, path=['type_id'], value=type_id)
        conditional_insert(container=body, path=['state'], value=state)
        conditional_insert(container=body, path=['coordinates'], value=coordinates)
        conditional_insert(container=body, path=['movement'], value=movement)
        conditional_insert(container=body, path=['effect_modes'], value=effect_modes)
        params = {}
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='PATCH',
            url=f'{self._base_url}/sol/{sol_id}/item/{item_id}',
            params=params,
            json=body)

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
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__add_simple_item_request(
            cmd_name='stance',
            sol_id=sol_id,
            fit_id=fit_id,
            type_id=type_id,
            state=state,
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
        return self.__change_simple_item_request(
            cmd_name='stance',
            sol_id=sol_id,
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes,
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
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = {'type': 'sw_effect', 'type_id': type_id}
        conditional_insert(container=body, path=['state'], value=state)
        params = {}
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def change_sw_effect_request(
            self, *,
            sol_id: str,
            item_id: str,
            type_id: int | type[Absent],
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='sw_effect',
            sol_id=sol_id,
            item_id=item_id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode)

    # Auxiliary methods
    # TODO: remove first 2 ones after command refactor is done
    def __add_simple_item_request(
            self, *,
            cmd_name: str,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: bool | str | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = {
            'type': cmd_name,
            'fit_id': fit_id,
            'type_id': type_id}
        conditional_insert(container=body, path=['state'], value=state)
        params = {}
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def __change_simple_item_request(
            self, *,
            cmd_name: str,
            sol_id: str,
            item_id: str,
            type_id: int | type[Absent],
            state: bool | str | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = {'type': cmd_name}
        conditional_insert(container=body, path=['type_id'], value=type_id)
        conditional_insert(container=body, path=['state'], value=state)
        conditional_insert(container=body, path=['effect_modes'], value=effect_modes)
        params = {}
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='PATCH',
            url=f'{self._base_url}/sol/{sol_id}/item/{item_id}',
            params=params,
            json=body)

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
