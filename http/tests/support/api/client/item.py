from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.request import Request
from tests.support.util import conditional_insert
from .base import ApiClientBase

if TYPE_CHECKING:
    from collections.abc import Iterable
    from typing import Tuple, Type, Union

    from tests.support.consts import ApiItemInfoMode, ApiEffMode, ApiModAddMode, ApiRack, ApiState
    from tests.support.util import Absent


class ApiClientItem(ApiClientBase):

    # Generic item methods
    def get_item_request(
            self, *,
            sol_id: str,
            item_id: str,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='GET',
            url=f'{self._base_url}/sol/{sol_id}/item/{item_id}',
            params=params)

    def remove_item_request(
            self, *,
            sol_id: str,
            item_id: str,
    ) -> Request:
        return Request(
            self,
            method='DELETE',
            url=f'{self._base_url}/sol/{sol_id}/item/{item_id}')

    # Character methods
    def set_char_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__add_simple_item_request(
            cmd_name='character',
            sol_id=sol_id,
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def change_char_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='character',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            item_info_mode=item_info_mode)

    # Skill methods
    def add_skill_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            level: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {
            'type': 'skill',
            'fit_id': fit_id,
            'type_id': type_id,
            'level': level}
        conditional_insert(container=body, key='state', value=state)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def change_skill_request(
            self, *,
            sol_id: str,
            item_id: str,
            level: Union[int, Type[Absent]],
            state: Union[bool, Type[Absent]],
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {'type': 'skill'}
        conditional_insert(container=body, key='level', value=level)
        conditional_insert(container=body, key='state', value=state)
        conditional_insert(container=body, key='effect_modes', value=effect_modes)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='PATCH',
            url=f'{self._base_url}/sol/{sol_id}/item/{item_id}',
            params=params,
            json=body)

    # Implant methods
    def add_implant_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
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
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='implant',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            item_info_mode=item_info_mode)

    # Booster methods
    def add_booster_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]],
            side_effects: Union[dict[int, bool], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {
            'type': 'booster',
            'fit_id': fit_id,
            'type_id': type_id}
        conditional_insert(container=body, key='state', value=state)
        conditional_insert(container=body, key='side_effects', value=side_effects)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def change_booster_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]],
            side_effects: Union[dict[int, bool], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {'type': 'booster'}
        conditional_insert(container=body, key='state', value=state)
        conditional_insert(container=body, key='side_effects', value=side_effects)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='PATCH',
            url=f'{self._base_url}/sol/{sol_id}/item/{item_id}',
            params=params,
            json=body)

    # Ship methods
    def set_ship_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
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
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='ship',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            item_info_mode=item_info_mode)

    # Stance methods
    def set_stance_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
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
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__add_simple_item_request(
            cmd_name='subsystem',
            sol_id=sol_id,
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    # Module methods
    def add_mod_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            rack: ApiRack,
            state: ApiState,
            mutation: Union[int, Tuple[int, dict[int, dict]], Type[Absent]],
            charge_type_id: Union[int, Type[Absent]],
            mode: ApiModAddMode,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {
            'type': 'module',
            'fit_id': fit_id,
            'rack': rack,
            'add_mode': mode,
            'type_id': type_id,
            'state': state}
        conditional_insert(container=body, key='mutation', value=mutation)
        conditional_insert(container=body, key='charge_type_id', value=charge_type_id)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def change_mod_request(
            self, *,
            sol_id: str,
            item_id: str,
            state: Union[ApiState, Type[Absent]],
            mutation: Union[int, Tuple[int, dict[int, dict]], dict[int, dict], None, Type[Absent]],
            charge: Union[int, None, Type[Absent]],
            add_projs: Union[Iterable[(str, Union[float, None])], Type[Absent]],
            change_projs: Union[Iterable[(str, Union[float, None])], Type[Absent]],
            rm_projs: Union[Iterable[str], Type[Absent]],
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {'type': 'module'}
        conditional_insert(container=body, key='state', value=state)
        conditional_insert(container=body, key='mutation', value=mutation)
        conditional_insert(container=body, key='charge', value=charge)
        conditional_insert(container=body, key='add_projs', value=add_projs)
        conditional_insert(container=body, key='change_projs', value=change_projs)
        conditional_insert(container=body, key='rm_projs', value=rm_projs)
        conditional_insert(container=body, key='effect_modes', value=effect_modes)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
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
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
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
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='rig',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            item_info_mode=item_info_mode)

    # Drone methods
    def add_drone_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: ApiState,
            mutation: Union[int, Tuple[int, dict[int, dict[str, float]]], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {
            'type': 'drone',
            'fit_id': fit_id,
            'type_id': type_id}
        conditional_insert(container=body, key='state', value=state)
        conditional_insert(container=body, key='mutation', value=mutation)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def change_drone_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: Union[ApiState, Type[Absent]],
            mutation: Union[int, Tuple[int, dict[int, dict]], dict[int, dict], None, Type[Absent]],
            add_projs: Union[Iterable[(str, Union[float, None])], Type[Absent]],
            change_projs: Union[Iterable[(str, Union[float, None])], Type[Absent]],
            rm_projs: Union[Iterable[str], Type[Absent]],
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {'type': 'drone', 'item_id': item_id}
        conditional_insert(container=body, key='state', value=state)
        conditional_insert(container=body, key='mutation', value=mutation)
        conditional_insert(container=body, key='add_projs', value=add_projs)
        conditional_insert(container=body, key='change_projs', value=change_projs)
        conditional_insert(container=body, key='rm_projs', value=rm_projs)
        conditional_insert(container=body, key='effect_modes', value=effect_modes)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
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
            state: ApiState,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__add_simple_item_request(
            cmd_name='fighter',
            sol_id=sol_id,
            fit_id=fit_id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def change_fighter_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: Union[ApiState, Type[Absent]],
            add_projs: Union[Iterable[(str, Union[float, None])], Type[Absent]],
            change_projs: Union[Iterable[(str, Union[float, None])], Type[Absent]],
            rm_projs: Union[Iterable[str], Type[Absent]],
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {'type': 'fighter', 'item_id': item_id}
        conditional_insert(container=body, key='state', value=state)
        conditional_insert(container=body, key='add_projs', value=add_projs)
        conditional_insert(container=body, key='change_projs', value=change_projs)
        conditional_insert(container=body, key='rm_projs', value=rm_projs)
        conditional_insert(container=body, key='effect_modes', value=effect_modes)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='PATCH',
            url=f'{self._base_url}/sol/{sol_id}/item/{item_id}',
            params=params,
            json=body)

    # Charge methods
    def change_charge_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='charge',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            item_info_mode=item_info_mode)

    # Autocharge methods
    def change_autocharge_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='autocharge',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            item_info_mode=item_info_mode)

    # System-wide effect methods
    def add_sw_effect_request(
            self, *,
            sol_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {'type': 'sw_effect', 'type_id': type_id}
        conditional_insert(container=body, key='state', value=state)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def change_sw_effect_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='sw_effect',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            item_info_mode=item_info_mode)

    # Fit-wide effect methods
    def add_fw_effect_request(
            self, *,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
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
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self.__change_simple_item_request(
            cmd_name='fw_effect',
            sol_id=sol_id,
            item_id=item_id,
            state=state,
            item_info_mode=item_info_mode)

    # Projected effect methods
    def add_proj_effect_request(
            self, *,
            sol_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {'type': 'proj_effect', 'type_id': type_id}
        conditional_insert(container=body, key='state', value=state)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def change_proj_effect_request(
            self, *,
            sol_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]],
            add_projs: Union[Iterable[str], Type[Absent]],
            rm_projs: Union[Iterable[str], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {'type': 'proj_effect', 'item_id': item_id}
        conditional_insert(container=body, key='state', value=state)
        conditional_insert(container=body, key='add_projs', value=add_projs)
        conditional_insert(container=body, key='rm_projs', value=rm_projs)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='PATCH',
            url=f'{self._base_url}/sol/{sol_id}/item/{item_id}',
            params=params,
            json=body)

    # Auxiliary methods
    def __add_simple_item_request(
            self, *,
            cmd_name: str,
            sol_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, str, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {
            'type': cmd_name,
            'fit_id': fit_id,
            'type_id': type_id}
        conditional_insert(container=body, key='state', value=state)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='POST',
            url=f'{self._base_url}/sol/{sol_id}/item',
            params=params,
            json=body)

    def __change_simple_item_request(
            self, *,
            cmd_name: str,
            sol_id: str,
            item_id: int,
            state: Union[bool, str, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        body = {'type': cmd_name}
        conditional_insert(container=body, key='state', value=state)
        params = {}
        conditional_insert(container=params, key='item', value=item_info_mode)
        return Request(
            self,
            method='PATCH',
            url=f'{self._base_url}/sol/{sol_id}/item/{item_id}',
            params=params,
            json=body)
