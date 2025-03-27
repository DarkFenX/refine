from __future__ import annotations

import typing

from tests.fw.request import Request
from tests.fw.util import conditional_insert
from .base import ApiClientBase

if typing.TYPE_CHECKING:
    from tests.fw.consts import (
        ApiEffMode,
        ApiItemInfoMode,
        ApiMinionState,
        ApiModAddMode,
        ApiModRmMode,
        ApiModuleState,
        ApiRack,
        ApiServiceState,
    )
    from tests.fw.util import Absent


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

    # Autocharge methods
    def change_autocharge_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='autocharge',
            sol_id=sol_id,
            item_id=item_id,
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
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = {
            'type': 'booster',
            'fit_id': fit_id,
            'type_id': type_id}
        conditional_insert(container=body, path=['state'], value=state)
        conditional_insert(container=body, path=['side_effects'], value=side_effects)
        params = {}
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def change_booster_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: bool | type[Absent],
            side_effects: dict[str, bool] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = {'type': 'booster'}
        conditional_insert(container=body, path=['state'], value=state)
        conditional_insert(container=body, path=['side_effects'], value=side_effects)
        params = {}
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='PATCH',
            url=f'{self._base_url}/sol/{sol_id}/item/{item_id}',
            params=params,
            json=body)

    # Charge methods
    def change_charge_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='charge',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            effect_modes=effect_modes,
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
            item_id: int,
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='character',
            sol_id=sol_id,
            item_id=item_id,
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
            mutation: int | tuple[int, dict[int, dict[str, float]]] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = {
            'type': 'drone',
            'fit_id': fit_id,
            'type_id': type_id,
            'state': state}
        conditional_insert(container=body, path=['mutation'], value=mutation)
        params = {}
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def change_drone_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: ApiMinionState | type[Absent],
            mutation: int | tuple[int, dict[int, dict[str, float]]] | dict[int, dict] | None | type[Absent],
            add_projs: list[tuple[str, float | None] | str] | type[Absent],
            change_projs: list[tuple[str, float | None]] | type[Absent],
            rm_projs: list[str] | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = {'type': 'drone', 'item_id': item_id}
        conditional_insert(container=body, path=['state'], value=state)
        conditional_insert(container=body, path=['mutation'], value=mutation)
        conditional_insert(container=body, path=['add_projs'], value=add_projs)
        conditional_insert(container=body, path=['change_projs'], value=change_projs)
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

    # Fighter methods
    def add_fighter_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: ApiMinionState,
            count: int | None | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = {
            'type': 'fighter',
            'fit_id': fit_id,
            'type_id': type_id,
            'state': state}
        conditional_insert(container=body, path=['count'], value=count)
        params = {}
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def change_fighter_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: ApiMinionState | type[Absent],
            count: int | None | type[Absent],
            add_projs: list[tuple[str, float | None] | str] | type[Absent],
            change_projs: list[tuple[str, float | None]] | type[Absent],
            rm_projs: list[str] | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = {'type': 'fighter', 'item_id': item_id}
        conditional_insert(container=body, path=['state'], value=state)
        conditional_insert(container=body, path=['count'], value=count)
        conditional_insert(container=body, path=['add_projs'], value=add_projs)
        conditional_insert(container=body, path=['change_projs'], value=change_projs)
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

    # Fit-wide effect methods
    def add_fw_effect_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__add_simple_item_request(
            cmd_name='fw_effect',
            sol_id=sol_id,
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def change_fw_effect_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='fw_effect',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode)

    # Implant methods
    def add_implant_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__add_simple_item_request(
            cmd_name='implant',
            sol_id=sol_id,
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def change_implant_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='implant',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode)

    # Module methods
    def add_mod_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            rack: ApiRack,
            state: ApiModuleState,
            mutation: int | tuple[int, dict[int, dict[str, float]]] | type[Absent],
            charge_type_id: int | type[Absent],
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
            state: ApiModuleState | type[Absent],
            mutation: int | tuple[int, dict[int, dict[str, float]]] | dict[int, dict] | None | type[Absent],
            charge: int | None | type[Absent],
            add_projs: list[tuple[str, float | None] | str] | type[Absent],
            change_projs: list[tuple[str, float | None]] | type[Absent],
            rm_projs: list[str] | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = {'type': 'module'}
        conditional_insert(container=body, path=['state'], value=state)
        conditional_insert(container=body, path=['mutation'], value=mutation)
        conditional_insert(container=body, path=['charge'], value=charge)
        conditional_insert(container=body, path=['add_projs'], value=add_projs)
        conditional_insert(container=body, path=['change_projs'], value=change_projs)
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
            item_id: int,
            state: bool | type[Absent],
            add_projs: list[str] | type[Absent],
            rm_projs: list[str] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = {'type': 'proj_effect', 'item_id': item_id}
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
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__add_simple_item_request(
            cmd_name='rig',
            sol_id=sol_id,
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def change_rig_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='rig',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode)

    # Service methods
    def add_service_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: ApiServiceState | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__add_simple_item_request(
            cmd_name='service',
            sol_id=sol_id,
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def change_service_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: ApiServiceState | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='service',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode)

    # Ship methods
    def set_ship_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__add_simple_item_request(
            cmd_name='ship',
            sol_id=sol_id,
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def change_ship_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='ship',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode)

    # Skill methods
    def add_skill_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            level: int,
            state: bool | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = {
            'type': 'skill',
            'fit_id': fit_id,
            'type_id': type_id,
            'level': level}
        conditional_insert(container=body, path=['state'], value=state)
        params = {}
        conditional_insert(container=params, path=['item'], value=item_info_mode)
        return Request(
            client=self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def change_skill_request(
            self, *,
            sol_id: str,
            item_id: str,
            level: int | type[Absent],
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = {'type': 'skill'}
        conditional_insert(container=body, path=['level'], value=level)
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

    # Subsystem methods
    def add_subsystem_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: bool | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__add_simple_item_request(
            cmd_name='subsystem',
            sol_id=sol_id,
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def change_subsystem_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='subsystem',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            effect_modes=effect_modes,
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
            item_id: int,
            state: bool | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='sw_effect',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode)

    # Auxiliary methods
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
            item_id: int,
            state: bool | str | type[Absent],
            effect_modes: dict[str, ApiEffMode] | type[Absent],
            item_info_mode: ApiItemInfoMode | type[Absent],
    ) -> Request:
        body = {'type': cmd_name}
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
