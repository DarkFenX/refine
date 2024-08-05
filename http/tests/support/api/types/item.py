from __future__ import annotations

from collections import namedtuple
from typing import TYPE_CHECKING

from tests.support.consts import ApiItemInfoMode
from tests.support.util import Absent, AttrDict, AttrHookDef
from .mod_info import AttrModInfoMap
from .side_effect_info import SideEffectInfo, SideEffectStrInfo


if TYPE_CHECKING:
    from collections.abc import Iterable
    from typing import Type, Union

    from tests.support.api import ApiClient
    from tests.support.consts import ApiEffMode, ApiState
    from tests.support.request import Request

AttrVals = namedtuple('AttrVals', ('base', 'dogma', 'extra'))
EffectInfo = namedtuple('EffectInfo', ('running', 'mode'))


class Item(AttrDict):

    def __init__(self, client: ApiClient, data: dict, sol_id: str):
        super().__init__(
            data=data,
            hooks={
                'charge': AttrHookDef(func=lambda charge: Item(client=client, data=charge, sol_id=sol_id)),
                'autocharges': AttrHookDef(
                    func=lambda acs: {int(k): Item(client=client, data=v, sol_id=sol_id) for k, v in acs.items()}),
                'side_effects': AttrHookDef(func=lambda ses: {
                    int(k): SideEffectInfo(v[0], v[1], None if v[2] is None else SideEffectStrInfo(*v[2]))
                    for k, v in ses.items()}),
                'attrs': AttrHookDef(func=lambda attrs: {int(k): AttrVals(*v) for k, v in attrs.items()}),
                'effects': AttrHookDef(func=lambda effects: {int(k): EffectInfo(*v) for k, v in effects.items()}),
                'mods': AttrHookDef(func=AttrModInfoMap)})
        self._client = client
        self._sol_id = sol_id

    def update_request(
            self,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.get_item_request(sol_id=self._sol_id, item_id=self.id, item_info_mode=item_info_mode)

    def update(
            self,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.full,
            status_code: int = 200,
    ) -> Union[Item, None]:
        resp = self.update_request(item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def remove_request(self) -> Request:
        return self._client.remove_item_request(sol_id=self._sol_id, item_id=self.id)

    def remove(self, status_code: int = 204, json_predicate: Union[dict, None] = None) -> None:
        resp = self.remove_request().send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code, json_predicate=json_predicate)

    # Character methods
    def change_char_request(
            self,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.change_char_request(
            sol_id=self._sol_id,
            item_id=self.id,
            state=state,
            item_info_mode=item_info_mode)

    def change_char(
            self,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Union[Item, None]:
        resp = self.change_char_request(state=state, item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    # Skill methods
    def change_skill_request(
            self,
            level: Union[int, Type[Absent]],
            state: Union[bool, Type[Absent]],
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.change_skill_request(
            sol_id=self._sol_id,
            item_id=self.id,
            level=level,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode)

    def change_skill(
            self,
            level: Union[int, Type[Absent]] = Absent,
            state: Union[bool, Type[Absent]] = Absent,
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Union[Item, None]:
        resp = self.change_skill_request(
            level=level,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    # Implant methods
    def change_implant_request(
            self,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.change_implant_request(
            sol_id=self._sol_id,
            item_id=self.id,
            state=state,
            item_info_mode=item_info_mode)

    def change_implant(
            self,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Union[Item, None]:
        resp = self.change_implant_request(state=state, item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    # Booster methods
    def change_booster_request(
            self,
            state: Union[bool, Type[Absent]],
            side_effects: Union[dict[int, bool], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.change_booster_request(
            sol_id=self._sol_id,
            item_id=self.id,
            state=state,
            side_effects=side_effects,
            item_info_mode=item_info_mode)

    def change_booster(
            self,
            state: Union[bool, Type[Absent]] = Absent,
            side_effects: Union[dict[int, bool], Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
            json_predicate: Union[dict, None] = None,
    ) -> Union[Item, None]:
        resp = self.change_booster_request(state=state, side_effects=side_effects, item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    # Ship methods
    def change_ship_request(
            self,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.change_ship_request(
            sol_id=self._sol_id,
            item_id=self.id,
            state=state,
            item_info_mode=item_info_mode)

    def change_ship(
            self,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Union[Item, None]:
        resp = self.change_ship_request(state=state, item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    # Module methods
    def change_mod_request(
            self,
            state: Union[ApiState, Type[Absent]],
            charge: Union[int, None, Type[Absent]],
            add_projs: Union[Iterable[(str, Union[float, None])], Type[Absent]],
            change_projs: Union[Iterable[(str, Union[float, None])], Type[Absent]],
            rm_projs: Union[Iterable[str], Type[Absent]],
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.change_mod_request(
            sol_id=self._sol_id,
            item_id=self.id,
            state=state,
            charge=charge,
            add_projs=add_projs,
            change_projs=change_projs,
            rm_projs=rm_projs,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode)

    def change_mod(
            self,
            state: Union[ApiState, Type[Absent]] = Absent,
            charge: Union[int, Type[Absent]] = Absent,
            add_projs: Union[Iterable[(str, Union[float, None])], Type[Absent]] = Absent,
            change_projs: Union[Iterable[(str, Union[float, None])], Type[Absent]] = Absent,
            rm_projs: Union[Iterable[str], Type[Absent]] = Absent,
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Union[Item, None]:
        resp = self.change_mod_request(
            state=state,
            charge=charge,
            add_projs=add_projs,
            change_projs=change_projs,
            rm_projs=rm_projs,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    # Rig methods
    def change_rig_request(
            self,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.change_rig_request(
            sol_id=self._sol_id,
            item_id=self.id,
            state=state,
            item_info_mode=item_info_mode)

    def change_rig(
            self,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Union[Item, None]:
        resp = self.change_rig_request(state=state, item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    # System-wide effect methods
    def change_sw_effect_request(
            self,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.change_sw_effect_request(
            sol_id=self._sol_id,
            item_id=self.id,
            state=state,
            item_info_mode=item_info_mode)

    def change_sw_effect(
            self,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Union[Item, None]:
        resp = self.change_sw_effect_request(state=state, item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    # Fit-wide effect methods
    def change_fw_effect_request(
            self,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.change_fw_effect_request(
            sol_id=self._sol_id,
            item_id=self.id,
            state=state,
            item_info_mode=item_info_mode)

    def change_fw_effect(
            self,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Union[Item, None]:
        resp = self.change_fw_effect_request(state=state, item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    # Projected effect methods
    def change_proj_effect_request(
            self,
            state: Union[bool, Type[Absent]],
            add_projs: Union[Iterable[str], Type[Absent]],
            rm_projs: Union[Iterable[str], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.change_proj_effect_request(
            sol_id=self._sol_id,
            item_id=self.id,
            state=state,
            add_projs=add_projs,
            rm_projs=rm_projs,
            item_info_mode=item_info_mode)

    def change_proj_effect(
            self,
            state: Union[bool, Type[Absent]] = Absent,
            add_projs: Union[Iterable[str], Type[Absent]] = Absent,
            rm_projs: Union[Iterable[str], Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Union[Item, None]:
        resp = self.change_proj_effect_request(
            state=state,
            add_projs=add_projs,
            rm_projs=rm_projs,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None
